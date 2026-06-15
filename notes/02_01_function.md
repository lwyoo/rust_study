# Rust 함수 구조 (Function)

---

## 1️⃣ 함수의 기본 구조

```rust
fn function_name(param1: type1, param2: type2) -> return_type {
    // 함수 본문 (블록 코드)
    // 처리 로직
    return_value
}
```

### 각 부분 설명

| 부분 | 설명 | 예시 |
|------|------|------|
| **fn** | function 키워드 (함수 선언 필수) | `fn` |
| **함수명** | 사용자가 임의로 정하는 이름 | `add_numbers` |
| **매개변수** | 입력값 (입력 창구) | `(x: i32, y: i32)` |
| **반환 타입** | 반환값의 타입 | `-> i32` |
| **블록 코드** | 함수 본문 (중괄호 안) | `{ ... }` |

---

## 2️⃣ 간단한 함수 예제

### 예제 1: 매개변수 없는 함수

```rust
fn greet() {
    println!("Hello, Rust!");
}

fn main() {
    greet();  // 함수 호출
}
```

**출력:**
```
Hello, Rust!
```

### 예제 2: 매개변수 있는 함수

```rust
fn greet_with_name(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet_with_name("Dogyeom");
}
```

**출력:**
```
Hello, Dogyeom!
```

### 예제 3: 반환값 있는 함수

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // 반환값 (세미콜론 없음!)
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}
```

**출력:**
```
5 + 3 = 8
```

---

## 3️⃣ 함수의 4가지 형태

### 형태 1️⃣: 매개변수 X, 반환값 X

```rust
fn say_hello() {
    println!("Hello!");
}

fn main() {
    say_hello();
}
```

### 형태 2️⃣: 매개변수 O, 반환값 X

```rust
fn print_number(n: i32) {
    println!("숫자: {}", n);
}

fn main() {
    print_number(42);
}
```

### 형태 3️⃣: 매개변수 X, 반환값 O

```rust
fn get_answer() -> i32 {
    42
}

fn main() {
    let answer = get_answer();
    println!("답: {}", answer);
}
```

### 형태 4️⃣: 매개변수 O, 반환값 O (가장 일반적)

```rust
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let result = multiply(6, 7);
    println!("6 * 7 = {}", result);
}
```

---

## 4️⃣ 매개변수 (입력 창구)

### 단일 매개변수

```rust
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    println!("{}", square(5));  // 25
}
```

### 다중 매개변수

```rust
fn add_three_numbers(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

fn main() {
    println!("{}", add_three_numbers(1, 2, 3));  // 6
}
```

### 다양한 타입의 매개변수

```rust
fn describe_person(name: &str, age: i32, height: f64) {
    println!("이름: {}", name);
    println!("나이: {}살", age);
    println!("키: {}cm", height);
}

fn main() {
    describe_person("Dogyeom", 10, 175.5);
}
```

**출력:**
```
이름: Dogyeom
나이: 10살
키: 175.5cm
```

---

## 5️⃣ 반환값 (Return Type)

### 반환 타입 선언

```rust
// i32를 반환
fn get_number() -> i32 {
    42
}

// String을 반환
fn get_name() -> String {
    String::from("Rust")
}

// bool을 반환
fn is_positive(n: i32) -> bool {
    n > 0
}
```

### 반환값 작성 방법

```rust
// 방법 1: 명시적 return 사용
fn add_v1(x: i32, y: i32) -> i32 {
    return x + y;  // 세미콜론 있음
}

// 방법 2: 마지막 표현식 (권장)
fn add_v2(x: i32, y: i32) -> i32 {
    x + y  // 세미콜론 없음!
}

fn main() {
    println!("{}", add_v1(3, 4));  // 7
    println!("{}", add_v2(3, 4));  // 7
}
```

**⚠️ 중요한 차이:**
```rust
fn with_semicolon() -> i32 {
    5;  // ❌ 이것은 statement (값이 아님)
}

fn without_semicolon() -> i32 {
    5   // ✅ 이것은 expression (값을 반환)
}
```

---

## 6️⃣ 함수 본문 (블록 코드)

### 간단한 본문

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

### 복잡한 본문

```rust
fn calculate(x: i32, y: i32) -> i32 {
    // 변수 선언
    let sum = x + y;
    
    // 조건 처리
    if sum > 0 {
        sum * 2
    } else {
        sum
    }
}

fn main() {
    println!("{}", calculate(5, 3));    // 16 (8*2)
    println!("{}", calculate(-5, 3));   // -2
}
```

---

## 7️⃣ 실무 예제

### 예제 1: 학점 계산

```rust
fn calculate_grade(score: f64) -> &'static str {
    if score >= 90.0 {
        "A"
    } else if score >= 80.0 {
        "B"
    } else if score >= 70.0 {
        "C"
    } else {
        "F"
    }
}

