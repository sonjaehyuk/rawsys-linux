# rawsys-linux

[![Rust 2024](https://img.shields.io/badge/rust-2024-edition?style=for-the-badge&logo=rust)](https://doc.rust-lang.org/edition-guide/)
![License](https://img.shields.io/badge/license-BSD--2--Clause-blue?style=for-the-badge)

[README.ko.md](README.ko.md)

**Call the Linux kernel directly without libc.**

> This is a low-level library to enumerate and invoke raw Linux system calls.
> Key changes are summarized in [CHANGELOG.md](CHANGELOG.md). Highlights in v1.0.0:
* Raw call number types are now fixed-width to the architecture (`u32`/`u64`) and exposed via the `SyscallWord` alias.
* Migrated to Rust 2024 edition and reduced compiler warnings.
* Kernel version tables can be selected at build time via features.

---

* [Introduction](#introduction)
* [Installation](#installation)
* [Quick Start](#quick-start)
* [Features & Configuration](#features--configuration)
  * [no_std](#no_std)
  * [Kernel Version Selection (mutually exclusive)](#kernel-version-selection-mutually-exclusive)
  * [Serde](#serde)
* [Build Examples](#build-examples)
* [Architecture Support](#architecture-support)
* [Updating the syscall list](#updating-the-syscall-list)
* [About contributing](#about-contributing)
* [License & Acknowledgements](#license--acknowledgements)

## Introduction

`rawsys-linux` exposes:
- A `Sysno` enum per architecture with syscall IDs and names.
- Inlinable raw syscall functions for several architectures.
- `Errno`, `SysnoSet`, and `SysnoMap` utilities for ergonomic and fast lookup.

## Installation

Using a Git dependency:

```toml
[dependencies]
rawsys-linux = { git = "https://github.com/sonjaehyuk/rawsys-linux" }
```

In code, `rawsys-linux` is imported as `rawsys_linux` (hyphen → underscore):

```rust
use rawsys_linux::{syscall, Sysno, SyscallWord};
```

## Quick Start

Unsafe by nature: invoking syscalls bypasses libc. Use with care.

```rust
use rawsys_linux::{syscall, Sysno, SyscallWord};

fn main() {
  unsafe {
    // openat(AT_FDCWD, "/dev/null", O_RDONLY, 0)
    let fd = syscall(Sysno::openat, &rawsys_linux::SyscallArgs::from(&[
      (-100isize) as SyscallWord,
      "/dev/null\0".as_ptr() as SyscallWord,
      0,
      0,
    ]))?;

    // close(fd)
    let _ = syscall(Sysno::close, &rawsys_linux::SyscallArgs::from(&[fd]))?;
  }
}
```

Macro form is also available:

```rust
use rawsys_linux::syscall;

fn main() {
  unsafe {
    let tid = syscall!(Sysno::gettid)?;
    let _ = tid;
  } 
}
```

## Features & Configuration

Default features: `std`, `serde`.

Additional features:
- `full`: Enables all optional features.
- `all`: Exposes syscall tables for all architectures.
- Per-architecture: `aarch64`, `arm`, `loongarch64`, `mips`, `mips64`, `powerpc`, `powerpc64`, `riscv32`, `riscv64`, `s390x`, `sparc`, `sparc64`, `x86`, `x86_64`.
- Kernel versions: `default_kernel_5_4`, `default_kernel_5_10`, `default_kernel_5_15`, `default_kernel_6_1`, `default_kernel_6_6`, `default_kernel_6_10`, `default_kernel_6_12`.

**kernel versions selects the kernel version exposed by default, not the available kernels. All versions are available via modules.**

### no_std

Disable default features and choose exactly one kernel version:

```toml
[dependencies]
rawsys-linux = { git = "https://github.com/sonjaehyuk/rawsys-linux", default-features = false, features = ["default_kernel_6_12", "serde"] }
```

Notes:
- Syscall invocation works without `std`. Error handling (`Errno`) and internals are `no_std`-friendly.
- Some architectures require nightly due to inline assembly constraints (see Architecture Support).

### Kernel Version Selection (mutually exclusive)

Pick exactly one kernel version to lock syscall tables. If none is chosen, the latest (`default_kernel_6_12`) is used by default.

```bash
cargo build --features "default_kernel_5_10"
cargo build --features "default_kernel_6_6 aarch64"
```

Enabling multiple `default_kernel_*` features simultaneously will cause duplicate symbol errors. Choose exactly one.

### Serde

`Sysno`, `SysnoSet`, and related types implement Serde. To disable Serde, turn off default features and opt-in only what you need.

```toml
[dependencies]
rawsys-linux = { git = "https://github.com/sonjaehyuk/rawsys-linux", default-features = false, features = ["default_kernel_6_12"] }
```

## Build Examples

```bash
# x86-64 target with kernel 6.1 tables
cargo build --features "default_kernel_6_1"

# Cross-compile: expose aarch64 tables + kernel 5.15
cargo build --features "aarch64 default_kernel_5_15"

# All architectures + kernel 6.12
cargo build --features "all default_kernel_6_12"
```

## Architecture Support

The *Enum* column means that a `Sysno` enum is implemented for this
architecture.

The *Invoke* column means that syscalls can be invoked for this architecture.

The *Stable Rust?* column means that syscall invocation only requires stable Rust. Some architectures require nightly Rust because inline assembly [is not yet stabilized for all architectures][asm_experimental_arch].

[asm_experimental_arch]: https://github.com/rust-lang/rust/issues/93335

|    Arch     | Enum | Invoke | Stable Rust? |
|:-----------:|:----:|:------:|:------------:|
|   `arm`\*   |  ✅   |   ✅    |    Yes ✅     |
|  `aarch64`  |  ✅   |   ✅    |    Yes ✅     |
|   `mips`    |  ✅   |   ✅    |     No ❌     |
|  `mips64`   |  ✅   |   ✅    |     No ❌     |
|  `powerpc`  |  ✅   |   ✅    |     No ❌     |
| `powerpc64` |  ✅   |   ✅    |     No ❌     |
|  `riscv32`  |  ✅   |   ❌†   |     No ❌     |
|  `riscv64`  |  ✅   |   ✅    |    Yes ✅     |
|   `s390x`   |  ✅   |   ✅    |     No ❌     |
|   `sparc`   |  ✅   |   ❌    |     N/A      |
|  `sparc64`  |  ✅   |   ❌    |     N/A      |
|    `x86`    |  ✅   |   ✅    |    Yes ✅     |
|  `x86_64`   |  ✅   |   ✅    |    Yes ✅     |

\* Includes ARM thumb mode support.

† Rust does not support riscv32 Linux targets, but syscall functions are
implemented if you're feeling adventurous.

## Updating the syscall list

Updates are pulled from the Linux source tree (tables or unistd headers).

- Generate syscalls for a specific version:
  - `cd syscalls-gen && cargo run -- --version v6.10`
- Generate multiple versions and/or specific archs:
  - `cd syscalls-gen && cargo run -- --versions v6.8,v6.10 --archs x86_64,aarch64`

This will add files under `src/arch/<arch>/vX_Y.rs` (e.g., `src/arch/x86_64/v6_10.rs`).
When adding a new version, declare the module in each architecture’s `mod.rs` and update the selection logic as needed.

## About contributing

Check out [CONTRIBUTING.md](CONTRIBUTING.md)

## License & Acknowledgements

- License: [BSD-2-Clause](LICENSE)
- Acknowledgements: Thanks to Jason White’s original `syscalls` project.
