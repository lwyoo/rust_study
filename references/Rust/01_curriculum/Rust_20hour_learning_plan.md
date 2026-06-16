# Rust 20시간 집중 학습 계획
## 파레토 원칙 적용 (80/20) - 핵심 20%에 집중

**학습 대상:** 10년 경력 C/C++ 개발자  
**목표 기간:** 10개 세션 × 2시간 (총 20시간)  
**세션 구조:** 1시간 45분 학습 + 15분 복습  
**수준:** 실무 적용 가능한 기초~중급

---

## 📊 학습 가중치
- **Ownership & Borrowing:** 25% (가장 중요, C/C++과의 가장 큰 차이)
- **Pattern Matching & Enums:** 15%
- **Error Handling:** 15%
- **Traits & Generics:** 20%
- **Async/Await & 실무:** 15%
- **기타 (모듈, 테스트, 도구):** 10%

---

## 🎯 세션별 학습 계획

### **세션 1: Rust 기초 & 변수, 타입 시스템**
**1시간 45분 학습**
- Rust 철학: 메모리 안전성, 속도, 병렬성
- 설치 & 개발 환경 (rustup, cargo)
- 기본 문법: 변수, 상수, 스코핑
- 스칼라 타입: i32, u32, f64, bool, char
- 복합 타입: tuple, array
- 문자열: String vs &str (핵심!)
- 함수 정의와 반환값

**실습:**
```rust
fn main() {
    let s: String = String::from("hello");
    let s_ref: &str = &s;
    println!("{}, {}", s, s_ref);
}
```

**15분 복습 체크리스트**
- [ ] 변수 선언과 mut의 차이 이해
- [ ] String과 &str의 차이 설명 가능
- [ ] 함수 시그니처 작성 가능

---

### **세션 2: Ownership & Borrowing (Part 1)**
**1시간 45분 학습**

⭐ **가장 중요한 세션 - 반드시 깊이 있게**

- Ownership 규칙 3가지
  - 각 값은 하나의 owner를 가짐
  - owner가 스코프를 벗어나면 드롭됨
  - 값을 move할 수 있음
- Move semantics (C++의 std::move와 유사하지만 자동)
- Copy trait vs Move
- 문자열과 벡터의 ownership

**중요 비교:**
```rust
// Move
let s1 = String::from("hello");
let s2 = s1;  // s1은 더 이상 유효하지 않음
// println!("{}", s1);  // 에러!

// Copy (기본 타입)
let x = 5;
let y = x;  // x는 여전히 유효 (Copy trait)
println!("{}", x);  // OK
```

**실습:**
- Ownership과 move 실습
- 컴파일러 에러 메시지 읽기 (매우 친절함!)

**15분 복습 체크리스트**
- [ ] Move의 정의 설명 가능
- [ ] Copy trait 이해
- [ ] 컴파일러 에러 해석 가능

---

### **세션 3: Borrowing & References (Part 2)**
**1시간 45분 학습**

⭐ **필수 개념 - Ownership 다음 중요**

- 불변 참조 (&T)와 가변 참조 (&mut T)
- 차용 규칙 (Borrowing Rules)
  - 여러 불변 참조는 가능
  - 하나의 가변 참조만 가능
  - 불변과 가변 참조는 동시에 불가
- Dangling reference 방지
- 함수 파라미터와 reference

**핵심 예제:**
```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // 불변 참조
    let r2 = &s;      // 또 다른 불변 참조 - OK
    // let r3 = &mut s;  // 에러! 가변 참조는 불가
    
    println!("{}, {}", r1, r2);
    
    let r3 = &mut s;  // r1, r2가 더 이상 사용되지 않으므로 OK
    r3.push_str(" world");
    println!("{}", r3);
}
```

**실습:**
- 다양한 참조 조합 실습
- 컴파일러 에러 해결 연습

**15분 복습 체크리스트**
- [ ] 차용 규칙 3가지 설명 가능
- [ ] 불변과 가변 참조의 차이 명확
- [ ] Dangling reference 이해

---

### **세션 4: Pattern Matching & Enums**
**1시간 45분 학습**

- Enum 정의 및 사용 (C의 enum과 완전히 다름!)
- Enum의 각 variant에 데이터 포함 가능
- match 문법 (매우 강력)
- if let, while let (편의 문법)

