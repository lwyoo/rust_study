# 🦀 Rust 난이도 5단계 로드맵
## Level 1 (초보) → Level 5 (고급)까지의 명확한 도달 기준

---

## 📊 전체 로드맵 요약

```
Level 1: 기초 문법 (2주)
  └─ 변수, 함수, 제어문, 기본 타입 이해
  └─ "Hello, World!"부터 간단한 CLI 도구까지

Level 2: 메모리 관리 (3주) ⭐ 핵심
  └─ Ownership, Borrowing, Lifetime 마스터
  └─ 안전성의 진정한 의미 이해

Level 3: 타입 시스템 (3주)
  └─ Traits, Generics, Pattern Matching
  └─ 재사용 가능하고 확장성 있는 코드 작성

Level 4: 실무 능력 (4주)
  └─ Async/Await, 멀티스레딩, 모듈 시스템
  └─ 실제 프로젝트에 적용 가능한 수준

Level 5: 전문가 (진행 중)
  └─ FFI, Unsafe Rust, 최적화, 도메인 특화
  └─ 오픈소스 기여, 라이브러리 설계
```

---

## 🎯 Level 1: 기초 문법 (초보자)
### 학습 시간: 2주 | 선행 조건: 프로그래밍 기본 개념

### 📚 학습 주제
- 변수와 뮤테이션
- 기본 타입 (i32, f64, bool, char)
- 문자열 (String vs &str 기초)
- 함수 정의와 호출
- 제어문 (if/else, loop, for, while)
- 주석과 문서화

### ✅ Level 1 도달 기준 (모두 할 수 있어야 함)

**기초 능력:**
- [ ] cargo를 사용해 새로운 프로젝트 생성 가능 (`cargo new`)
- [ ] 기본 변수 선언과 mut 키워드 사용 이해
- [ ] 함수 작성 (파라미터, 반환값) 가능
- [ ] println! 매크로로 변수 출력 가능

**문법:**
- [ ] if/else 문으로 분기 처리 가능
- [ ] for 루프로 컬렉션 순회 가능
- [ ] 간단한 패턴 매칭 (match 문) 이해

**코드 작성:**
- [ ] 명령행 인수를 받아서 처리하는 프로그램 작성 가능
- [ ] 숫자를 입력받아 계산하고 결과 출력 가능
- [ ] 간단한 함수 여러 개를 조합해서 작동하는 프로그램 만들 수 있음

### 📋 Level 1 체크리스트