fn main() {
    println!("85점: {}", calculate_grade(85.0));  // B
    println!("92점: {}", calculate_grade(92.0));  // A
}
```

### 예제 2: 온도 변환

```rust
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let c = 25.0;
    let f = celsius_to_fahrenheit(c);
    println!("{}°C = {}°F", c, f);  // 25°C = 77°F
    
    println!("{}°F = {}°C", f, fahrenheit_to_celsius(f));  // 77°F = 25°C
}
```

### 예제 3: 벡터의 합계 계산

```rust
fn sum_vector(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers {
        sum += num;
    }
    sum
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    println!("합계: {}", sum_vector(&nums));  // 15
}
```

---

## 8️⃣ 함수 명명 규칙 (Naming Convention)

### ✅ Rust 권장 규칙 (snake_case)

```rust
fn calculate_average() { }     // ✅ 좋음
fn add_numbers() { }           // ✅ 좋음
fn is_positive() { }           // ✅ 좋음
fn get_user_name() { }         // ✅ 좋음

fn calculateAverage() { }      // ❌ 나쁨 (camelCase)
fn CalculateAverage() { }      // ❌ 나쁨 (PascalCase)
fn calculate_average_v1() { }  // ⚠️ 버전이 필요하면 끝에
```

### 함수명 접두사 관례

```rust
// 불린값 반환: is_, has_, can_
fn is_positive(n: i32) -> bool { n > 0 }
fn has_permission(user_id: i32) -> bool { true }
fn can_proceed() -> bool { true }

// 값을 얻음: get_
fn get_user_name() -> String { String::from("Rust") }

// 값을 변환: to_, into_
fn to_uppercase(s: &str) -> String { s.to_uppercase() }

// 계산: calculate_, compute_
fn calculate_total(items: &[i32]) -> i32 { 0 }

// 출력: print_, display_
fn print_results(results: &[i32]) { }
```

---

## 9️⃣ 함수 내부에서 함수 호출

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn add_then_multiply(a: i32, b: i32, c: i32) -> i32 {
    let sum = add(a, b);           // 함수 호출
    multiply(sum, c)               // 또 다른 함수 호출
}

fn main() {
    println!("{}", add_then_multiply(2, 3, 4));  // (2+3)*4 = 20
}
```

---

## 🔟 함수 구조 다시 정리

```rust
fn function_name(parameter: type) -> return_type {
    //   ↑              ↑         ↑              ↑
    //  키워드        입력창구   화살표      반환타입
    //                         (생략가능)
    
    // 블록 코드 (함수 본문)
    let result = parameter * 2;
    result  // 반환값 (세미콜론 없음)
}
```

---

## 1️⃣1️⃣ 자주 하는 실수

### ❌ 실수 1: 반환값에 세미콜론

```rust
fn get_number() -> i32 {
    42;  // ❌ 에러! (statement가 됨)
}

// 에러 메시지:
// mismatched types
// expected `i32`, found `()`
```

### ❌ 실수 2: 반환 타입 선언 안 함

```rust
fn add(x: i32, y: i32) {  // ❌ 반환값이 있는데 타입 선언 없음
    x + y
}
```

### ❌ 실수 3: 매개변수 타입 생략

```rust
fn add(x, y) -> i32 {  // ❌ 에러! 각 매개변수마다 타입 필수
    x + y
}

// ✅ 올바름
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

---

## 📌 함수 구조 체크리스트

```
fn 함수명 ( 매개변수: 타입, ... ) -> 반환타입 {
  ✅ fn 키워드
  ✅ 함수명 (snake_case)
  ✅ 매개변수 (타입 필수)
  ✅ 반환 타입 (있으면 ->)
  ✅ 블록 코드 { }
  ✅ 반환값 (세미콜론 없음)
}
```

---

## 🎯 핵심 정리

| 항목 | 설명 |
|------|------|
| **fn** | 함수 선언 키워드 |
| **함수명** | snake_case 사용 |
| **매개변수** | 타입 반드시 지정 |
| **반환값** | `-> type` 선언 |
| **블록 코드** | `{ }` 안의 처리 로직 |
| **반환 방식** | expression (세미콜론 없음) |
| **4가지 형태** | X,X / O,X / X,O / O,O |

이제 Rust 함수 구조가 완벽하게 이해되셨을 거예요! 🦀