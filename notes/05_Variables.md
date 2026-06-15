# Rust 변수 (Variables)

## 1️⃣ 불변 변수 (Immutable - 기본값)
* **선언 방법**
```rust
  let n = 12;
```
* **특징**
  - 변수 타입은 **타입 추론(type inference)**으로 자동 결정됨
  - 한번 선언하면 값을 변경할 수 없음

* **명시적 타입 지정** (선택사항)
```rust
  let n: i32 = 12;  // 타입을 명시적으로 지정
```

---

## 2️⃣ 가변 변수 (Mutable)
* **선언 방법**
```rust
  let mut n = 12;  // mut 키워드 필수
```
* **특징**
  - 값을 언제든 변경 가능
```rust
  let mut n = 12;
  n = 13;  // ✅ 가능
```

---

## 3️⃣ 미사용 변수 (Unused Variable)
* **경고 제거 방법**
```rust
  let _ = 12;              // 가장 간단한 방법
  let _n = 12;             // 또는 언더스코어 접두사 사용
  let _unused_var = 12;    // 의도를 명확히 하기
```

---

## 4️⃣ Static 변수 (프로그램 전체 생명주기)
```rust
static CONSTANT: i32 = 100;

fn main() {
    println!("{}", CONSTANT);  // 반드시 사용해야 함
}
```

---

## 5️⃣ Const (컴파일 타임 상수)
```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("{}", MAX_POINTS);
}
```

---

## 📌 핵심 정리
| 종류 | 선언 | 변경 | 생명주기 |
|------|------|------|--------|
| **let** | `let x = 5;` | ❌ | 함수 스코프 |
| **let mut** | `let mut x = 5;` | ✅ | 함수 스코프 |
| **static** | `static X: i32 = 5;` | ❌ (mut 가능) | 프로그램 전체 |
| **const** | `const X: i32 = 5;` | ❌ | 컴파일 타임 |