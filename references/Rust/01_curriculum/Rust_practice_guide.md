# Rust 실습 가이드 & C++ 비교

## 🔄 C++ 개발자를 위한 Rust 전환 가이드

### 1. 메모리 관리 패러다임 비교

#### C++
```cpp
// 수동 메모리 관리
int* p = new int(5);
delete p;  // 개발자가 담당

// RAII 사용 (현대 C++)
std::unique_ptr<int> p(new int(5));
// p가 스코프를 벗어나면 자동으로 delete
```

#### Rust
```rust
// 자동 메모리 관리 (컴파일 타임)
let p = Box::new(5);
// p가 스코프를 벗어나면 자동으로 drop()
// 개발자는 소유권 규칙만 따르면 됨
```

**핵심:** Rust는 C++의 RAII를 자동으로 강제하는 언어

---

### 2. 포인터와 참조 비교

#### C++
```cpp
void increment(int* p) {  // 포인터는 null일 수 있음
    if (p != nullptr) {
        (*p)++;
    }
}

int main() {
    int x = 5;
    increment(&x);
}
```

#### Rust
```rust
fn increment(p: &mut i32) {  // 참조는 절대 null이 될 수 없음
    *p += 1;
}

fn main() {
    let mut x = 5;
    increment(&mut x);
}
```

**핵심:** Rust 참조는 null이 아니라는 것을 컴파일러가 보장

---

### 3. 소유권 (가장 중요!)

#### C++ 개발자의 실수
```cpp
std::string s = "hello";
std::string s2 = s;  // 깊은 복사, 상관없음
std::cout << s;      // OK - s는 여전히 유효
```

#### Rust (Move가 일어남)
```rust
let s = String::from("hello");
let s2 = s;  // Move! s의 소유권이 s2로 이동
println!("{}", s);  // ❌ 컴파일 에러! s는 더 이상 유효하지 않음
```

**실습:** Move 이해하기
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1의 소유권이 s2로 이동
    
    // println!("{}", s1);  // 에러!
    println!("{}", s2);      // OK
    
    // Copy를 하려면 명시적으로 clone
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("{}, {}", s3, s4);  // 둘 다 OK
}
```

---

## 💪 세션별 실전 연습문제

### 세션 1 연습: 기본 타입과 문자열

**문제 1-1: 정수와 실수 다루기**
```rust
fn main() {
    // TODO: 정수 x = 42, 실수 y = 3.14 선언
    // x를 f64로 변환하고 y와 더한 결과 출력
    
    let x: i32 = 42;
    let y: f64 = 3.14;
    let result = (x as f64) + y;
    println!("Result: {}", result);
}
```

**문제 1-2: String vs &str**
```rust
fn main() {
    let s1: String = String::from("hello");
    let s2: &str = "world";
    
    // TODO: s1과 s2의 타입 차이 설명하기
    // s1: String (힙에 할당, 소유권 있음, 가변 가능)
    // s2: &str (슬라이스, 참조, 변경 불가)
    
    println!("s1 type: String, s2 type: &str");
}
```

**풀이:**
```rust
fn demonstrate_string_types() {
    let owned: String = String::from("I am owned");
    let borrowed: &str = "I am borrowed";
    
    println!("Owned: {}", owned);
    println!("Borrowed: {}", borrowed);
    
    // String을 &str로 변환 가능
    let as_str: &str = &owned;
    println!("Owned as slice: {}", as_str);
}
```

---

### 세션 2-3 연습: Ownership & Borrowing

**문제 2-1: Move 추적하기**
```rust
fn main() {
    let v = vec![1, 2, 3];
    takes_ownership(v);  // v의 소유권이 함수로 이동
    
    // println!("{:?}", v);  // 에러! v는 더 이상 유효하지 않음
}

fn takes_ownership(v: Vec<i32>) {
    println!("Vector: {:?}", v);
    // v가 함수를 빠져나올 때 drop됨
}
```

**문제 2-2: Borrowing으로 해결하기**
```rust
fn main() {
    let v = vec![1, 2, 3];
    borrows_vector(&v);  // v의 참조만 전달
    println!("{:?}", v);  // OK! v는 여전히 유효
}

