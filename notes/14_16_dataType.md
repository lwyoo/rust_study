# Rust 자료형 (Data Types)

---

## 1️⃣ 정수형 (Integer Types)

### 부호 있는 정수 (Signed Integers)

**음수와 양수를 모두 표현할 수 있습니다.**

| 타입 | 비트 수 | 범위 |
|------|---------|------|
| `i8` | 8비트 | -128 ~ 127 |
| `i16` | 16비트 | -32,768 ~ 32,767 |
| `i32` | 32비트 (기본) | -2,147,483,648 ~ 2,147,483,647 |
| `i64` | 64비트 | -9,223,372,036,854,775,808 ~ 9,223,372,036,854,775,807 |
| `i128` | 128비트 | 매우 큼 |
| `isize` | 포인터 크기 | 플랫폼 의존 (32/64bit) |

#### 부호 있는 정수 예제

```rust
fn main() {
    let x: i32 = -42;      // 32비트 부호 있는 정수
    let y: i8 = -128;      // 8비트 최소값
    let z: i64 = 1_000_000_000;  // 큰 수
    
    println!("x = {}", x);      // -42
    println!("y = {}", y);      // -128
    println!("z = {}", z);      // 1000000000
}
```

### 부호 없는 정수 (Unsigned Integers)

**양수만 표현합니다. 최상위 비트를 부호로 사용하지 않습니다.**

| 타입 | 비트 수 | 범위 |
|------|---------|------|
| `u8` | 8비트 | 0 ~ 255 |
| `u16` | 16비트 | 0 ~ 65,535 |
| `u32` | 32비트 | 0 ~ 4,294,967,295 |
| `u64` | 64비트 | 0 ~ 18,446,744,073,709,551,615 |
| `u128` | 128비트 | 매우 큼 |
| `usize` | 포인터 크기 | 플랫폼 의존 |

#### 부호 없는 정수 예제

```rust
fn main() {
    let a: u8 = 255;       // 8비트 최대값
    let b: u16 = 1000;     // 16비트
    let c: u32 = 3_000_000_000;  // 32비트
    
    println!("a = {}", a);      // 255
    println!("b = {}", b);      // 1000
    println!("c = {}", c);      // 3000000000
}
```

---

## 2️⃣ 십진수 이외의 숫자 표현

### 16진수 (Hexadecimal)

```rust
fn main() {
    let hex = 0xFF;        // 255
    let hex2 = 0x10;       // 16
    
    println!("0xFF = {}", hex);     // 255
    println!("0x10 = {}", hex2);    // 16
}
```

**용도:** 색상, 메모리 주소, 바이트 값

### 8진수 (Octal)

```rust
fn main() {
    let octal = 0o10;      // 8
    let octal2 = 0o77;     // 63
    
    println!("0o10 = {}", octal);    // 8
    println!("0o77 = {}", octal2);   // 63
}
```

### 2진수 (Binary)

```rust
fn main() {
    let binary = 0b1010;   // 10
    let binary2 = 0b1111;  // 15
    
    println!("0b1010 = {}", binary);   // 10
    println!("0b1111 = {}", binary2);  // 15
}
```

### 10진수 (Decimal) - 기본

```rust
fn main() {
    let decimal = 255;     // 255
    let decimal2 = 10;     // 10
    
    println!("255 = {}", decimal);   // 255
    println!("10 = {}", decimal2);   // 10
}
```

### 언더스코어로 가독성 향상

```rust
fn main() {
    let million = 1_000_000;       // 1,000,000
    let binary = 0b1111_0000;      // 11110000
    let hex = 0xFF_FF_FF_FF;       // FFFFFFFF
    
    println!("1,000,000 = {}", million);
    println!("0b1111_0000 = {}", binary);
    println!("0xFFFFFFFF = {}", hex);
}
```

---

## 3️⃣ 실수형 (Floating Point)

### 기본 실수형

```rust
fn main() {
    let x: f32 = 3.14;     // 32비트 실수
    let y: f64 = 2.71828;  // 64비트 실수 (기본)
    
    println!("f32: {}", x);
    println!("f64: {}", y);
}
```

### 지수 표현 (Scientific Notation)

```rust
fn main() {
    let exp1 = 1e2;        // 1 × 10² = 100
    let exp2 = 1.5e3;      // 1.5 × 10³ = 1500
    let exp3 = 2e-1;       // 2 × 10⁻¹ = 0.2
    
    println!("1e2 = {}", exp1);        // 100
    println!("1.5e3 = {}", exp2);      // 1500
    println!("2e-1 = {}", exp3);       // 0.2
}
```

