# rawsys-linux (한국어)

[![Rust 2024](https://img.shields.io/badge/rust-2024-edition?style=for-the-badge&logo=rust)](https://doc.rust-lang.org/edition-guide/)
![License](https://img.shields.io/badge/license-BSD--2--Clause-blue?style=for-the-badge)

**libc 없이 리눅스 커널을 직접 호출하세요.**

> Linux 시스템 호출 목록을 열거하고 호출하는 저수준 라이브러리입니다.
> 주요 변경점은 [CHANGELOG.md](CHANGELOG.md)를 참고하세요. v1.0.0 주요 변경점:
* raw 호출 번호 타입을 아키텍처 고정 폭(`u32`/`u64`)으로 정리하고 `SyscallWord` 별칭 제공
* Rust 2024 edition으로 전환, 경고 최소화
* 기능 플래그로 커널 버전 테이블 선택 가능

---

* [소개](#소개)
* [설치](#설치)
* [사용 예시](#사용-예시)
* [기능 및 구성](#기능-및-구성)
  * [no_std](#no_std)
  * [커널 버전 선택(서로 배타적)](#커널-버전-선택서로-배타적)
  * [Serde](#serde)
* [빌드 예시](#빌드-예시)
* [지원 아키텍처](#지원-아키텍처)
* [syscall 목록 추가](#syscall-목록-추가)
* [기여](#기여)
* [라이선스와 감사의 말](#라이선스와-감사의-말)

## 소개

`rawsys-linux`는 다음을 제공합니다:
- 아키텍처별 `Sysno` 열거형(번호/이름)
- 여러 아키텍처의 인라인 가능한 raw syscall 함수
- `Errno`, `SysnoSet`, `SysnoMap` 등 빠르고 실용적인 유틸리티

## 설치

Git 의존성으로 사용할 수 있습니다:

```toml
[dependencies]
rawsys-linux = { git = "https://github.com/sonjaehyuk/rawsys-linux" }
```

코드에서는 `rawsys-linux`가 `rawsys_linux`로 임포트됩니다(하이픈→언더스코어):

```rust
use rawsys_linux::{syscall, Sysno, SyscallWord};
```

## 사용 예시

시스템 호출은 본질적으로 unsafe입니다. 주의해서 사용하세요.

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

매크로로도 호출할 수 있습니다:

```rust
use rawsys_linux::syscall;

fn main() {
  unsafe {
    let tid = syscall!(Sysno::gettid)?;
    let _ = tid;
  } 
}
```

## 기능 및 구성

기본 기능: `std`, `serde`.

추가 기능:
- `full`: 모든 부가 기능 활성화
- `all`: 모든 아키텍처의 테이블 노출
- 개별 아키텍처: `aarch64`, `arm`, `loongarch64`, `mips`, `mips64`, `powerpc`, `powerpc64`, `riscv32`, `riscv64`, `s390x`, `sparc`, `sparc64`, `x86`, `x86_64`
- 커널 버전: `default_kernel_5_4`, `default_kernel_5_10`, `default_kernel_5_15`, `default_kernel_6_1`, `default_kernel_6_6`, `default_kernel_6_10`, `default_kernel_6_12`

**커널 버전은 기본으로 노출한 커널 버전을 선택하는 것이지, 사용 가능한 커널을 선택하는 것이 아닙니다. 모든 버전은 모듈을 통해 사용 가능합니다.**

### no_std

기본 기능을 끄고 정확히 하나의 커널 버전을 선택하세요:

```toml
[dependencies]
rawsys-linux = { git = "https://github.com/sonjaehyuk/rawsys-linux", default-features = false, features = ["default_kernel_6_12", "serde"] }
```

참고:
- `std` 없이도 호출 가능하며, `Errno` 등 내부 유틸은 `no_std` 호환입니다.
- 일부 아키텍처는 인라인 어셈 제약으로 nightly가 필요할 수 있습니다(아래 지원 아키텍처 참고).

### 커널 버전 선택(서로 배타적)

커널 버전은 정확히 하나만 선택하세요. 선택하지 않으면 최신(`default_kernel_6_12`)이 기본값입니다.

```bash
cargo build --features "default_kernel_5_10"
cargo build --features "default_kernel_6_6 aarch64"
```

여러 `default_kernel_*` 기능을 동시에 켜면 중복 심볼로 컴파일이 실패합니다.

### Serde

`Sysno`, `SysnoSet` 등은 Serde를 지원합니다. 비활성화하려면 기본 기능을 끄고 필요한 기능만 선택하세요.

```toml
[dependencies]
rawsys-linux = { git = "https://github.com/sonjaehyuk/rawsys-linux", default-features = false, features = ["default_kernel_6_12"] }
```

## 빌드 예시

```bash
# x86-64 대상, 커널 6.1 고정
cargo build --features "default_kernel_6_1"

# aarch64 테이블 노출 + 커널 5.15
cargo build --features "aarch64 default_kernel_5_15"

# 모든 아키텍처 + 커널 6.12
cargo build --features "all default_kernel_6_12"
```

## 지원 아키텍처

*Enum* 열은 해당 아키텍처에 `Sysno` enum이 구현되어 있다는 것을 의미합니다.


*Invoke* 열은 해당 아키텍처에 시스템 호출이 가능하다는 것을 의미합니다.

*Stable Rust?* 열은 stable rust만으로 해당 아키텍처에서 시스템 호출을 하는 것이 가능한지 나타냅니다. [몇몇 아키텍처는 인라인 어셈블리를 쓰기 위해 nightly rust가 필요합니다.][asm_experimental_arch]  

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

\* ARM thumb mode 지원 포함.

† Rust는 riscv32 Linux를 지원하지 않지만, 구현은 되어 있기에 모험심이 있다면 사용해 보실 수 있습니다.

## syscall 목록 추가

업데이트는 Linux 소스 트리(테이블이나 unistd 헤더)에서 가져옵니다.

- 특정 버전에 대한 시스템 호출을 생성합니다.
  - `cd syscalls-gen && cargo run -- --version v6.10`
- 여러 버전 및/또는 특정 아키텍처 전용으로 생성:
  - `cd syscalls-gen && cargo run -- --versions v6.8,v6.10 --archs x86_64,aarch64`

이렇게 하면 `src/arch/<arch>/vX_Y.rs` 형식의 파일이 추가됩니다.(e.g., `src/arch/x86_64/v6_10.rs`).
새 버전을 추가했다면 각 아키텍처의 `mod.rs`에 모듈을 선언하고 선택 로직을 갱신하세요.

## 기여

[CONTRIBUTING.md](CONTRIBUTING.md)를 확인하세요.

## 라이선스와 감사의 말

- 라이선스: [BSD-2-Clause](LICENSE)
- 원저작물: Jason White의 `syscalls` 프로젝트에 감사드립니다.

