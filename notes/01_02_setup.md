# Rust 설치 및 프로젝트 생성

---

## 1️⃣ 설치 항목

### rustc (Rust 컴파일러)

```
Rust 프로그램을 컴파일하여 실행 파일로 변환하는 도구
역할: .rs 파일 → 실행 가능한 바이너리
```

### cargo (Rust 패키지 매니저)

```
Rust 프로젝트 관리 도구
역할: 프로젝트 생성, 의존성 관리, 빌드, 실행, 테스트 등
npm (JavaScript), pip (Python)와 유사
```

---

## 2️⃣ 설치 방법 (Linux/macOS)

### 설치 명령어

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**설명:**
- `curl`: 인터넷에서 파일 다운로드
- `--proto '=https'`: HTTPS 프로토콜만 사용
- `--tlsv1.2`: TLS 1.2 이상 사용
- `-sSf`: 조용히 실행, 실패 시 종료
- `sh.rustup.rs`: Rust 설치 스크립트

### 설치 후 경로 활성화

```bash
source $HOME/.cargo/env
```

---

## 3️⃣ 설치 확인 ✅

### rustc 버전 확인

```bash
rustc --version
```

**출력:**
```
rustc 1.96.0 (ac68faa20 2026-05-25)
```

### cargo 버전 확인

```bash
cargo --version
```

**출력:**
```
cargo 1.96.0 (30a34c682 2026-05-25)
```

### 두 버전 동시 확인

```bash
rustc --version && cargo --version
```

---

## 4️⃣ 프로젝트 생성 및 실행

### Step 1️⃣: 프로젝트 디렉토리 이동

```bash
cd ~/Study/rust_study/playground
```

### Step 2️⃣: Cargo로 새 프로젝트 생성

```bash
cargo new hello_world
```

**출력:**
```
Creating binary (application) `hello_world` package
note: see more `Cargo.toml` keys and their definitions at 
https://doc.rust-lang.org/cargo/reference/manifest.html
```

### Step 3️⃣: 프로젝트 폴더 구조

```
hello_world/
├── Cargo.toml          ← 프로젝트 메타데이터 (설정 파일)
├── Cargo.lock          ← 의존성 잠금 파일 (생성 후)
├── src/
│   └── main.rs         ← 메인 프로그램 소스 코드
└── target/             ← 컴파일된 바이너리 (생성 후)
    └── debug/
```

---

## 5️⃣ 주요 파일 설명

### 📄 Cargo.toml (프로젝트 설정)

```toml
[package]
name = "hello_world"           # 프로젝트 이름
version = "0.1.0"              # 버전
edition = "2021"               # Rust 에디션 (2015, 2018, 2021)
authors = ["Your Name <you@example.com>"]

[dependencies]
# 외부 라이브러리 추가
# serde = "1.0"
# tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
# 테스트에만 필요한 라이브러리
```

### 📄 src/main.rs (메인 코드)

```rust
fn main() {
    println!("Hello, world!");
}
```

**설명:**
- `fn main()`: 프로그램 진입점 (필수)
- `println!`: 화면에 출력하는 매크로

---

## 6️⃣ 프로젝트 실행

### 실행 방법 1️⃣: cargo run (컴파일 + 실행)

```bash
cd ~/Study/rust_study/playground/hello_world
cargo run
```

**출력:**
```
Compiling hello_world v0.1.0 (/home/ldg/Study/rust_study/playground/hello_world)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
Running `target/debug/hello_world`

Hello, world!
```

### 실행 방법 2️⃣: 빌드 후 실행

```bash
# 1단계: 컴파일
cargo build

# 2단계: 실행
./target/debug/hello_world
```

### 실행 방법 3️⃣: 최적화된 빌드 (Release)

```bash
# 최적화하여 빌드 (느리지만 실행 빠름)
cargo build --release

# 실행
./target/release/hello_world
```

---

## 7️⃣ 주요 Cargo 명령어

| 명령어 | 설명 | 예시 |
|--------|------|------|
| `cargo new` | 새 프로젝트 생성 | `cargo new my_project` |
| `cargo run` | 컴파일 + 실행 | `cargo run` |
| `cargo build` | 컴파일만 | `cargo build` |
| `cargo build --release` | 최적화 빌드 | `cargo build --release` |
| `cargo check` | 컴파일 가능 여부만 확인 | `cargo check` |
| `cargo test` | 테스트 실행 | `cargo test` |
| `cargo clean` | 빌드 파일 삭제 | `cargo clean` |
| `cargo add` | 라이브러리 추가 | `cargo add serde` |