fn borrows_vector(v: &Vec<i32>) {
    println!("Vector: {:?}", v);
}
```

**문제 2-3: 가변 참조**
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    add_element(&mut v, 4);
    println!("{:?}", v);  // [1, 2, 3, 4]
}

fn add_element(v: &mut Vec<i32>, x: i32) {
    v.push(x);
}
```

**문제 2-4: 차용 규칙 위반 찾기**
```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // 불변 참조
    let r2 = &s;      // OK - 또 다른 불변 참조
    // let r3 = &mut s;  // ❌ 에러! 불변과 가변 참조 동시 불가
    
    println!("{}, {}", r1, r2);
    
    let r3 = &mut s;  // OK - r1, r2가 더 이상 사용되지 않음
    r3.push_str(" world");
    println!("{}", r3);
}
```

---

### 세션 4 연습: Pattern Matching

**문제 4-1: Option 매칭**
```rust
fn main() {
    let numbers = vec![Some(5), None, Some(10)];
    
    for num in numbers {
        match num {
            Some(n) => println!("Value: {}", n),
            None => println!("No value"),
        }
    }
}
```

**문제 4-2: Enum 정의 및 매칭**
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn process_light(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Caution!"),
        TrafficLight::Green => println!("Go!"),
    }
}

fn main() {
    let light = TrafficLight::Green;
    process_light(light);
}
```

**문제 4-3: 복잡한 패턴 매칭**
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
    }
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    process(msg);
}
```

---

### 세션 5 연습: Error Handling

**문제 5-1: Result 기본**
```rust
use std::num::ParseIntError;

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

fn main() {
    match parse_number("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
```

**문제 5-2: ? 연산자**
```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("test.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**문제 5-3: 커스텀 에러 타입**
```rust
use std::fmt;

#[derive(Debug)]
enum MyError {
    InvalidInput(String),
    IoError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            MyError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

fn validate_age(age: i32) -> Result<(), MyError> {
    if age < 0 || age > 150 {
        Err(MyError::InvalidInput(format!("Age out of range: {}", age)))
    } else {
        Ok(())
    }
}
```

---

### 세션 6-7 연습: Traits & Generics

**문제 6-1: Generic 함수**
```rust
fn find_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", find_max(5, 10));           // 10
    println!("{}", find_max(3.14, 2.71));      // 3.14
    println!("{}", find_max("hello", "world")); // "world"
}
```

**문제 6-2: Trait 정의**
```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * 3.14159 * self.radius
    }
}

fn print_shape_info<S: Shape>(shape: &S) {
    println!("Area: {}, Perimeter: {}", shape.area(), shape.perimeter());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    print_shape_info(&circle);
}
```

**문제 6-3: Lifetime이 필요한 경우**
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = "short";
    
    let result = longest(&s1, s2);
    println!("Longest: {}", result);
}

// Lifetime 없이는 컴파일러가 반환 참조의 유효 범위를 알 수 없음
```

---

### 세션 8 연습: Collections & Iterators

**문제 8-1: Vec와 Iterator**
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // iter() - 불변 참조
    for x in v.iter() {
        println!("{}", x);
    }
    
    // into_iter() - 소유권 이동
    let v2 = vec![1, 2, 3];
    for x in v2 {  // into_iter()가 암묵적으로 호출됨
        println!("{}", x);
    }
    // v2는 더 이상 사용 불가
}
```

**문제 8-2: Iterator Adapter 체이닝**
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let result: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // 짝수만
        .map(|x| x * x)             // 제곱
        .sum();                      // 합계
    
    println!("Sum of squared even numbers: {}", result);  // 4 + 16 = 20
}
```

**문제 8-3: HashMap 실습**
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    
    // 업데이트
    scores.entry("Alice").or_insert(0);
    if let Some(score) = scores.get_mut("Alice") {
        *score += 5;
    }
}
```

---

### 세션 9 연습: Async/Threading

**문제 9-1: 스레드 생성**
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 1..3 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    
    handle.join().unwrap();  // 스레드가 종료될 때까지 대기
}
```

**문제 9-2: 채널 통신**
```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let message = "Hello from thread";
        tx.send(message).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
```

**문제 9-3: 여러 producer**
```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // 여러 스레드에서 메시지 전송
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(format!("Message {}", i)).unwrap();
        });
    }
    drop(tx);  // 모든 sender가 드롭되어야 recv가 끝남
    
    for received in rx {
        println!("{}", received);
    }
}
```