**실습 예제:**
```rust
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
        Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
    }
}
```

**실습:**
- Enum 정의 및 match 패턴 매칭
- 복잡한 패턴 분해

**15분 복습 체크리스트**
- [ ] Enum과 match의 강력함 이해
- [ ] 패턴 분해 가능
- [ ] if let 편의 문법 이해

---

### **세션 5: Error Handling (Result & Option)**
**1시간 45분 학습**

- Option<T> (값이 있거나 없거나)
- Result<T, E> (성공 또는 에러)
- ? operator (에러 전파)
- unwrap vs expect vs match

**실무 패턴:**
```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;  // ? operator
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 사용
match read_file("test.txt") {
    Ok(data) => println!("Data: {}", data),
    Err(e) => eprintln!("Error: {}", e),
}
```

**실습:**
- 파일 읽기/쓰기 에러 처리
- ? operator 활용
- 에러 타입 정의

**15분 복습 체크리스트**
- [ ] Option과 Result의 차이 명확
- [ ] ? operator 사용 가능
- [ ] 에러 처리 패턴 습득

---

### **세션 6: Traits & Generics (Part 1)**
**1시간 45분 학습**

- Generic 타입 (C++의 template과 유사)
- Trait 정의 (인터페이스 개념)
- Trait implementation
- Trait bounds

**실습 예제:**
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

fn print_animal_info<T: Animal>(animal: &T) {
    println!("{} says: ", animal.name());
    animal.speak();
}
```

**실습:**
- Generic 함수 작성
- Trait 구현
- Trait bounds 적용

**15분 복습 체크리스트**
- [ ] Generic 문법 이해
- [ ] Trait 정의 및 구현 가능
- [ ] Trait bounds 이해

---

### **세션 7: Traits & Generics (Part 2) + Lifetime**
**1시간 45분 학습**

- Lifetime 기본 개념 (참조의 유효 범위)
- Lifetime annotation 문법
- 함수의 lifetime
- 구조체의 lifetime

**필수 이해:**
```rust
// lifetime이 명시적으로 필요한 경우
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Article<'a> {
    title: &'a str,
    content: &'a str,
}
```

**실습:**
- 함수의 lifetime 추론
- 구조체 lifetime
- 컴파일러와 협력하기

**15분 복습 체크리스트**
- [ ] Lifetime의 목적 이해
- [ ] 기본 lifetime 문법 가능
- [ ] 함수 signature에 lifetime 작성 가능

---

### **세션 8: Collections & Iterators**
**1시간 45분 학습**

- Vec<T> (동적 배열) 심화
- HashMap & HashSet
- Iterator trait와 adapter methods
- map, filter, fold 등 함수형 메서드

**실습:**
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Iterator와 closure
    let sum: i32 = v.iter()
        .filter(|&&x| x > 2)
        .map(|x| x * 2)
        .sum();
    
    println!("Sum: {}", sum);  // 22
}
```

**실습:**
- 컬렉션 조작
- Iterator adapter 체이닝
- Closure와 함수형 스타일

**15분 복습 체크리스트**
- [ ] Vec, HashMap 사용 가능
- [ ] Iterator adapter 이해
- [ ] Closure 문법 습득

---

### **세션 9: Async/Await & 기본 멀티스레딩**
**1시간 45분 학습**

⭐ **실무에서 매우 중요 (ROS2, 네트워크 프로그래밍)**

- async/await 기본
- Future trait 이해
- tokio 런타임 기초
- 스레드 생성 (std::thread)
- 채널 (mpsc) - 스레드 간 통신

**실습:**
```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send("Hello from thread").unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("{}", received);
}
```

**실습:**
- 간단한 async 함수
- tokio 기본 사용
- 스레드 생성 및 채널 통신

**15분 복습 체크리스트**
- [ ] async/await 기본 이해
- [ ] 스레드 생성 가능
- [ ] 채널을 통한 통신 이해

---

### **세션 10: 모듈 체계 & 실전 프로젝트 + 종합 복습**
**1시간 45분 학습**

- 모듈 시스템 (mod, pub)
- 크레이트와 패키지 구조
- Cargo.toml 설정
- 테스트 작성 기초