---

## 8️⃣ Dev vs Release 빌드

| 항목 | Dev (debug) | Release |
|------|-----------|---------|
| **빌드 명령어** | `cargo build` | `cargo build --release` |
| **빌드 시간** | 빠름 | 느림 |
| **실행 속도** | 느림 | 빠름 |
| **파일 크기** | 큼 | 작음 |
| **디버그 정보** | 있음 | 없음 |
| **최적화** | 없음 | 높음 |
| **용도** | 개발 중 | 배포용 |

### 빌드 비교 예제

```bash
# Dev 빌드 (개발 중)
cargo run
# 빠르게 컴파일되고 즉시 실행

# Release 빌드 (배포)
cargo run --release
# 느리게 컴파일되지만 실행이 훨씬 빠름
```

---

## 9️⃣ 프로젝트 구조 상세 분석

```
hello_world/
│
├── Cargo.toml                 # 📋 프로젝트 설정 파일
│   └── 패키지 정보, 의존성 관리
│
├── Cargo.lock                 # 🔒 의존성 버전 잠금 (git에 포함)
│   └── 같은 환경에서 같은 버전 사용 보장
│
├── src/                       # 📁 소스 코드 디렉토리
│   └── main.rs               # 🎯 메인 프로그램
│       └── fn main() { ... }  # 프로그램 시작점
│
├── target/                    # 🎁 컴파일된 파일 (git 제외)
│   ├── debug/                # Dev 빌드
│   │   └── hello_world       # 실행 가능한 바이너리
│   └── release/              # Release 빌드
│       └── hello_world       # 최적화된 바이너리
│
└── .gitignore               # 📝 Git 제외 파일 목록
```

---

## 🔟 실행 흐름

```
1️⃣ cargo run 실행
        ↓
2️⃣ Cargo.toml 읽기
        ↓
3️⃣ 소스 코드 컴파일 (rustc)
        ↓
4️⃣ target/debug/ 에 바이너리 생성
        ↓
5️⃣ 바이너리 실행
        ↓
6️⃣ 결과 출력
```

---

## 1️⃣1️⃣ 자주 하는 실수

### ❌ 실수 1: 프로젝트 폴더 밖에서 실행

```bash
# 🔴 안 됨
cd ~/Study/rust_study
cargo run

# ✅ 올바름
cd ~/Study/rust_study/playground/hello_world
cargo run
```

### ❌ 실수 2: main.rs 수정 후 재빌드 안 함

```bash
# ✅ cargo run이 자동으로 재빌드
cargo run

# ✅ 또는 수동 빌드
cargo build
```

### ❌ 실수 3: target 폴더 커밋

```bash
# .gitignore에 자동으로 target/ 포함됨
# (Cargo.toml과 src/ 폴더만 git에 올리기)
```

---

## 1️⃣2️⃣ 다음 단계

```bash
# 1. Hello World 이해
cd hello_world
cat src/main.rs

# 2. 프로그램 수정
# src/main.rs를 편집기로 열어서 코드 수정

# 3. 변경 사항 실행
cargo run

# 4. 컴파일만 확인
cargo check

# 5. 테스트 작성
# tests/ 디렉토리 생성 후 테스트 코드 작성
```

---

## 📌 핵심 정리

| 항목 | 설명 |
|------|------|
| **설치** | `curl ... sh.rustup.rs \| sh` |
| **확인** | `rustc --version`, `cargo --version` |
| **생성** | `cargo new 프로젝트명` |
| **실행** | `cargo run` |
| **빌드** | `cargo build` |
| **최적화** | `cargo build --release` |
| **구조** | Cargo.toml + src/main.rs |

---

## 🎯 체크리스트

- [ ] Rust 설치 완료
- [ ] rustc 버전 확인
- [ ] cargo 버전 확인
- [ ] hello_world 프로젝트 생성
- [ ] `cargo run` 으로 실행 성공
- [ ] src/main.rs 파일 수정 후 다시 실행
- [ ] Cargo.toml 파일 내용 이해

모든 항목을 완료했다면 Rust 개발 환경 구축 완료입니다! 🦀