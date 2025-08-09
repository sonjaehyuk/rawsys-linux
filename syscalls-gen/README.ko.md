## syscalls-gen

이 도구는 업스트림 Linux 소스(GitHub)에서 아키텍처별 시스템 호출 정의를 가져와, 이 프로젝트의 구조(ISA별 + 커널 버전 명시)에 맞춘 Rust 코드로 생성합니다. rawsys-linux 사용 목적에 맞게 포크되어, 단일 커널 버전이 아닌 **아키텍처별/버전별 파일**을 생성합니다.

### 주요 기능
- Linux 소스 트리에서 시스템 호출 정의를 가져옴 (`syscall.tbl` 또는 `unistd.h`, 아키텍처별 상이)
- `../src/arch/<arch>/vX_Y.rs` 형태로 Rust 코드 생성 (예: `../src/arch/x86_64/v6_10.rs`)
- 동일한 Linux 버전의 errno 헤더를 파싱하여 `../src/errno/generated.rs` 생성
- `mod.rs`는 자동 **수정하지 않음** — 사용자가 노출할 버전을 직접 선택

### 요구 사항
- Rust 툴체인 (stable 권장)
- `https://raw.githubusercontent.com/torvalds/linux` 네트워크 접근

### 사용법 (이 디렉터리에서 실행)

- 단일 버전 (기본값 사용 또는 명시):
  - `cargo run`
  - `cargo run -- --version v6.10`

- 다중 버전/아키텍처 필터:
  - `cargo run -- --versions v6.8,v6.10`
  - `cargo run -- --versions v6.8,v6.10 --archs x86_64,aarch64`
  - 반복 플래그도 가능: `cargo run -- --version v6.8 --version v6.10 --arch x86_64 --arch aarch64`

지원 플래그:
- `--version <vX.Y>`: 단일 Linux 태그(반복 가능)
- `--versions <vX.Y,vA.B,...>`: 콤마 구분 다중 태그
- `--arch <name>`: 단일 아키텍처 필터(반복 가능)
- `--archs <a,b,c>`: 콤마 구분 다중 아키텍처

버전 지시자를 주지 않으면 내장 기본값(현재 `v6.10`)을 사용합니다. 아키텍처 필터가 없으면 지원되는 모든 아키텍처를 생성합니다.

### 생성 결과 경로
- 아키텍처/버전별 시스템 호출:
  - `../src/arch/<arch>/vX_Y.rs`
- 동일 버전의 errno 정의:
  - `../src/errno/generated.rs`

예시:
- `cargo run -- --version v6.10 --arch x86_64`
  - `../src/arch/x86_64/v6_10.rs` 생성
  - `../src/errno/generated.rs` 업데이트

### crate에서 버전 선택
이 생성기는 `mod.rs`를 자동 변경하지 않습니다. 노출할 버전을 직접 선택하세요. 예: `../src/arch/x86_64/mod.rs`

```rust
//! x86_64 architecture syscall definitions.

pub mod v6_8;
pub mod v6_10;

// 원하는 버전을 선택해 노출
pub use v6_10::*;
```

### 지원 아키텍처
`src/main.rs`의 `SOURCES`에 정의된 아키텍처를 지원합니다. 일반적으로 다음을 포함합니다:
- `x86`, `x86_64`, `arm`, `aarch64`, `sparc`, `sparc64`, `powerpc`, `powerpc64`, `mips`, `mips64`, `s390x`, `riscv32`, `riscv64`, `loongarch64`

참고:
- 일부 아키텍처는 테이블(`syscall.tbl`)에서, 일부는 헤더(`unistd.h`)에서 파싱합니다.
- RISC-V/LoongArch 등에서는 호환성용 일부 정의(예: `sync_file_range2`)가 제외될 수 있습니다.

### 팁 및 주의사항
- 가능하면 안정 버전을 사용하세요. 안정 버전은 [https://www.kernel.org/](https://www.kernel.org/)에서 찾을 수 있습니다.
- 빈번한 실행 시 GitHub rate limit에 유의하세요.
- 생성 파일은 자동 생성물이므로 직접 수정하지 않는 것을 권장합니다.
- errno에 변경사항이 있을 것을 대비하여 오래된 버전부터 최신 버전 순으로 실행하는 것을 권장합니다.

### 원본 `syscalls` crate 대비 차이점
- 한 번의 실행으로 다중 커널 버전 지원
- 아키텍처/버전별(`vX_Y.rs`) 파일로 누적 생성 (기존 버전 덮어쓰지 않음)
- `mod.rs` 자동 업데이트 없음 — 사용자가 명시적으로 선택

### 라이선스
이 생성기는 원본 `syscalls` 프로젝트를 기반으로 하며, 해당 라이선스를 따릅니다. 저장소의 `LICENSE`를 참고하세요.