**소규모 실전 프로젝트 (세션 10)**
JSON 파서 또는 간단한 CLI 도구:
```rust
use std::collections::HashMap;

fn parse_config(config: &str) -> Result<HashMap<String, String>, String> {
    let mut map = HashMap::new();
    for line in config.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err("Invalid format".to_string());
        }
        map.insert(parts[0].to_string(), parts[1].to_string());
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse() {
        let config = "key=value\nkey2=value2";
        let result = parse_config(config).unwrap();
        assert_eq!(result.get("key").map(|s| s.as_str()), Some("value"));
    }
}
```

**실습:**
- 모듈 구성
- 간단한 프로젝트 구조화
- 단위 테스트

**15분 복습 체크리스트**
- [ ] 모듈 시스템 이해
- [ ] 프로젝트 구조 가능
- [ ] 기본 테스트 작성 가능
- [ ] 지난 9개 세션 핵심 정리

---

## 📚 추천 학습 자료

### 최우선 (필독)
1. **The Rust Book** (공식)
   - https://doc.rust-lang.org/book/
   - 세션 1-5 해당: Ch. 3-6, 8-9

2. **Rust by Example**
   - https://doc.rust-lang.org/rust-by-example/

### 보조 자료
- **Rust Playground**: https://play.rust-lang.org/ (온라인 실습)
- **rustlings**: 소규모 연습 문제 (공식)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://win.rustup.rs | sh
  rustup component add rustfmt clippy
  ```

### 심화 (나중에)
- **Async Rust** (tokio 문서)
- **The Nomicon** (고급 메모리 안전성)

---

## 🎯 학습 전략

### 매 세션마다
1. **개념 학습** (50분)
   - 핵심 개념 읽기
   - 예제 코드 한 줄씩 타이핑하며 실행
   
2. **실습** (40분)
   - 주어진 연습 문제 풀이
   - 컴파일 에러 메시지를 "친구"처럼 읽기
   - 변형 코드 작성해보기

3. **복습** (15분)
   - 체크리스트 확인
   - 핵심 개념 1-2문장으로 요약
   - 다음 세션 미리보기

### 학습 팁
- **컴파일러를 믿어라**: Rust 컴파일러의 에러 메시지는 매우 명확하고 도움이 된다
- **한 번에 이해하려 하지 말 것**: 특히 Ownership은 여러 번 접하면서 익히는 것
- **C/C++ 비교하지 말 것**: Rust는 다른 철학. 처음부터 Rust식 사고로
- **매일 코드 작성**: 동영상만 보는 것보다 직접 타이핑이 훨씬 효과적

---

## ⏱️ 학습 일정 제안

**주 3회 × 3~4주** 또는 **주 5회 × 2주**

예시 (주 3회 학습):
- 월: 세션 1 + 세션 2
- 수: 세션 3 + 세션 4  
- 금: 세션 5 + 세션 6
- (다음 주) 월: 세션 7 + 세션 8
- (다음 주) 수: 세션 9 + 세션 10

---

## 🏁 목표 달성 기준

학습 완료 후 다음을 할 수 있어야 함:

✅ **필수 능력**
- [ ] 소유권 규칙을 자세히 설명 가능
- [ ] 컴파일러 에러 메시지 읽고 해결 가능
- [ ] 기본 에러 처리 (Result, Option) 작성 가능
- [ ] Trait를 정의하고 구현 가능
- [ ] 간단한 async 함수 작성 가능

✅ **경력에 도움이 될 프로젝트**
- [ ] CLI 도구 (설정 파일 파싱)
- [ ] 간단한 HTTP 클라이언트 (ROS2 통신 응용)
- [ ] 문제 풀이 사이트 (LeetCode) Rust 솔루션

---

## 🚀 ROS2 / 자동차 소프트웨어 응용

20시간 완료 후, 다음 단계:
1. **rclrs** (Rust ROS 2 클라이언트 라이브러리)
2. **DDS & SOME/IP** Rust 구현체
3. 기존 C++ 경험 + Rust = 자동차 미들웨어 개선

---

**화이팅! 🚀 Ownership을 이해하면 Rust는 이미 반은 마스터한 것입니다.**
