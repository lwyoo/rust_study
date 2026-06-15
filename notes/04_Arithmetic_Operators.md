# Rust 산술 연산 (Arithmetic Operations)

---

## 1️⃣ 기본 산술 연산자

| 연산자 | 이름 | 예제 | 결과 |
|--------|------|------|------|
| `+` | 덧셈 | `5 + 3` | `8` |
| `-` | 뺄셈 | `5 - 3` | `2` |
| `*` | 곱셈 | `5 * 3` | `15` |
| `/` | 나눗셈 | `6 / 3` | `2` |
| `%` | 나머지 | `7 % 3` | `1` |

---

## 2️⃣ 같은 타입끼리 연산 ✅

### 정수형 연산 (i32)

```rust
fn main() {
    let a: i32 = 10;
    let b: i32 = 3;
    
    println!("덧셈: {} + {} = {}", a, b, a + b);        // 13
    println!("뺄셈: {} - {} = {}", a, b, a - b);        // 7
    println!("곱셈: {} * {} = {}", a, b, a * b);        // 30
    println!("나눗셈: {} / {} = {}", a, b, a / b);      // 3
    println!("나머지: {} % {} = {}", a, b, a % b);      // 1
}
```

**출력:**
```
덧셈: 10 + 3 = 13
뺄셈: 10 - 3 = 7
곱셈: 10 * 3 = 30
나눗셈: 10 / 3 = 3
나머지: 10 % 3 = 1
```

### 실수형 연산 (f64)

```rust
fn main() {
    let x: f64 = 10.5;
    let y: f64 = 3.2;
    
    println!("덧셈: {:.1}", x + y);        // 13.7
    println!("뺄셈: {:.1}", x - y);        // 7.3
    println!("곱셈: {:.1}", x * y);        // 33.6
    println!("나눗셈: {:.2}", x / y);      // 3.28
    println!("나머지: {:.1}", x % y);      // 0.9
}
```

**출력:**
```
덧셈: 13.7
뺄셈: 7.3
곱셈: 33.6
나눗셈: 3.28
나머지: 0.9
```

---

## 3️⃣ ⚠️ 타입 불일치 - 자동 형변환 안 됨!

### ❌ 에러: i32 + i64

```rust
fn main() {
    let a: i32 = 10;
    let b: i64 = 5;
    let c = a + b;  // ❌ 컴파일 에러!
}
```

**에러 메시지:**
```
error[E0308]: mismatched types
expected `i32`, found `i64`
```

### ❌ 에러: 정수 + 실수

```rust
fn main() {
    let x: i32 = 10;
    let y: f64 = 3.5;
    let z = x + y;  // ❌ 컴파일 에러!
}
```

---

## 4️⃣ 다른 타입 간 연산 처리 방법

### 방법 1️⃣ : `as` 키워드로 형변환 (권장 ⭐⭐⭐⭐⭐)

```rust
fn main() {
    let a: i32 = 10;
    let b: i64 = 5;
    
    // i32를 i64로 변환
    let result1 = (a as i64) + b;
    println!("i32 as i64 + i64: {}", result1);  // 15
    
    // i64를 i32로 변환
    let result2 = a + (b as i32);
    println!("i32 + i64 as i32: {}", result2);  // 15
}
```

### 방법 2️⃣ : 쉐도잉으로 재선언

```rust
fn main() {
    let a: i32 = 10;
    let b: i64 = 5;
    
    // 쉐도잉으로 타입 변환
    let a = a as i64;  // i32 → i64로 재선언
    let result = a + b;
    println!("쉐도잉 결과: {}", result);  // 15
}
```

### 방법 3️⃣ : 별도 변수 생성

```rust
fn main() {
    let a: i32 = 10;
    let b: i64 = 5;
    
    // 새로운 변수로 형변환
    let a_i64 = a as i64;
    let result = a_i64 + b;
    println!("별도 변수 결과: {}", result);  // 15
}
```

---

## 5️⃣ 정수 + 실수 연산

### i32 + f64 연산

```rust
fn main() {
    let count: i32 = 5;      // 개수 (정수)
    let amount: f64 = 12.99; // 금액 (실수)
    
    // i32를 f64로 변환
    let total = (count as f64) * amount;
    println!("총액: {:.2}", total);  // 출력: 총액: 64.95
}
```

### f32 + f64 연산

```rust
fn main() {
    let x: f32 = 10.5;
    let y: f64 = 3.2;
    
    // f32를 f64로 변환
    let result = (x as f64) + y;
    println!("결과: {:.2}", result);  // 출력: 결과: 13.70
}
```

---

## 6️⃣ 부호 있는 정수 + 부호 없는 정수

```rust
fn main() {
    let signed: i32 = 10;    // 부호 있음 (-,+)
    let unsigned: u32 = 5;   // 부호 없음 (+만)
    
    // i32를 u32로 변환
    let result1 = (signed as u32) + unsigned;
    println!("i32 as u32 + u32: {}", result1);  // 15
    
    // u32를 i32로 변환
    let result2 = signed + (unsigned as i32);
    println!("i32 + u32 as i32: {}", result2);  // 15
}
```

