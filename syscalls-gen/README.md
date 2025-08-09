## syscalls-gen

Generates per-architecture, per-kernel-version syscall tables for this project from the upstream Linux source on GitHub. This is a forked generator tailored for rawsys-linux and its per-arch, explicit-version layout.

### What it does
- Pulls syscall definitions from the Linux source tree (either `syscall.tbl` files or `unistd.h` headers, depending on the arch).
- Generates Rust enums under `../src/arch/<arch>/vX_Y.rs` (e.g., `../src/arch/x86_64/v6_10.rs`).
- Generates `../src/errno/generated.rs` from the same Linux version’s errno headers.
- **Does not modify any `mod.rs` files;** you choose which version to expose.

### Requirements
- Rust toolchain (stable is fine).
- Network access to `https://raw.githubusercontent.com/torvalds/linux`.

### CLI
Run from this directory:

- Single version (defaults to latest configured):
  - `cargo run`
  - or explicitly: `cargo run -- --version v6.10`

- Multiple versions and/or arch filters:
  - `cargo run -- --versions v6.8,v6.10`
  - `cargo run -- --versions v6.8,v6.10 --archs x86_64,aarch64`
  - Repeating flags also works: `cargo run -- --version v6.8 --version v6.10 --arch x86_64 --arch aarch64`

Supported flags:
- `--version <vX.Y>`: A single Linux tag (repeatable)
- `--versions <vX.Y,vA.B,...>`: Comma-separated Linux tags
- `--arch <name>`: Filter to a single architecture (repeatable)
- `--archs <a,b,c>`: Comma-separated list of architectures

If no `--version/--versions` is provided, the generator uses the built-in default (currently `v6.10`). If no arch filter is provided, all supported arches are generated.

### Output layout
- Syscalls per arch and version:
  - `../src/arch/<arch>/vX_Y.rs`
- Errno definitions (matching version):
  - `../src/errno/generated.rs`

Example:
- `cargo run -- --version v6.10 --arch x86_64`
  - Writes `../src/arch/x86_64/v6_10.rs`
  - Updates `../src/errno/generated.rs`

### Selecting a version in your crate
This generator intentionally does not update `mod.rs`. Choose the version you want to expose manually. Example for `../src/arch/x86_64/mod.rs`:

```rust
//! x86_64 architecture syscall definitions.

pub mod v6_8;
pub mod v6_10;

// Select one explicitly:
pub use v6_10::*;
```

### Supported architectures
The generator currently covers architectures present in `SOURCES` (see `src/main.rs`). Typical list:
- `x86`, `x86_64`, `arm`, `aarch64`, `sparc`, `sparc64`, `powerpc`, `powerpc64`, `mips`, `mips64`, `s390x`, `riscv32`, `riscv64`, `loongarch64`.

Notes:
- Some arches are parsed from tables (e.g., `syscall.tbl`); others from headers (e.g., `unistd.h` for aarch64/riscv/loongarch).
- Certain compatibility-only syscalls may be blocklisted per-arch (e.g., `sync_file_range2` on riscv/loongarch) to match upstream definitions.

### Tips and warnings
- Prefer stable Linux versions. You can find the stable Linux versions in [https://www.kernel.org/](https://www.kernel.org/).
- GitHub rate limits may apply if running frequently.
- Generated files are auto-formatted to the project’s style on build; no manual edits needed.
- It is recommended to run from oldest to newest versions in case there are changes to errno.

### Differences from original `syscalls`
- Supports multiple kernel versions in one invocation.
- Writes per-arch, per-version modules (`vX_Y.rs`) without overwriting prior versions.
- Does not auto-update `mod.rs`; selection is explicit and manual.

### License
This directory’s generator is adapted from the original `syscalls` project and retains its license. See the repository’s `LICENSE`.