---

## 4️⃣ 형변환 (Type Casting)

### as 키워드 사용

```rust
fn main() {
    let x: i32 = 100;
    let y: f64 = x as f64;     // i32 → f64
    
    println!("i32: {}", x);
    println!("f64: {}", y);    // 100.0
}
```

### 정수 ↔ 정수 형변환

```rust
fn main() {
    let a: u8 = 255;
    let b: u32 = a as u32;     // u8 → u32
    
    let c: i32 = 100;
    let d: u32 = c as u32;     // i32 → u32
    
    println!("u8 255 → u32 {}", b);
    println!("i32 100 → u32 {}", d);
}
```

### 정수 → 실수

```rust
fn main() {
    let a: i32 = 10;
    let b: f64 = a as f64;     // 정수 → 실수
    
    println!("10 as f64 = {}", b);  // 10.0
}
```

### 실수 → 정수 (소수 버림)

```rust
fn main() {
    let x: f64 = 3.9;
    let y: i32 = x as i32;     // 소수 버림!
    
    println!("3.9 as i32 = {}", y);  // 3
}
```

### ⚠️ 오버플로우 주의

```rust
fn main() {
    let large: i32 = 300;
    let small: u8 = large as u8;   // u8 범위: 0-255
    
    println!("300 as u8 = {}", small);  // 44 (오버플로우!)
}
```

---

## 5️⃣ bool 타입

```rust
fn main() {
    let is_true: bool = true;
    let is_false: bool = false;
    
    println!("true: {}", is_true);
    println!("false: {}", is_false);
    
    // 조건식 결과
    let x = 5;
    let result = x > 3;  // true
    println!("5 > 3: {}", result);
}
```

---

## 6️⃣ char 타입

### 단일 문자

```rust
fn main() {
    let c: char = 'A';
    let emoji: char = '😀';
    
    println!("문자: {}", c);      // A
    println!("이모지: {}", emoji); // 😀
}
```

### char 메서드

```rust
fn main() {
    let ch = 'a';
    
    println!("대문자? {}", ch.is_uppercase());  // false
    println!("소문자? {}", ch.is_lowercase());  // true
    println!("숫자? {}", ch.is_numeric());      // false
    
    println!("대문자 변환: {}", ch.to_uppercase());  // A
}
```

---

## 7️⃣ 튜플 (Tuple)

### 개념

**여러 타입의 값을 하나로 묶는 자료형입니다.**

### 기본 튜플

```rust
fn main() {
    // 다양한 타입을 하나에 묶기
    let person: (String, i32, f64) = ("Alice".to_string(), 25, 5.8);
    
    println!("이름: {}", person.0);  // Alice
    println!("나이: {}", person.1);  // 25
    println!("키: {}", person.2);    // 5.8
}
```

### 튜플 언팩 (Destructuring)

```rust
fn main() {
    let (name, age, height) = ("Bob", 30, 5.9);
    
    println!("이름: {}", name);      // Bob
    println!("나이: {}", age);       // 30
    println!("키: {}", height);      // 5.9
}
```

### 튜플 반환값

```rust
fn get_coordinates() -> (i32, i32) {
    (10, 20)
}

fn main() {
    let (x, y) = get_coordinates();
    println!("좌표: ({}, {})", x, y);  // (10, 20)
}
```

### 빈 튜플 (Unit Type)

```rust
fn main() {
    let empty: () = ();
    
    println!("빈 튜플: {:?}", empty);  // ()
}
```

**의미:** C/C++의 `void`와 유사 (반환값 없음)

---

## 8️⃣ 배열 타입 지정

### 배열 타입

```rust
fn main() {
    // 타입: [요소타입; 크기]
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("배열: {:?}", arr);
    println!("길이: {}", arr.len());
}
```

### 2D 배열 타입

```rust
fn main() {
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    
    println!("행렬: {:?}", matrix);
}
```

---

## 9️⃣ 벡터 타입 지정

### 벡터 타입

```rust
fn main() {
    // 타입: Vec<요소타입>
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    println!("벡터: {:?}", vec);
    println!("길이: {}", vec.len());
}
```

### 다양한 요소 타입

```rust
fn main() {
    let vec_int: Vec<i32> = vec![1, 2, 3];
    let vec_str: Vec<String> = vec!["a".to_string(), "b".to_string()];
    let vec_bool: Vec<bool> = vec![true, false, true];
    
    println!("정수: {:?}", vec_int);
    println!("문자열: {:?}", vec_str);
    println!("불린: {:?}", vec_bool);
}
```

---

## 🔟 상수 (Constants)

