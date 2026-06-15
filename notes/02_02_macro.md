# Rust 매크로 (Macro)

---

## 1️⃣ 매크로란?

**매크로는 컴파일 단계에서 코드를 다른 코드로 자동으로 치환(확장)하는 메커니즘입니다.**

### ! 기호의 의미

```rust
println!("Hello");  // println은 매크로
                    //        ↑
                    //      ! 기호 = 이것은 매크로다!

fn my_function() { }  // my_function은 함수 (! 없음)
```

---

## 2️⃣ 함수 vs 매크로 비교

### 함수 (Function)

```
컴파일 단계 → 실행 단계
            ↓
        함수 호출
           ↓
        함수 본문 실행
           ↓
        반환값 전달
```

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let result = add(5, 3);  // 함수 호출 시점에 실행
    println!("{}", result);
}
```

### 매크로 (Macro)

```
컴파일 단계
     ↓
매크로 호출 위치의 코드를 
다른 코드로 자동 치환 (확장)
     ↓
새로운 코드가 컴파일됨
     ↓
실행 단계
```

```rust
macro_rules! add_macro {
    ($x:expr, $y:expr) => {
        $x + $y
    }
}

fn main() {
    let result = add_macro!(5, 3);  // 컴파일 단계에서 "5 + 3"으로 치환됨
    println!("{}", result);
}
```

---

## 3️⃣ 시각적 비교

### 함수의 흐름

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let result = add(5, 3);
    // ↑
    // 런타임에 함수로 점프
}
```

**결과:**
```
main → add() 호출 → add 함수 실행 → main으로 반환
```

### 매크로의 흐름

```rust
macro_rules! add {
    ($x:expr, $y:expr) => ($x + $y)
}

fn main() {
    let result = add!(5, 3);
    
    // 컴파일 단계에서 아래처럼 변환됨:
    // let result = 5 + 3;
}
```

**결과:**
```
컴파일 단계: add!(5, 3) → 5 + 3 로 치환
실행 단계: let result = 5 + 3;
```

---

## 4️⃣ 자주 사용하는 표준 매크로들

### println! (출력)

```rust
fn main() {
    println!("Hello, world!");
    println!("x = {}", 5);
    println!("x = {}, y = {}", 5, 10);
}
```

**컴파일 단계에서 확장:**
```rust
// println!("Hello, world!") →
std::io::stdout().write_fmt(format_args!("Hello, world!"))

// println!("x = {}", 5) →
std::io::stdout().write_fmt(format_args!("x = {}", 5))
```

### format! (문자열 포맷팅)

```rust
fn main() {
    let s = format!("Hello, {}!", "Rust");
    println!("{}", s);  // Hello, Rust!
}
```

### vec! (벡터 생성)

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // 컴파일 단계에서:
    // let v = {
    //     let mut temp = Vec::new();
    //     temp.push(1);
    //     temp.push(2);
    //     temp.push(3);
    //     temp.push(4);
    //     temp.push(5);
    //     temp
    // };
}
```

### assert! (단언)

```rust
fn main() {
    assert!(5 > 3);           // ✅ 통과
    assert_eq!(5 + 3, 8);     // ✅ 같은지 확인
    assert_ne!(5, 3);         // ✅ 다른지 확인
}
```

### panic! (프로그램 중단)

```rust
fn main() {
    let number = 5;
    
    if number < 0 {
        panic!("음수가 입력되었습니다!");
    }
}
```

---

## 5️⃣ 매크로와 함수 비교 예제

### 예제 1: 배수 확인

#### 함수 방식

```rust
fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn main() {
    println!("{}", is_even(4));  // true
    println!("{}", is_even(5));  // false
}
```

#### 매크로 방식

```rust
macro_rules! is_even {
    ($x:expr) => {
        $x % 2 == 0
    }
}

fn main() {
    println!("{}", is_even!(4));  // true
    println!("{}", is_even!(5));  // false
}
```

---

## 6️⃣ 매크로의 장점

### 장점 1️⃣: 가변 인자 처리

```rust
// 함수: 고정된 개수의 매개변수
fn add_two(x: i32, y: i32) -> i32 {
    x + y
}

// 매크로: 가변 개수 처리 가능
macro_rules! add_all {
    ($($x:expr),*) => {
        0 $( + $x)*
    }
}

fn main() {
    println!("{}", add_all!(1));           // 1
    println!("{}", add_all!(1, 2));        // 3
    println!("{}", add_all!(1, 2, 3));     // 6
    println!("{}", add_all!(1, 2, 3, 4));  // 10
}
```

### 장점 2️⃣: 타입 무관 처리

```rust
// 함수: 타입 지정 필요
fn print_i32(x: i32) {
    println!("{}", x);
}

// 매크로: 모든 타입 처리 가능
macro_rules! print_any {
    ($x:expr) => {
        println!("{}", $x);
    }
}

fn main() {
    print_any!(5);
    print_any!(5.5);
    print_any!("hello");
    print_any!(true);
}
```

### 장점 3️⃣: 컴파일 타임 최적화

```rust
// 함수: 런타임에 계산
fn compute(x: i32) -> i32 {
    x * x + 2 * x + 1
}

// 매크로: 컴파일 타임에 계산 가능
macro_rules! compute {
    ($x:expr) => {
        $x * $x + 2 * $x + 1
    }
}