```rust
// 이 정도는 이해하고 직접 작성할 수 있어야 함

fn main() {
    let x = 5;
    let mut y = 10;
    y += x;
    
    let result = add(x, y);
    println!("Result: {}", result);
    
    for i in 1..=5 {
        println!("Number: {}", i);
    }
    
    match result {
        15 => println!("Fifteen"),
        _ => println!("Other: {}", result),
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 🎓 실습 프로젝트

**프로젝트 1:** 온도 변환기
```rust
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    let temps = vec![0.0, 10.0, 20.0, 30.0];
    for temp in temps {
        println!("{}°C = {}°F", temp, celsius_to_fahrenheit(temp));
    }
}
```

**프로젝트 2:** 단어 세기 CLI
```
$ cargo run -- "hello world rust"
Word count: 3
Sorted: ["hello", "rust", "world"]
```

**프로젝트 3:** 피보나치 계산기
```rust
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}
```

### 📖 학습 자료

**필독:**
- The Rust Book - Ch. 1, 2, 3, 4, 5 (함수, 제어문까지)
- Rust by Example - Variables and Bindings, Flow Control

**참고:**
- Rust Playground (https://play.rust-lang.org/)로 온라인 실습
- rustlings 초급 과제 (variables, functions, conditionals)

### ⏭️ Level 2로 진행하기 전 확인

```
☐ rustlings 완료: variables, functions, conditionals
☐ 작성한 함수가 모두 컴파일되고 실행됨
☐ let/mut 구분이 명확함
☐ 함수 시그니처를 스스로 작성할 수 있음
☐ 패턴 매칭 기초 (match 문) 이해
```

---

## 🎯 Level 2: 메모리 관리 (중급)
### 학습 시간: 3주 | 선행 조건: Level 1 완료

### ⭐ 가장 중요한 레벨

Rust와 다른 언어의 **가장 큰 차이**는 메모리 관리입니다. 이 레벨을 마스터하면 Rust의 80%를 이해한 것입니다.

### 📚 학습 주제

**1. Ownership (소유권)**
- 소유권 규칙 3가지
- Move semantics
- Copy vs Clone
- 함수와 소유권

**2. Borrowing (차용)**
- 불변 참조 (&T)
- 가변 참조 (&mut T)
- 차용 규칙
- Dangling reference 방지

**3. Lifetime (생명주기)**
- 기본 개념
- Lifetime annotation
- 함수의 lifetime
- 구조체의 lifetime

### ✅ Level 2 도달 기준 (모두 할 수 있어야 함)

**Ownership:**
- [ ] Move가 발생하는 상황 정확히 인식 가능
- [ ] Copy trait를 갖는 타입과 아닌 타입 구분
- [ ] 함수에 소유권을 전달하는 것 vs 참조를 전달하는 것의 차이 이해
- [ ] 소유권 이동으로 인한 컴파일 에러 스스로 해결 가능

**Borrowing:**
- [ ] 불변 참조 여러 개는 가능하지만, 가변 참조는 하나만 가능한 이유 이해
- [ ] "차용 규칙" 3가지를 설명할 수 있음
- [ ] 컴파일러 에러 메시지를 읽고 해결책 찾을 수 있음
- [ ] 함수 파라미터에 &T, &mut T, T를 적절히 사용

**Lifetime:**
- [ ] 함수 시그니처에 lifetime annotation 작성 가능
- [ ] 컴파일러가 자동으로 추론하는 경우 이해
- [ ] 명시적으로 lifetime을 작성해야 하는 경우 판단 가능

### 📋 Level 2 체크리스트

**1. Move 이해하기**
```rust
// 이 코드에서 뭐가 작동하고 뭐가 에러인지 설명 가능?
let s1 = String::from("hello");
let s2 = s1;
// println!("{}", s1);  // ❌ error!
println!("{}", s2);     // ✓ ok

let x = 5;
let y = x;
println!("{}, {}", x, y);  // ✓ ok (Copy)
```

**2. 차용 규칙 실습**
```rust
let mut s = String::from("hello");

let r1 = &s;      // 불변 참조 1
let r2 = &s;      // 불변 참조 2
// let r3 = &mut s;  // ❌ error! 불변과 가변 동시 사용 불가

println!("{}, {}", r1, r2);
let r3 = &mut s;  // ✓ ok (r1, r2가 더 이상 사용 안 됨)
r3.push_str(" world");
println!("{}", r3);
```

**3. Lifetime annotation**
```rust
// 이 함수 시그니처를 작성할 수 있음?
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 이 구조체도 작성 가능?
struct Article<'a> {
    title: &'a str,
    content: &'a str,
}
```

**4. 실무 코드**
```rust
// 파일의 각 라인을 처리하면서
// 참조와 소유권을 올바르게 사용
fn process_lines(data: &str) -> Vec<String> {
    data.lines()
        .map(|line| line.to_uppercase())
        .collect()
}

fn main() {
    let input = String::from("hello\nworld");
    let result = process_lines(&input);
    println!("{:?}", result);
    println!("{}", input);  // 여전히 사용 가능!
}
```

### 🎓 Level 2 실습 프로젝트

**프로젝트 1: 텍스트 처리기 (참조 사용)**
```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn reverse_words(text: &str) -> String {
    text.split_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let text = "hello rust world";
    println!("Word count: {}", count_words(text));
    println!("Reversed: {}", reverse_words(text));
    println!("Original: {}", text);  // 여전히 사용 가능
}
```

**프로젝트 2: 가변 벡터 수정 (가변 참조)**
```rust
fn add_prefix(items: &mut Vec<String>, prefix: &str) {
    for item in items {
        *item = format!("{}{}", prefix, item);
    }
}

fn main() {
    let mut words = vec![
        String::from("apple"),
        String::from("banana"),
    ];
    add_prefix(&mut words, "fruit:");
    println!("{:?}", words);
    // ["fruit:apple", "fruit:banana"]
}
```

**프로젝트 3: HashMap의 참조와 소유권**
```rust
use std::collections::HashMap;