### 개념

**컴파일 타임에 값이 결정되는 변수입니다.**

### 상수 선언

```rust
const N: usize = 20;

fn main() {
    let array = [0; N];
    println!("N: {}, array: {:?}", N, array);
    // N: 20, array: [0, 0, ..., 0]
}
```

### 상수의 특징

```rust
const PI: f64 = 3.14159;
const MAX_SIZE: usize = 1000;
const MESSAGE: &str = "Hello, Rust!";

fn main() {
    println!("PI: {}", PI);
    println!("MAX_SIZE: {}", MAX_SIZE);
    println!("MESSAGE: {}", MESSAGE);
}
```

### let vs const

| 특징 | let | const |
|------|-----|-------|
| **타입 지정** | 선택 | 필수 |
| **값 변경** | mut 사용 가능 | 불가능 |
| **스코프** | 블록 내 | 전역/모듈 |
| **컴파일 타임** | 런타임 | 컴파일 타임 |
| **최적화** | 기본 | 높음 |

### 실무 예제: 상수 사용

```rust
const BUFFER_SIZE: usize = 256;
const MAX_CONNECTIONS: i32 = 100;
const DEBUG_MODE: bool = true;

fn main() {
    let mut buffer = [0; BUFFER_SIZE];
    println!("버퍼 크기: {}", buffer.len());
    
    if DEBUG_MODE {
        println!("디버그 모드 활성화");
    }
}
```

---

## 1️⃣1️⃣ 타입 추론 vs 명시적 지정

### 타입 추론

```rust
fn main() {
    let x = 42;            // i32로 추론
    let y = 3.14;          // f64로 추론
    let z = true;          // bool로 추론
}
```

### 명시적 타입 지정

```rust
fn main() {
    let x: i64 = 42;       // 명시적 i64
    let y: f32 = 3.14;     // 명시적 f32
    let z: bool = true;    // 명시적 bool
}
```

---

## 1️⃣2️⃣ 실무 예제

### 예제 1: RGB 색상

```rust
fn main() {
    // u8: 0-255 범위에 완벽
    let red: u8 = 255;
    let green: u8 = 128;
    let blue: u8 = 0;
    
    println!("RGB({}, {}, {})", red, green, blue);
}
```

### 예제 2: 좌표와 거리

```rust
fn main() {
    let point: (i32, i32) = (10, 20);
    let distance: f64 = 5.5;
    
    println!("좌표: {:?}", point);
    println!("거리: {}", distance);
}
```

### 예제 3: 설정값

```rust
const MAX_RETRIES: u32 = 3;
const TIMEOUT_MS: u32 = 5000;

fn main() {
    let mut retries = 0;
    
    while retries < MAX_RETRIES {
        println!("재시도 {}/{}", retries + 1, MAX_RETRIES);
        retries += 1;
    }
}
```

### 예제 4: 여러 데이터 묶기

```rust
fn get_user_info() -> (String, i32, bool) {
    ("Alice".to_string(), 25, true)
}

fn main() {
    let (name, age, active) = get_user_info();
    println!("사용자: {}, 나이: {}, 활성: {}", name, age, active);
}
```

---

## 📌 핵심 정리

```rust
// 정수형
let a: i32 = 100;        // 부호 있는 32비트
let b: u8 = 255;         // 부호 없는 8비트

// 다양한 진수
let hex = 0xFF;          // 16진수
let oct = 0o77;          // 8진수
let bin = 0b1010;        // 2진수

// 실수형
let x: f64 = 3.14;       // 64비트 실수
let y = 1e2;             // 지수 표현: 100

// 형변환
let z: i32 = (x as i32); // f64 → i32

// bool과 char
let flag: bool = true;
let ch: char = 'A';

// 튜플
let tuple = (10, 20, "hello");
let (a, b, c) = tuple;

// 상수
const PI: f64 = 3.14159;

// 배열과 벡터
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let vec: Vec<i32> = vec![1, 2, 3];
```

---

## 🎯 타입 선택 가이드

| 상황 | 추천 타입 |
|------|---------|
| 정수, 음수 불필요 | `u8`, `u16`, `u32`, `u64` |
| 정수, 음수 필요 | `i8`, `i16`, `i32`, `i64` |
| 소수 | `f32`, `f64` |
| 255 이하 | `u8` |
| 참/거짓 | `bool` |
| 단일 문자 | `char` |
| 여러 타입 혼합 | `tuple` |
| 같은 타입 여러 개 | `array` 또는 `Vec` |

이제 Rust의 모든 기본 자료형을 이해하셨습니다! 🦀