fn main() {
    let result = compute!(5);
    // 컴파일 단계에서: 5*5 + 2*5 + 1 = 36
}
```

---

## 7️⃣ 매크로의 단점

### 단점 1️⃣: 코드 크기 증가

```rust
macro_rules! repeat_code {
    () => {
        println!("A");
        println!("B");
        println!("C");
    }
}

fn main() {
    repeat_code!();
    repeat_code!();
    repeat_code!();
    
    // 컴파일 단계에서:
    // println!("A"); × 3
    // println!("B"); × 3
    // println!("C"); × 3
    // → 바이너리 크기 증가
}
```

### 단점 2️⃣: 디버깅 어려움

```rust
macro_rules! debug_add {
    ($x:expr, $y:expr) => {
        $x + $y  // 에러 발생 시 매크로 위치가 혼란스러움
    }
}

fn main() {
    let result = debug_add!(5, "3");  // ❌ 타입 에러
    // 에러 메시지가 명확하지 않을 수 있음
}
```

### 단점 3️⃣: 학습 곡선 가파름

```rust
macro_rules! complex_macro {
    // 복잡한 패턴 매칭 규칙
    ($($x:expr),* ; $($y:expr),*) => {
        // 어려운 문법...
    }
}
```

---

## 8️⃣ 매크로 정의 기초

### 간단한 매크로 정의

```rust
// 패턴: macro_rules! 매크로명 { 규칙들 }
macro_rules! greet {
    () => {
        println!("Hello!");
    }
}

fn main() {
    greet!();  // Hello!
}
```

### 매개변수가 있는 매크로

```rust
macro_rules! say {
    ($message:expr) => {
        println!("{}", $message);
    }
}

fn main() {
    say!("Hello, Rust!");
}
```

### 여러 패턴 규칙

```rust
macro_rules! max {
    ($x:expr, $y:expr) => {
        if $x > $y { $x } else { $y }
    };
}

fn main() {
    println!("{}", max!(5, 3));    // 5
    println!("{}", max!(2, 8));    // 8
}
```

---

## 9️⃣ 실무 예제

### 예제 1: 디버그 출력 매크로

```rust
macro_rules! debug_print {
    ($name:expr, $value:expr) => {
        println!("[DEBUG] {} = {}", $name, $value);
    }
}

fn main() {
    let x = 42;
    let y = "hello";
    
    debug_print!("x", x);      // [DEBUG] x = 42
    debug_print!("y", y);      // [DEBUG] y = hello
}
```

### 예제 2: 조건부 로깅

```rust
macro_rules! log {
    ($level:expr, $msg:expr) => {
        println!("[{}] {}", $level, $msg);
    }
}

fn main() {
    log!("INFO", "프로그램 시작");      // [INFO] 프로그램 시작
    log!("ERROR", "오류 발생");         // [ERROR] 오류 발생
    log!("WARNING", "경고 메시지");     // [WARNING] 경고 메시지
}
```

### 예제 3: 단위 변환

```rust
macro_rules! km_to_miles {
    ($km:expr) => {
        $km as f64 * 0.621371
    }
}

fn main() {
    let distance_km = 10;
    let distance_miles = km_to_miles!(distance_km);
    println!("{}km = {}miles", distance_km, distance_miles);
    // 10km = 6.21371miles
}
```

---

## 🔟 함수 vs 매크로 선택 기준

| 상황 | 함수 | 매크로 |
|------|------|--------|
| **간단한 작업** | ✅ 추천 | ❌ 과함 |
| **가변 인자** | ❌ 어려움 | ✅ 추천 |
| **타입 무관** | ❌ 어려움 | ✅ 추천 |
| **디버깅** | ✅ 쉬움 | ❌ 어려움 |
| **코드 재사용** | ✅ 좋음 | ⚠️ 중복 |
| **성능** | ✅ 런타임 | ✅ 컴파일타임 |
| **배우기** | ✅ 쉬움 | ❌ 어려움 |

---

## 1️⃣1️⃣ 컴파일 단계 시각화

### 함수의 경우

```
Source Code (main.rs)
    ↓
Parsing & Type Checking
    ↓
Code Generation (함수 호출 코드 생성)
    ↓
Compilation
    ↓
Binary (실행 파일)
    ↓
Runtime (함수 실행)
```

### 매크로의 경우

```
Source Code (main.rs)
    ↓
Macro Expansion (매크로 → 실제 코드로 치환)
    ↓
Parsing & Type Checking
    ↓
Code Generation
    ↓
Compilation
    ↓
Binary (실행 파일)
    ↓
Runtime
```

**차이점: 매크로 확장이 먼저 일어남!**

---

## 📌 핵심 정리

| 항목 | 함수 | 매크로 |
|------|------|--------|
| **기호** | 없음 | `!` |
| **시점** | 런타임 호출 | 컴파일타임 확장 |
| **동작** | 함수 실행 | 코드 치환 |
| **반환** | 반환값 | 확장된 코드 |
| **가변인자** | ❌ | ✅ |
| **타입** | 지정 필수 | 무관 가능 |

---

## 🎯 다시 한번 정리

```rust
println!("Hello");     // ! 있음 → 매크로
                       // 컴파일 단계에서 다른 코드로 치환됨

fn my_func() { }       // ! 없음 → 함수
                       // 런타임에 호출됨
```

이제 Rust의 매크로와 함수의 차이가 완벽하게 이해되셨을 거예요! 🦀