---

## 7️⃣ 복합 산술 연산

### 여러 타입 혼합

```rust
fn main() {
    let a: i32 = 10;
    let b: i64 = 20;
    let c: f32 = 5.5;
    let d: f64 = 3.2;
    
    // 모두 f64로 통일
    let result = (a as f64) + (b as f64) + (c as f64) + d;
    println!("모든 타입 합: {:.1}", result);  // 38.7
}
```

### 우선순위 연산

```rust
fn main() {
    let x: f64 = 10.0;
    let y: f64 = 3.0;
    
    // 곱셈, 나눗셈이 먼저 실행
    let result = 2.0 * x + y / 2.0;  // (2*10) + (3/2) = 20 + 1.5
    println!("결과: {}", result);  // 21.5
    
    // 괄호로 우선순위 변경
    let result2 = 2.0 * (x + y) / 2.0;  // (10+3) * 2 / 2 = 13
    println!("괄호 사용: {}", result2);  // 13.0
}
```

---

## 💼 실무 예제 1: 상품 가격 계산

```rust
fn main() {
    let quantity: i32 = 3;      // 수량 (정수)
    let unit_price: f64 = 99.99; // 단가 (실수)
    let tax_rate: f64 = 0.1;     // 세율 (실수)
    
    // 수량을 f64로 변환하여 연산
    let subtotal = (quantity as f64) * unit_price;
    let tax = subtotal * tax_rate;
    let total = subtotal + tax;
    
    println!("소계: {:.2}원", subtotal);  // 299.97원
    println!("세금: {:.2}원", tax);       // 30.00원
    println!("합계: {:.2}원", total);     // 329.97원
}
```

---

## 💼 실무 예제 2: 시간 단위 변환

```rust
fn main() {
    let hours: i32 = 2;
    let minutes: i32 = 30;
    let seconds: f64 = 45.5;
    
    // 모두 초(seconds)로 변환
    let total_seconds = (hours as f64) * 3600.0 
                      + (minutes as f64) * 60.0 
                      + seconds;
    
    println!("총 시간: {:.1}초", total_seconds);  // 9045.5초
}
```

---

## 💼 실무 예제 3: 학점 계산

```rust
fn main() {
    let math: i32 = 85;
    let english: f64 = 92.5;
    let korean: i32 = 88;
    
    // 모두 f64로 변환하여 평균 계산
    let average = ((math as f64) + english + (korean as f64)) / 3.0;
    
    println!("수학: {}점", math);
    println!("영어: {}점", english);
    println!("국어: {}점", korean);
    println!("평균: {:.1}점", average);  // 88.5점
}
```

---

## ⚠️ 주의사항

### 1️⃣ 정수 나눗셈은 소수점 버림

```rust
fn main() {
    let a: i32 = 10;
    let b: i32 = 3;
    
    println!("{}", a / b);  // 출력: 3 (1 손실!)
    
    // 정확한 나눗셈: f64로 변환
    println!("{:.2}", (a as f64) / (b as f64));  // 출력: 3.33
}
```

### 2️⃣ 실수 → 정수 변환은 소수 버림

```rust
fn main() {
    let f: f64 = 3.9;
    let i = f as i32;
    println!("{}", i);  // 출력: 3 (0.9 손실!)
}
```

### 3️⃣ 오버플로우 위험

```rust
fn main() {
    let big: i32 = 300;
    let small: u8 = big as u8;  // u8 범위: 0-255
    println!("{}", small);  // 출력: 44 (오버플로우!)
}
```

---

## 📊 타입별 범위

| 타입 | 범위 | 크기 |
|------|------|------|
| `i8` | -128 ~ 127 | 1바이트 |
| `i32` | -2,147,483,648 ~ 2,147,483,647 | 4바이트 |
| `i64` | -9,223,372,036,854,775,808 ~ 9,223,372,036,854,775,807 | 8바이트 |
| `u8` | 0 ~ 255 | 1바이트 |
| `u32` | 0 ~ 4,294,967,295 | 4바이트 |
| `f32` | ±1.4 × 10^-45 ~ ±3.4 × 10^38 | 4바이트 |
| `f64` | ±2.2 × 10^-308 ~ ±1.8 × 10^308 | 8바이트 |

---

## 📌 핵심 정리

| 규칙 | 설명 |
|------|------|
| **자동 형변환 없음** | 다른 타입은 명시적 변환 필수 |
| **같은 타입만 연산** | `i32 + i32` ✅, `i32 + i64` ❌ |
| **형변환 방법** | `as` 키워드, 쉐도잉, 별도 변수 |
| **나눗셈** | 정수는 소수 버림, 정확함은 f64 사용 |
| **우선순위** | `* / %` > `+ -` |

---

## ✅ 형변환 패턴 정리

```rust
// 패턴 1: 한쪽만 변환
let result = (a as i64) + b;

// 패턴 2: 쉐도잉
let a = a as i64;
let result = a + b;

// 패턴 3: 모두 통일 (복합 연산)
let result = (a as f64) + (b as f64) + (c as f64);
```

이제 Rust의 산술 연산과 타입 처리가 명확하죠? 🦀