---

### 세션 10 연습: 소규모 프로젝트

**프로젝트: 간단한 설정 파일 파서**

```rust
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
struct Config {
    settings: HashMap<String, String>,
}

impl Config {
    fn new() -> Self {
        Config {
            settings: HashMap::new(),
        }
    }
    
    fn parse(content: &str) -> Result<Self, String> {
        let mut config = Config::new();
        
        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            
            // 주석 및 빈 줄 무시
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            
            // key=value 파싱
            let parts: Vec<&str> = trimmed.split('=').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid format at line {}: {}", line_num + 1, line));
            }
            
            let key = parts[0].trim();
            let value = parts[1].trim();
            
            if key.is_empty() || value.is_empty() {
                return Err(format!("Empty key or value at line {}", line_num + 1));
            }
            
            config.settings.insert(key.to_string(), value.to_string());
        }
        
        Ok(config)
    }
    
    fn get(&self, key: &str) -> Option<&str> {
        self.settings.get(key).map(|s| s.as_str())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = "
# Database configuration
host=localhost
port=5432
user=admin

# App settings
debug=true
timeout=30
    ";
    
    let config = Config::parse(content)?;
    println!("Config: {:?}", config);
    println!("Host: {:?}", config.get("host"));
    println!("Port: {:?}", config.get("port"));
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_valid_config() {
        let content = "host=localhost\nport=5432";
        let config = Config::parse(content).unwrap();
        
        assert_eq!(config.get("host"), Some("localhost"));
        assert_eq!(config.get("port"), Some("5432"));
    }
    
    #[test]
    fn test_parse_ignores_comments() {
        let content = "# comment\nhost=localhost";
        let config = Config::parse(content).unwrap();
        
        assert_eq!(config.settings.len(), 1);
    }
    
    #[test]
    fn test_parse_invalid_format() {
        let content = "invalid line";
        assert!(Config::parse(content).is_err());
    }
}
```

---

## 📖 매일 해야 할 일 (Daily Practice)

### 매 세션 후
1. **코드 한 줄씩 입력 (copy-paste 금지)**
2. **컴파일 에러 직면하면 메시지 읽기**
3. **변수명 바꿔보기 - 예) x → count, y → total**
4. **주석 달기 - 각 라인이 하는 일 설명**

### 주말 복습
```rust
// 주간 학습 내용을 한 파일에 통합하는 연습
fn weekly_summary() {
    // 배운 개념들을 하나의 파일에서 모두 사용
    // 예: ownership + traits + error handling 조합
}
```

---

## 🐛 흔한 실수와 해결책

### 실수 1: Copy trait 혼동
```rust
// ❌ 잘못된 가정
let s1 = String::from("hello");
let s2 = s1;
println!("{}, {}", s1, s2);  // 에러!

// ✅ 해결책 1: clone
let s2 = s1.clone();

// ✅ 해결책 2: 참조 사용
let s2 = &s1;
```

### 실수 2: 라이프타임 애노테이션 누락
```rust
// ❌ 컴파일 안 됨
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// ✅ 라이프타임 추가
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 실수 3: 차용 규칙 위반
```rust
// ❌ 불변과 가변 참조 동시 사용
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;  // 에러!

// ✅ 순서 조정
let mut s = String::from("hello");
let r1 = &s;
println!("{}", r1);
let r2 = &mut s;  // OK - r1이 더 이상 사용되지 않음
r2.push_str(" world");
```

---

## 🎓 학습 완료 후 다음 단계

### 단계 1: 실무 코드 읽기 (1주)
- GitHub의 실제 Rust 프로젝트
- 추천: `ripgrep`, `tokio`, `serde`

### 단계 2: 미니 프로젝트 (2주)
- HTTP 서버 (hyper 사용)
- 파일 시스템 도구
- JSON 파서/생성기

### 단계 3: ROS2/자동차 소프트웨어 응용 (진행중)
- rclrs 튜토리얼
- SOME/IP, DDS 학습
- 기존 C++ 미들웨어와 통합

---

**Remember: Rust의 컴파일러는 당신의 적이 아니라 최고의 선생님입니다! 🦀**
