use crate::{ABI, fetch_path};
use color_eyre::eyre::{Result, WrapErr, bail, eyre};
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::fmt;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

pub struct Table<'a> {
    pub arch: &'a str,
    pub path: &'a str,
    pub abi: &'a [ABI<'a>],
}

pub struct Header<'a> {
    pub arch: &'a str,
    pub headers: &'a [&'a str],
    pub blocklist: &'a [&'a str],
}

pub enum Source<'a> {
    /// The definitions are in a `syscall.tbl` file.
    Table(Table<'a>),
    /// The definitions are in a unistd.h header file.
    Header(Header<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableEntry {
    pub id: u32,
    pub name: String,
    pub entry_point: Option<String>,
}

impl TableEntry {
    fn ident(&self) -> Cow<str> {
        // Produce a Rust identifier without using raw id syntax (r#...).
        // 1) Replace any non [A-Za-z0-9_] with '_'.
        // 2) If it starts with a digit, prefix with '_'.
        // 3) If it matches a Rust reserved keyword, append an underscore.
        let mut out: String = self
            .name
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => c,
                _ => '_',
            })
            .collect();

        if out
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            out.insert(0, '_');
        }

        // Rust reserved keywords (2018 edition + reserved).
        const KEYWORDS: &[&str] = &[
            "as", "break", "const", "continue", "crate", "else", "enum",
            "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
            "match", "mod", "move", "mut", "pub", "ref", "return", "self",
            "Self", "static", "struct", "super", "trait", "true", "type",
            "unsafe", "use", "where", "while", "async", "await", "dyn",
            "abstract", "become", "box", "do", "final", "macro", "override",
            "priv", "try", "typeof", "unsized", "virtual", "yield",
        ];

        if KEYWORDS.contains(&out.as_str()) {
            out.push('_');
        }

        if out == self.name {
            Cow::Borrowed(&self.name)
        } else {
            Cow::Owned(out)
        }
    }
}

impl<'a> Table<'a> {
    async fn fetch_table(&self, version: &str) -> Result<Vec<TableEntry>> {
        let contents = fetch_path(self.path, version).await?;

        let mut table = Vec::new();

        for line in contents.lines() {
            let line = line.trim();

            // Skip over empty lines and comments.
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut fields =
                line.split(char::is_whitespace).filter(|x| !x.is_empty());

            let id: u32 = fields
                .next()
                .ok_or_else(|| eyre!("Missing syscall number (line {line:?})"))?
                .parse()
                .wrap_err_with(|| eyre!("Failed parsing line {line:?}"))?;
            let abi_name = fields.next().ok_or_else(|| {
                eyre!("Missing syscall abi field (line {line:?})")
            })?;
            let name = fields
                .next()
                .ok_or_else(|| {
                    eyre!("Missing syscall name field (line {line:?})")
                })?
                .into();
            let entry_point = fields.next().map(Into::into);

            for abi in self.abi {
                if abi.name == abi_name {
                    table.push(TableEntry {
                        id: id + abi.offset,
                        name,
                        entry_point,
                    });
                    break;
                }
            }
        }

        // The table should already be sorted, but lets make sure.
        table.sort();

        Ok(table)
    }
}