fn merge_maps(
    map1: &HashMap<String, i32>,
    map2: &HashMap<String, i32>,
) -> HashMap<String, i32> {
    let mut result = map1.clone();
    for (k, v) in map2 {
        *result.entry(k.clone()).or_insert(0) += v;
    }
    result
}
```

### 📖 학습 자료

**필독:**
- The Rust Book - Ch. 4 (Ownership), Ch. 10 (Lifetimes)
- Rust by Example - Scope, Borrowing, Lifetimes
- Rust Reference - Ownership and Moves

**고급 자료:**
- "Understanding Ownership" (Rust Book 상세 설명)
- Rustlings: ownership, borrowing exercises

### ⚠️ Level 2에서 가장 흔한 실수

| 실수 | 원인 | 해결책 |
|------|------|--------|
| Move 이해 안 됨 | C++의 얕은 복사와 헷갈림 | ownership을 여러 번 반복 학습 |
| 차용 규칙 위반 | 불변/가변 참조 동시 사용 | 컴파일 에러 메시지 정독 |
| 함수 파라미터 실수 | T vs &T vs &mut T 혼동 | 각 경우마다 소유권 변화 그려보기 |
| Lifetime annotation 과다 | 불필요한 lifetime 작성 | 컴파일러가 자동 추론하는 경우 이해 |

### ⏭️ Level 3으로 진행하기 전 확인

```
☐ Ownership의 3가지 규칙을 설명 가능
☐ Move와 Copy의 차이 명확히 이해
☐ 차용 규칙을 자신의 말로 설명 가능
☐ 컴파일러 에러 메시지를 읽고 해결 가능
☐ rustlings ownership & borrowing 100% 완료
☐ 자신이 작성한 코드의 소유권 흐름을 다이어그램으로 그릴 수 있음
☐ Lifetime annotation이 필요한 함수 정의 가능
```

---

## 🎯 Level 3: 고급 타입 시스템 (상급)
### 학습 시간: 3주 | 선행 조건: Level 2 완료

### 📚 학습 주제

**1. Enums & Pattern Matching**
- Enum 정의 (각 variant에 데이터 포함 가능)
- match 문법
- if let, while let
- 복잡한 패턴 분해

**2. Traits (특성)**
- Trait 정의
- Trait 구현 (impl Trait for Type)
- Default method
- Trait objects (dyn Trait)

**3. Generics (제네릭)**
- Generic 함수
- Generic 구조체와 열거형
- Generic 메서드
- Trait bounds

**4. Error Handling**
- Result<T, E>와 Option<T>
- ? operator
- Custom error types
- Error propagation

### ✅ Level 3 도달 기준 (모두 할 수 있어야 함)

**Enums & Pattern Matching:**
- [ ] Enum을 정의하고, 각 variant에 데이터 저장 가능
- [ ] match 문으로 모든 경우의 수 처리 가능
- [ ] 복잡한 패턴을 분해할 수 있음 (nested patterns)
- [ ] if let으로 특정 패턴만 처리 가능

**Traits:**
- [ ] Trait를 정의하고 구현 가능
- [ ] 여러 타입이 동일한 trait 구현 가능
- [ ] Trait bounds를 사용해 제네릭 제약 가능
- [ ] dyn Trait를 사용한 타입 추상화 가능

**Generics:**
- [ ] Generic 함수 작성 가능
- [ ] Generic 구조체 정의 가능
- [ ] Trait bounds와 함께 사용 가능
- [ ] 여러 Generic 파라미터 조합 가능

**Error Handling:**
- [ ] Result<T, E>와 Option<T>의 차이 명확히 이해
- [ ] ? operator를 사용해 에러 전파 가능
- [ ] 커스텀 에러 타입 정의 및 구현 가능
- [ ] 함수에서 여러 에러 타입 처리 가능

### 📋 Level 3 체크리스트

**1. Enum & Pattern Matching**
```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(s) => println!("Write: {}", s),
        Message::ChangeColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
    }
}

// 이것도 작성 가능?
fn extract_name(msg: &Message) -> Option<&str> {
    match msg {
        Message::Write(s) => Some(s),
        _ => None,
    }
}
```

**2. Traits**
```rust
trait Animal {
    fn speak(&self);
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
    fn name(&self) -> &str {
        &self.name
    }
}

