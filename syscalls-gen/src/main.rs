#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::upper_case_acronyms)]

use crate::tables::Source;
use color_eyre::eyre::{eyre, Result, WrapErr};
use futures::future::try_join_all;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tables::{Header, Table};

mod errors;
mod tables;

/// URL of the Linux repository to pull the syscall tables from.
static LINUX_REPO: &str = "https://raw.githubusercontent.com/torvalds/linux";

/// Default Linux version to pull the syscall tables from.
/// Multiple versions can be specified via CLI flags.
static DEFAULT_LINUX_VERSION: &str = "v6.10";

lazy_static! {
    /// List of syscall tables for each architecture.
    static ref SOURCES: Vec<Source<'static>> = vec![
        Source::Table(Table {
            arch: "x86",
            path: "arch/x86/entry/syscalls/syscall_32.tbl",
            abi: &[ABI::I386],
        }),
        Source::Table(Table {
            arch: "x86_64",
            path: "arch/x86/entry/syscalls/syscall_64.tbl",
            abi: &[ABI::COMMON, ABI::B64],
        }),
        Source::Table(Table {
            arch: "arm",
            path: "arch/arm/tools/syscall.tbl",
            abi: &[ABI::COMMON],
        }),
        // NOTE: arm64/aarch64 is a little different from all the other tables.
        // These are defined in `unistd.h`, which is supposed to be the method
        // used for all new architectures going forward.
        Source::Header(Header {
            arch: "aarch64",
            headers: &[
                "include/uapi/asm-generic/unistd.h",
                //"arch/arm64/include/asm/unistd.h",
            ],
            blocklist: &[
                // NOTE: On aarch64 platforms, `sync_file_range2` only provides
                // compatibility for aarch32.
                "sync_file_range2",
            ],
        }),
        Source::Table(Table {
            arch: "sparc",
            path: "arch/sparc/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::B32],
        }),
        Source::Table(Table {
            arch: "sparc64",
            path: "arch/sparc/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::B64],
        }),
        Source::Table(Table {
            arch: "powerpc",
            path: "arch/powerpc/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::NOSPU, ABI::B32],
        }),
        Source::Table(Table {
            arch: "powerpc64",
            path: "arch/powerpc/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::NOSPU, ABI::B64],
        }),
        Source::Table(Table {
            arch: "mips",
            path: "arch/mips/kernel/syscalls/syscall_o32.tbl",
            abi: &[ABI::O32],
        }),
        Source::Table(Table {
            arch: "mips64",
            path: "arch/mips/kernel/syscalls/syscall_n64.tbl",
            abi: &[ABI::N64],
        }),
        Source::Table(Table {
            arch: "s390x",
            path: "arch/s390/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::B64],
        }),
        Source::Header(Header {
            arch: "riscv32",
            headers: &[
                "include/uapi/asm-generic/unistd.h",
                "arch/riscv/include/uapi/asm/unistd.h",
            ],
            blocklist: &[
                // It doesn't have defines `__NR_sync_file_range2` or
                // `__ARCH_WANT_SYNC_FILE_RANGE2` in
                // `arch/riscv/include/uapi/asm/unistd.h` header file
                "sync_file_range2",
            ],
        }),
        Source::Header(Header {
            arch: "riscv64",
            headers: &[
                "include/uapi/asm-generic/unistd.h",
                "arch/riscv/include/uapi/asm/unistd.h",
            ],
            blocklist: &[
                // For riscv64, see riscv32's explanation.
                "sync_file_range2",
            ],
        }),
        Source::Header(Header {
            arch: "loongarch64",
            headers: &[
                "include/uapi/asm-generic/unistd.h",
                "arch/loongarch/include/uapi/asm/unistd.h",
            ],
            blocklist: &[
                // For loongarch64, see riscv32's explanation.
                "sync_file_range2",
            ],
        }),
    ];
}

pub struct ABI<'a> {
    name: &'a str,
    offset: u32,
}

impl<'a> ABI<'a> {
    // Different syscall ABIs have different offsets. This currently only
    // applies to MIPS and ia64. (Search for `__NR_Linux` in the kernel source
    // to find syscall offsets.)
    pub const COMMON: Self = Self::new("common", 0);
    pub const I386: Self = Self::new("i386", 0);
    pub const NOSPU: Self = Self::new("nospu", 0);
    pub const B32: Self = Self::new("32", 0);
    pub const B64: Self = Self::new("64", 0);
    pub const O32: Self = Self::new("o32", 4000);
    pub const N64: Self = Self::new("n64", 5000);

    #[must_use]
    pub const fn new(name: &'a str, offset: u32) -> Self {
        Self { name, offset }
    }
}

/// Fetches a file path from the repository.
async fn fetch_path(path: &str, version: &str) -> Result<String> {
    let url = format!("{LINUX_REPO}/{version}/{path}");

    println!("Fetching {url}");
    let contents = reqwest::get(&url)
        .await
        .wrap_err_with(|| eyre!("Failed to fetch URL '{url}'"))?
        .text()
        .await
        .wrap_err_with(|| eyre!("Failed to parse contents of URL '{url}'"))?;

    Ok(contents)
}

fn parse_args() -> (Vec<String>, Option<HashSet<String>>) {
    // Simple CLI parser to avoid extra dependencies.
    // Supported flags:
    //   --versions v6.8,v6.10   (comma-separated)
    //   --version v6.10         (repeatable)
    //   --archs x86_64,aarch64  (comma-separated)
    //   --arch x86_64           (repeatable)
    let mut versions: Vec<String> = Vec::new();
    let mut archs: HashSet<String> = HashSet::new();

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--versions" => {
                if let Some(v) = args.next() {
                    for s in v.split(',') {
                        let s = s.trim();
                        if !s.is_empty() {
                            versions.push(s.to_string());
                        }
                    }
                }
            }
            "--version" => {
                if let Some(v) = args.next() {
                    versions.push(v);
                }
            }
            "--archs" => {
                if let Some(v) = args.next() {
                    for s in v.split(',') {
                        let s = s.trim();
                        if !s.is_empty() {
                            archs.insert(s.to_string());
                        }
                    }
                }
            }
            "--arch" => {
                if let Some(v) = args.next() {
                    archs.insert(v);
                }
            }
            _ => {}
        }
    }

    if versions.is_empty() {
        versions.push(DEFAULT_LINUX_VERSION.to_string());
    }

    let archs = if archs.is_empty() { None } else { Some(archs) };
    (versions, archs)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let base_dir = Path::new("..");

    let (versions, arch_filter) = parse_args();

    for version in &versions {
        let mut futures: Vec<Pin<Box<dyn Future<Output = Result<()>>>>> =
            Vec::new();

        for source in SOURCES.iter() {
            if let Some(filter) = &arch_filter {
                if !filter.contains(source.arch()) {
                    continue;
                }
            }
            futures.push(Box::pin(source.generate(base_dir, version)));
        }

        let errno = base_dir.join("src/errno/generated.rs");
        futures.push(Box::pin(errors::generate_errno(errno, version.clone())));

        try_join_all(futures).await?;
    }

    Ok(())
}