impl<'a> Header<'a> {
    async fn fetch_table(&self, version: &str) -> Result<Vec<TableEntry>> {
        lazy_static! {
            // Pattern for matching the syscall definition.
            static ref RE_SYSCALLNR: Regex = Regex::new(r"^#define\s+__NR(?:3264)?_([a-z0-9_]+)\s+(\d+)").unwrap();
            static ref RE_SYSCALLNR_ARCH: Regex = Regex::new(r"^#define\s+__NR(?:3264)?_([a-z0-9_]+)\s+\(__NR_arch_specific_syscall\s*\+\s*(\d+)\)").unwrap();
        }

        let mut table = Vec::new();
        let mut arch_specific_syscall: Option<u32> = None;

        for header in self.headers {
            let contents = fetch_path(header, version).await?;

            for line in contents.lines() {
                let line = line.trim();

                if let Some(cap) = RE_SYSCALLNR.captures(line) {
                    let name: &str = cap[1].into();
                    let id: u32 = cap[2].parse()?;

                    if name == "syscalls" {
                        // This just keeps track of the number of syscalls in
                        // the table and isn't a real syscall.
                        continue;
                    }

                    if name == "arch_specific_syscall" {
                        // This is a placeholder for a block of 16 syscalls
                        // that are reserved for future use.
                        if arch_specific_syscall.is_none() {
                            arch_specific_syscall = Some(id);
                        } else {
                            bail!(
                                "__NR_arch_specific_syscall is defined multiple times"
                            )
                        }
                        continue;
                    }

                    if self.blocklist.contains(&name) {
                        continue;
                    }

                    table.push(TableEntry {
                        id,
                        name: name.into(),
                        entry_point: Some(format!("sys_{name}")),
                    });
                } else if let Some(cap) = RE_SYSCALLNR_ARCH.captures(line) {
                    if let Some(offset) = arch_specific_syscall {
                        let name: &str = cap[1].into();
                        let id: u32 = cap[2].parse()?;

                        if self.blocklist.contains(&name) {
                            continue;
                        }

                        table.push(TableEntry {
                            id: id + offset,
                            name: name.into(),
                            entry_point: Some(format!("sys_{name}")),
                        })
                    } else {
                        bail!(
                            "__NR_arch_specific_syscall definition not found before usage. \
                            Try reordering `Header::headers`?"
                        );
                    }
                }
            }
        }

        // The table should already be sorted, but lets make sure.
        table.sort();

        Ok(table)
    }
}

impl<'a> Source<'a> {
    pub fn arch(&self) -> &'a str {
        match self {
            Self::Table(table) => table.arch,
            Self::Header(header) => header.arch,
        }
    }

    async fn fetch_table(&self, version: &str) -> Result<Vec<TableEntry>> {
        match self {
            Self::Table(table) => table.fetch_table(version).await,
            Self::Header(header) => header.fetch_table(version).await,
        }
    }

    fn version_to_module(version: &str) -> String {
        let v = version.strip_prefix('v').unwrap_or(version);
        format!("v{}", v.replace('.', "_"))
    }

    /// Generates the source file for a specific arch and kernel version.
    pub(crate) async fn generate(
        &self,
        dir: &Path,
        version: &str,
    ) -> Result<()> {
        let arch = self.arch();
        let table = self
            .fetch_table(version)
            .await
            .wrap_err_with(|| eyre!("Failed fetching table for {arch}"))?;

        // Generate `src/arch/{arch}/vX_Y.rs`
        let module = Self::version_to_module(version);
        let arch_dir = dir.join(format!("src/arch/{arch}"));
        create_dir_all(&arch_dir).wrap_err_with(|| {
            eyre!("Failed to create directory {}", arch_dir.display())
        })?;
        let path = arch_dir.join(format!("{module}.rs"));

        let mut file = File::create(&path).wrap_err_with(|| {
            eyre!("Failed to create file {}", path.display())
        })?;
        writeln!(
            file,
            "//! Syscalls for the `{arch}` architecture (Linux {version}).\n"
        )?;
        write!(file, "{}", SyscallFile(&table))?;

        println!(
            "Generated syscalls for {arch} {version} at {}",
            path.display()
        );
        Ok(())
    }
}

struct SyscallFile<'a>(&'a [TableEntry]);

impl<'a> fmt::Display for SyscallFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "// This file is automatically generated. Do not edit!")?;
        writeln!(f)?;

        writeln!(f, "syscall_enum! {{")?;
        writeln!(f, "    pub enum Sysno {{")?;
        for entry in self.0 {
            if entry.entry_point.is_some() {
                writeln!(
                    f,
                    "        /// See [{name}(2)](https://man7.org/linux/man-pages/man2/{name}.2.html) for more info on this syscall.",
                    name = entry.name,
                )?;
                writeln!(
                    f,
                    "        {name} = {id},",
                    name = entry.ident(),
                    id = entry.id
                )?;
            } else {
                // This syscall has no entry point in the kernel, so we could
                // technically exclude this from our list, but that will leave
                // gaps in the syscall table. Our match statements can be better
                // optimized by the compiler if we don't have gaps in the
                // numbering.
                writeln!(
                    f,
                    "        /// NOTE: `{name}` is not implemented in the kernel.",
                    name = entry.name,
                )?;
                writeln!(
                    f,
                    "        {name} = {id},",
                    name = entry.ident(),
                    id = entry.id
                )?;
            }
        }
        writeln!(f, "    }}")?;
        writeln!(f, "    LAST: {};", self.0.last().unwrap().ident())?;
        writeln!(f, "}}")?;

        Ok(())
    }
}