// Generic과 함께 사용
fn make_sound<T: Animal>(animal: &T) {
    println!("{} says: ", animal.name());
    animal.speak();
}
```

**3. Error Handling**
```rust
use std::num::ParseIntError;

fn parse_and_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    match parse_and_add("5", "10") {
        Ok(result) => println!("Sum: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**4. Collections와 Iterators (심화)**
```rust
fn filter_and_transform(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&n| n > 0)
        .map(|n| n * n)
        .collect()
}

// HashMap과 함수형 스타일
use std::collections::HashMap;

fn group_by_parity(numbers: &[i32]) -> HashMap<&'static str, Vec<i32>> {
    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();
    for &n in numbers {
        if n % 2 == 0 {
            map.entry("even").or_insert_with(Vec::new).push(n);
        } else {
            map.entry("odd").or_insert_with(Vec::new).push(n);
        }
    }
    map
}
```

### 🎓 Level 3 실습 프로젝트

**프로젝트 1: 계산기 (Enum + Pattern Matching)**
```rust
enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

fn calculate(op: Operation) -> Result<i32, String> {
    match op {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
}
```

**프로젝트 2: 데이터 구조 (Traits + Generics)**
```rust
trait Storage<T> {
    fn add(&mut self, item: T);
    fn get(&self, index: usize) -> Option<&T>;
    fn len(&self) -> usize;
}

struct ListStorage<T> {
    items: Vec<T>,
}

impl<T> Storage<T> for ListStorage<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    fn len(&self) -> usize {
        self.items.len()
    }
}
```

**프로젝트 3: 설정 파서 (Result + Error Handling)**
```rust
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u32,
}

fn parse_config(input: &str) -> Result<Config, String> {
    let mut config = Config {
        host: String::new(),
        port: 8080,
        timeout: 30,
    };
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid line: {}", line));
        }
        
        match parts[0].trim() {
            "host" => config.host = parts[1].trim().to_string(),
            "port" => config.port = parts[1].trim().parse()?,
            "timeout" => config.timeout = parts[1].trim().parse()?,
            key => return Err(format!("Unknown key: {}", key)),
        }
    }
    
    Ok(config)
}
```

### 📖 학습 자료

**필독:**
- The Rust Book - Ch. 6 (Enums), Ch. 10 (Traits, Generics), Ch. 9 (Error Handling)
- Rust by Example - Pattern Matching, Traits, Generics

### ⏭️ Level 4로 진행하기 전 확인

```
☐ Enum을 정의하고 pattern matching으로 처리 가능
☐ Trait를 정의하고 구현 가능
☐ Generic 함수와 구조체 작성 가능
☐ Result와 Option을 적절히 사용 가능
☐ ? operator를 사용해 에러 처리 가능
☐ 자신이 작성한 코드의 타입 관계를 설명 가능
```

---

## 🎯 Level 4: 실무 능력 (고급)
### 학습 시간: 4주 | 선행 조건: Level 3 완료

### 📚 학습 주제

**1. 모듈 시스템**
- mod 선언
- pub/private
- use 임포트
- 크레이트 구조

**2. Async/Await**
- Future trait
- async/await 문법
- tokio 런타임
- 에러 처리

**3. 멀티스레딩**
- std::thread
- Mutex와 Arc
- 채널 (mpsc)
- 스레드 간 통신

**4. 테스트와 벤치마킹**
- Unit tests
- Integration tests
- Test organization
- Benchmarks

### ✅ Level 4 도달 기준 (모두 할 수 있어야 함)

**모듈 시스템:**
- [ ] 프로젝트를 여러 모듈로 구성 가능
- [ ] pub/private 키워드로 가시성 제어 가능
- [ ] 다른 모듈에서 함수/구조체 임포트 가능
- [ ] 실제 프로젝트 규모의 구조 설계 가능

**Async/Await:**
- [ ] async 함수 작성 가능
- [ ] .await로 Future 기다리기 가능
- [ ] tokio를 사용한 비동기 프로그램 작성 가능
- [ ] 에러 처리와 timeout 구현 가능

**멀티스레딩:**
- [ ] std::thread로 스레드 생성 가능
- [ ] Mutex로 공유 자원 보호 가능
- [ ] Arc로 여러 스레드가 데이터 소유 가능
- [ ] mpsc 채널로 스레드 간 통신 가능

**테스트:**
- [ ] Unit test 작성 및 실행 가능
- [ ] #[test] 매크로 사용 가능
- [ ] assert! 및 assert_eq! 활용 가능
- [ ] 테스트 조직화 가능

### 📋 Level 4 체크리스트

**1. 모듈 시스템**
```rust
// src/main.rs
mod math;
mod string_utils;

use math::add;
use string_utils::to_uppercase;

fn main() {
    println!("{}", add(5, 3));
    println!("{}", to_uppercase("hello"));
}

// src/math.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn multiply(a: i32, b: i32) -> i32 { a * b }

// src/string_utils.rs
pub fn to_uppercase(s: &str) -> String { s.to_uppercase() }
```

**2. Async/Await**
```rust
use tokio::time::{sleep, Duration};

async fn fetch_data(url: &str) -> Result<String, String> {
    sleep(Duration::from_secs(1)).await;
    Ok(format!("Data from {}", url))
}

#[tokio::main]
async fn main() {
    match fetch_data("https://example.com").await {
        Ok(data) => println!("{}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**3. 멀티스레딩**
```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}
```

**4. 테스트**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_divide_by_zero() {
        divide(5, 0);
    }
}
```

### 🎓 Level 4 실습 프로젝트

**프로젝트 1: 웹 서버 (Async + Tokio)**
```rust
use tokio::net::TcpListener;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080");
    
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            // 클라이언트 처리
        });
    }
}
```

**프로젝트 2: 병렬 데이터 처리 (Multithreading + Channels)**
```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    for i in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(process_data(i)).unwrap();
        });
    }
    drop(tx);
    
    for result in rx {
        println!("Processed: {}", result);
    }
}

fn process_data(n: i32) -> i32 {
    n * n
}
```

**프로젝트 3: 모듈화된 애플리케이션**
```
src/
  main.rs
  config/
    mod.rs
    loader.rs
    validator.rs
  database/
    mod.rs
    connection.rs
    query.rs
  api/
    mod.rs
    routes.rs
    handlers.rs
```

### 📖 학습 자료

**필독:**
- The Rust Book - Ch. 7 (Modules), Ch. 16 (Concurrency), Ch. 17 (OOP)
- Tokio Tutorial (https://tokio.rs/tokio/tutorial)
- The Async Rust Book

### ⏭️ Level 5로 진행하기 전 확인

```
☐ 프로젝트를 모듈로 구조화 가능
☐ async/await 사용해 비동기 코드 작성 가능
☐ 멀티스레드 프로그램 작성 가능
☐ 단위 테스트와 통합 테스트 작성 가능
☐ 실제 프로젝트 규모의 코드 조직화 가능
☐ tokio 또는 유사 런타임 사용 경험 있음
```

---

## 🎯 Level 5: 전문가 (고급)
### 학습 시간: 계속 진행 | 선행 조건: Level 4 완료

### 📚 학습 주제

**1. Unsafe Rust**
- unsafe 블록
- Raw pointers
- 안전성 보장 책임
- FFI (Foreign Function Interface)

**2. 고급 타입 시스템**
- Advanced trait patterns
- Higher-ranked trait bounds
- Associated types
- Type-level programming

**3. 성능 최적화**
- Profiling과 benchmarking
- 메모리 레이아웃 최적화
- SIMD
- 컴파일 시간 최적화

**4. 도메인 특화 개발**
- 웹 프레임워크 (Actix, Axum)
- 게임 개발 (Bevy)
- 임베디드 시스템 (embedded-hal)
- 자동차 소프트웨어 (ROS2, automotive crates)

**5. 고급 패턴**
- Builder pattern
- Type-state pattern
- RAII pattern 활용
- 매크로 작성

### ✅ Level 5 도달 기준 (전문가 수준)

**깊은 이해:**
- [ ] Unsafe Rust를 올바르게 사용할 수 있음
- [ ] Raw pointer 조작 가능
- [ ] C/C++ 라이브러리와 FFI로 연동 가능
- [ ] Advanced trait patterns 이해 및 활용

**성능:**
- [ ] Profiling 도구로 병목 지점 파악 가능
- [ ] Benchmarking 코드 작성 및 최적화 가능
- [ ] 메모리 레이아웃을 고려한 데이터 구조 설계 가능
- [ ] 실제 성능 개선 경험

**프로덕션 코드:**
- [ ] 대규모 프로젝트 아키텍처 설계 가능
- [ ] 오픈소스 라이브러리 활용 또는 기여 가능
- [ ] 에러 처리와 logging 제대로 구현 가능
- [ ] 문서화와 테스트 체계적으로 작성

**도메인 특화:**
- [ ] 자동차 소프트웨어: ROS2, SOME/IP, DDS 경험
- [ ] 임베디드: STM32, ARM 마이크로컨트롤러 프로그래밍
- [ ] 웹: 웹 프레임워크로 실제 서버 개발
- [ ] 게임: Bevy로 게임 프로토타입 구현

### 📋 Level 5 체크리스트

**1. Unsafe & FFI**
```rust
// C 함수와 연동
extern "C" {
    fn strlen(s: *const u8) -> usize;
}

fn measure_cstring(s: &std::ffi::CStr) -> usize {
    unsafe {
        strlen(s.as_ptr() as *const u8)
    }
}

// Raw pointer 조작
fn swap<T>(a: *mut T, b: *mut T) {
    unsafe {
        std::ptr::swap(a, b);
    }
}
```

**2. Advanced Patterns**
```rust
// Builder pattern
struct ServerBuilder {
    host: String,
    port: u16,
    timeout: Duration,
}

impl ServerBuilder {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            port: 8080,
            timeout: Duration::from_secs(30),
        }
    }
    
    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    fn build(self) -> Server {
        Server {
            host: self.host,
            port: self.port,
            timeout: self.timeout,
        }
    }
}

// 사용
let server = ServerBuilder::new("127.0.0.1")
    .port(9000)
    .build();
```

**3. ROS2/자동차 소프트웨어 특화**
```rust
// ROS2 노드 작성
use rclrs::prelude::*;

fn main() -> rclrs::RclrsResult<()> {
    let context = rclrs::init()?;
    let node = rclrs::create_node(&context, "my_node")?;
    
    // 퍼블리셔 생성
    let publisher = node.create_publisher::<std_msgs::msg::String>("topic", QosProfile::default())?;
    
    // 메시지 발행
    let msg = std_msgs::msg::String {
        data: "Hello, ROS2".to_string(),
    };
    publisher.publish(&msg)?;
    
    Ok(())
}
```

**4. 성능 최적화 (Benchmarking)**
```rust
#![feature(test)]
extern crate test;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;
    
    #[bench]
    fn bench_fibonacci(b: &mut Bencher) {
        b.iter(|| fibonacci(20));
    }
}
```

### 🎓 Level 5 실습 프로젝트

**프로젝트 1: 오픈소스 라이브러리**
- Crates.io에 공개 가능한 수준의 라이브러리
- 완전한 문서화와 테스트
- 예: JSON 파서, 이미지 프로세싱, 데이터 구조 라이브러리

**프로젝트 2: 실무 규모의 애플리케이션**
- REST API 서버 (Actix 또는 Axum)
- 데이터베이스 연동 (SQLx, Diesel)
- 캐싱, 로깅, 모니터링
- Docker 배포

**프로젝트 3: 자동차 미들웨어 (도그염용)**
```rust
// ROS2 기반 센서 데이터 처리
use rclrs::prelude::*;
use std::sync::Arc;

struct SensorNode {
    lidar_subscriber: Arc<rclrs::Subscription>,
    radar_subscriber: Arc<rclrs::Subscription>,
    fusion_publisher: Arc<rclrs::Publisher>,
}

impl SensorNode {
    fn new(node: &rclrs::Node) -> rclrs::RclrsResult<Self> {
        // 센서 데이터 구독
        let lidar_sub = node.create_subscription(
            "lidar/data",
            QosProfile::default(),
        )?;
        
        // 퓨전 결과 발행
        let fusion_pub = node.create_publisher(
            "sensor_fusion/output",
            QosProfile::default(),
        )?;
        
        Ok(Self {
            lidar_subscriber: Arc::new(lidar_sub),
            radar_subscriber: Arc::new(radar_sub),
            fusion_publisher: Arc::new(fusion_pub),
        })
    }
}
```

**프로젝트 4: 고성능 데이터 처리**
- SIMD 최적화를 사용한 벡터 연산
- Parallel processing with rayon
- 메모리 효율적인 데이터 구조
- 1초에 수백만 항목 처리

### 📖 학습 자료

**고급 서적:**
- The Rustonomicon (Unsafe Rust)
- Advanced Rust (Advanced Patterns)
- Rust API Guidelines

**특화 가이드:**
- ROS2 Rust Client (https://github.com/ros2-rust/ros2_rust)
- Embedded Rust Book
- WebAssembly with Rust

### ✨ Level 5 지속 성장

Level 5는 끝나는 지점이 아니라 **시작 지점**입니다.

```
지속적인 학습:
├─ 오픈소스 프로젝트 기여
├─ 블로그/글 작성으로 지식 정리
├─ 커뮤니티 활동 (Rust forums, meetups)
├─ 새로운 라이브러리 학습
├─ RustConf 등 컨퍼런스 참석
└─ 자신만의 라이브러리/도구 개발
```

---

## 📊 전체 로드맵 요약표

| Level | 주요 내용 | 학습 시간 | 도달 기준 | 프로젝트 예시 |
|-------|---------|---------|---------|-----------|
| **1** | 기초 문법 | 2주 | rustlings 초급 완료 | CLI 도구, 계산기 |
| **2** | 메모리 관리 | 3주 | Ownership & Borrowing 마스터 | 텍스트 처리, 참조 활용 |
| **3** | 타입 시스템 | 3주 | Traits, Generics, Error Handling | 설정 파서, 데이터 구조 |
| **4** | 실무 능력 | 4주 | 모듈화, Async, 멀티스레딩 | 웹 서버, 병렬 처리 |
| **5** | 전문가 | 지속 | FFI, 성능 최적화, 도메인 특화 | 오픈소스, ROS2 통합 |

---

## 🚀 도그염의 맞춤 로드맵

현재 상황: **Level 1.5** (기초 완료, Level 2 진행 중)
- 10년 C/C++ 경력 → 문법은 빨리 진행 가능
- 42dot 면접 준비 → Rust 기초 이미 학습함
- 자동차 소프트웨어 경력 → Level 4-5 목표 설정

### 추천 일정

```
2024년 6월-7월: Level 2 완료 (3주)
  → Ownership & Borrowing 마스터
  → rustlings 100% 완료
  
2024년 7월-8월: Level 3 완료 (3주)
  → Traits, Generics, Pattern Matching
  → 설정 파서 프로젝트
  
2024년 8월-9월: Level 4 진행 (4주)
  → Async/Await, 모듈 시스템
  → 간단한 Tokio 기반 프로젝트
  
2024년 9월-: Level 5 시작
  → ROS2 Rust 학습
  → 자동차 미들웨어 프로토타입
  → 오픈소스 기여
```

### 42dot 입사 전 달성 목표

```
✓ Level 2 완료 (최소)
✓ Level 3 기초 (Pattern Matching, Error Handling)
☐ Level 4 입문 (Async 기초)
☐ ROS2 Rust 파이로 기초 학습

입사 후:
└─ Level 4-5를 회사 프로젝트로 실전 습득
```

---

## 🎯 최종 체크리스트

### Level 1-2 정리 후 (4주 완료)

```
기초 능력:
☐ cargo와 rustup 능숙하게 사용
☐ Ownership의 3 규칙을 자신의 말로 설명 가능
☐ 소유권 위반 에러를 읽고 해결 가능
☐ 간단한 프로그램 작성 가능 (100-200줄)

실무 준비:
☐ rustlings 50% 이상 완료
☐ 자신의 코드의 메모리 흐름 파악 가능
☐ 참조와 소유권을 올바르게 사용
☐ 기본 에러 처리 가능
```

### Level 3-4 정리 후 (10주 완료)

```
고급 능력:
☐ Traits와 Generics 자유롭게 사용
☐ Pattern matching으로 복잡한 데이터 처리
☐ Result와 Option 마스터
☐ 모듈 시스템으로 프로젝트 구조화

프로덕션 준비:
☐ Async/Await 기본 이해
☐ 멀티스레드 프로그래밍 가능
☐ 단위 테스트 작성
☐ 500-1000줄 규모의 프로젝트 완성
```

---

**이 로드맵을 따라가면서 각 Level을 마스터하세요!** 🚀
