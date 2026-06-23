# Rust 열거형과 패턴 매칭 (Enum and Pattern Matching)

---

## 1️⃣ 열거형 (Enum)

### 개념

**여러 경우의 수를 정의하는 타입입니다.**

### 기본 열거형

```rust
enum Direction {
    East,
    West,
    South,
    North,
}

fn main() {
    let dir = Direction::East;
    println!("방향: 동쪽");
}
```

### 데이터가 없는 열거형

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Red;
    // 사용...
}
```

---

## 2️⃣ Match 구조

### 표현식 vs 문장

#### 표현식 (세미콜론 없음)

```rust
let result = match value {
    case1 => value1,
    case2 => value2,
    _ => value3
};  // ← 세미콜론!
```

**특징:**
- `;` 없음 (각 arm)
- 값을 반환
- 변수에 할당 가능

#### 문장 (세미콜론 있음)

```rust
match value {
    case1 => {
        println!("case1");
    },
    case2 => {
        println!("case2");
    },
    _ => {
        println!("default");
    }
};  // ← 세미콜론!
```

**특징:**
- `;` 있음 (각 arm)
- 값을 반환하지 않음
- 단순히 코드 실행

### Match 기본 문법

```rust
match 변수 {
    패턴1 => 표현식1,
    패턴2 => 표현식2,
    패턴3 => 표현식3,
    _ => 기본값,
}
```

---

## 3️⃣ 열거형과 비교 연산자

### ❌ 열거형은 if로 비교 불가

```rust
enum Status {
    Active,
    Inactive,
}

fn main() {
    let status = Status::Active;
    
    // ❌ 에러!
    if status == Status::Active {
        println!("활성화");
    }
}
```

**에러 이유:** 열거형은 `==` 연산자를 지원하지 않음

### ✅ Match로만 비교

```rust
enum Status {
    Active,
    Inactive,
}

fn main() {
    let status = Status::Active;
    
    match status {
        Status::Active => println!("활성화"),
        Status::Inactive => println!("비활성화"),
    }
}
```

---

## 4️⃣ 숫자와 함께 Match 사용

### 기본 예제

```rust
fn main() {
    let mut num = 11;
    
    match num {
        10 => println!("이것은 10입니다"),
        _ => {
            num = 1;
            println!("이것은 {}", num);
        }
    }
}
```

**출력:**
```
이것은 1
```

### 여러 경우 처리

```rust
fn main() {
    let score = 85;
    
    match score {
        90..=100 => println!("A"),
        80..=89  => println!("B"),
        70..=79  => println!("C"),
        60..=69  => println!("D"),
        _        => println!("F"),
    }
}
```

**출력:**
```
B
```

---

## 5️⃣ 데이터가 있는 열거형

### 정의

```rust
enum Result {
    Success(u8),           // 1개 데이터
    Failure(u16, char),    // 2개 데이터
    Uncertainty,           // 데이터 없음
}
```

### 사용

```rust
fn main() {
    let r1 = Result::Success(0);
    let r2 = Result::Failure(100, 'a');
    let r3 = Result::Uncertainty;
}
```

### Match로 처리

```rust
enum Result {
    Success(u8),
    Failure(u16, char),
    Uncertainty,
}

fn main() {
    let r = Result::Success(0);
    
    match r {
        Result::Success(0) => println!("성공 코드: 0"),
        Result::Success(1) => println!("성공 코드: 1"),
        Result::Success(2) => println!("성공 코드: 2"),
        Result::Success(_) => println!("성공 코드: 기타"),
        Result::Failure(_, _) => println!("실패!"),
        Result::Uncertainty => println!("불확실"),
        _ => println!("에러"),
    }
}
```

**출력:**
```
성공 코드: 0
```

---

## 6️⃣ 패턴에서 변수 사용 (쉐도잉)

### 개념

**Match 블록 내에서 데이터를 변수로 추출합니다.**

### 예제

```rust
enum Result {
    Success(u8),
    Failure(u16, char),
    Uncertainty,
}

fn main() {
    let r = Result::Success(50);
    let a = 99;  // 바깥쪽 변수
    
    match r {
        // match 블록 내에서 a는 Success의 데이터로 쉐도잉됨
        Result::Success(a) => println!("성공 코드: {}", a),  // a = 50
        Result::Failure(code, ch) => println!("실패: {} {}", code, ch),
        Result::Uncertainty => println!("불확실"),
        _ => println!("에러"),
    }
    
    // match 블록 밖에서는 a = 99
    println!("바깥쪽 a: {}", a);  // 99
}
```

**출력:**
```
성공 코드: 50
바깥쪽 a: 99
```

### 와일드카드와 조합

```rust
enum Result {
    Success(u8),
    Failure(u16, char),
}

fn main() {
    let r = Result::Failure(404, 'E');
    
    match r {
        Result::Success(code) => println!("성공: {}", code),
        Result::Failure(_, ch) => println!("에러 문자: {}", ch),  // 첫 번째는 무시
    }
}
```

**출력:**
```
에러 문자: E
```

---

## 7️⃣ Match 표현식으로 값 반환

### 기본 예제

```rust
enum Direction {
    East,
    West,
    South,
    North,
}

fn main() {
    let dir = Direction::South;
    
    let text = match dir {
        Direction::East => "동쪽이요",
        Direction::West => "서쪽이요",
        Direction::South => "남쪽이요",
        Direction::North => "북쪽이요",
    };
    
    println!("나의 방향은: {}", text);
}
```

**출력:**
```
나의 방향은: 남쪽이요
```

### 직접 println!에 사용

```rust
enum Direction {
    East,
    West,
    South,
    North,
}

fn main() {
    let dir = Direction::North;
    
    println!("나의 방향은: {}", 
        match dir {
            Direction::East => "동쪽",
            Direction::West => "서쪽",
            Direction::South => "남쪽",
            Direction::North => "북쪽",
        }
    );
}
```

**출력:**
```
나의 방향은: 북쪽
```

---

## 8️⃣ Match 가드 (Guard)

### 개념

**Match 패턴에 추가 조건을 붙입니다.**

### 문법

```rust
match value {
    pattern if condition => expression,
    _ => expression,
}
```

### 예제 1: 숫자 분류

```rust
fn main() {
    for n in -10..=10 {
        println!("{}: {}", n, match n {
            0 => "zero",
            _ if n > 0 => "양수",
            _ => "음수",
        });
    }
}
```

**출력:**
```
-10: 음수
-9: 음수
...
0: zero
1: 양수
2: 양수
...
10: 양수
```

### 예제 2: 범위와 조건

```rust
fn main() {
    let age = 25;
    
    match age {
        18..=65 if age >= 25 => println!("성인 (25세 이상)"),
        18..=65 => println!("성인"),
        _ => println!("어린이 또는 노인"),
    }
}
```

**출력:**
```
성인 (25세 이상)
```

### 예제 3: 데이터와 가드

```rust
enum Event {
    Click(u32),
    Scroll(i32),
}

fn main() {
    let event = Event::Click(500);
    
    match event {
        Event::Click(x) if x > 300 => println!("긴 클릭: {}", x),
        Event::Click(x) => println!("짧은 클릭: {}", x),
        Event::Scroll(y) if y > 0 => println!("스크롤 다운: {}", y),
        Event::Scroll(y) => println!("스크롤 업: {}", y),
    }
}
```

**출력:**
```
긴 클릭: 500
```

---

## 9️⃣ if-let 구조

### 개념

**한 가지 패턴만 처리할 때 match를 간단히 쓰는 문법입니다.**

### Match 방식

```rust
enum E {
    Case1(bool, i32),
    Case2(bool, i32),
    Case3(bool, i32),
}

fn main() {
    let v = E::Case1(true, 1);
    
    match v {
        E::Case1(state, code) => {
            if state {
                println!("Case1 (state: {}, code: {})", state, code);
            }
        },
        _ => {},
    }
}
```

### if-let 방식 (간단)

```rust
enum E {
    Case1(bool, i32),
    Case2(bool, i32),
    Case3(bool, i32),
}

fn main() {
    let v = E::Case1(true, 1);
    
    // 한 가지만 처리!
    if let E::Case1(state, code) = v {
        if state {
            println!("Case1 (state: {}, code: {})", state, code);
        }
    } else {
        println!("Case1이 아님");
    }
}
```

### if-let 문법

```rust
if let pattern = value {
    // 패턴과 일치할 때 실행
} else {
    // 일치하지 않을 때 실행
}
```

### 실제 예제

```rust
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let x = Some(5);
    
    // 긴 match 방식
    match x {
        Some(n) => println!("값: {}", n),
        None => {},
    }
    
    // 간단한 if-let 방식
    if let Some(n) = x {
        println!("값: {}", n);
    }
}
```

---

## 🔟 while-let 구조

### 개념

**패턴과 일치하는 동안 반복합니다.**

### 기본 문법

```rust
while let pattern = value {
    // 패턴과 일치하는 동안 반복
}
```

### 예제 1: 증가하는 값

```rust
enum E {
    Case1(u32),
    Case2(char),
}

fn main() {
    let mut v = E::Case1(1);
    
    while let E::Case1(n) = v {
        println!("{}", n);
        if n == 5 {
            break;
        }
        v = E::Case1(n + 1);
    }
}
```

**출력:**
```
1
2
3
4
5
```

### 예제 2: Option과 함께

```rust
fn main() {
    let mut stack = vec![1, 2, 3];
    
    while let Some(top) = stack.pop() {
        println!("팝된 값: {}", top);
    }
}
```

**출력:**
```
팝된 값: 3
팝된 값: 2
팝된 값: 1
```

### 예제 3: 사용자 입력

```rust
use std::io;

fn main() {
    let mut input = String::new();
    
    while let Ok(_) = io::stdin().read_line(&mut input) {
        println!("입력: {}", input.trim());
        if input.trim() == "quit" {
            break;
        }
        input.clear();
    }
}
```

---

## 1️⃣1️⃣ if-let vs while-let vs match 비교

| 상황 | if-let | while-let | match |
|------|--------|----------|-------|
| **한 패턴만** | ✅ 최고 | ❌ | ⚠️ 장황 |
| **여러 패턴** | ❌ | ❌ | ✅ 최고 |
| **반복 처리** | ❌ | ✅ 최고 | ⚠️ 복잡 |
| **조건 추가** | ✅ if 사용 | ✅ if 사용 | ✅ guard 사용 |

---

## 1️⃣2️⃣ 실무 예제

### 예제 1: API 응답 처리

```rust
enum ApiResponse {
    Success(String),
    NotFound,
    Error(u32, String),
}

fn main() {
    let response = ApiResponse::Success("데이터".to_string());
    
    match response {
        ApiResponse::Success(data) => println!("성공: {}", data),
        ApiResponse::NotFound => println!("찾을 수 없음"),
        ApiResponse::Error(code, msg) => println!("에러 {}: {}", code, msg),
    }
}
```

### 예제 2: 상태 관리

```rust
enum State {
    Idle,
    Running(u32),
    Paused(u32),
    Stopped,
}

fn main() {
    let state = State::Running(50);
    
    let status = match state {
        State::Idle => "대기 중",
        State::Running(progress) if progress >= 100 => "완료",
        State::Running(progress) => format!("실행 중 ({}%)", progress),
        State::Paused(progress) => format!("일시 중지 ({}%)", progress),
        State::Stopped => "중지됨",
    };
    
    println!("{}", status);
}
```

### 예제 3: 파일 처리

```rust
enum FileOp {
    Read(String),
    Write(String, String),
    Delete(String),
}

fn main() {
    let op = FileOp::Write("file.txt".to_string(), "내용".to_string());
    
    if let FileOp::Write(name, content) = op {
        println!("파일 '{}' 쓰기: {}", name, content);
    } else if let FileOp::Read(name) = op {
        println!("파일 '{}' 읽기", name);
    }
}
```

---

## 📌 핵심 정리

```rust
// 1. 열거형 정의
enum Status {
    Active,
    Inactive,
}

// 2. Match 표현식 (값 반환)
let result = match status {
    Status::Active => "활성",
    Status::Inactive => "비활성",
};

// 3. 데이터가 있는 열거형
enum Result {
    Ok(T),
    Err(E),
}

// 4. Match로 데이터 추출
match result {
    Result::Ok(value) => println!("{}", value),
    Result::Err(error) => println!("에러: {}", error),
}

// 5. if-let (한 가지만)
if let Result::Ok(value) = result {
    println!("{}", value);
}

// 6. while-let (반복)
while let Some(item) = iterator.next() {
    println!("{}", item);
}

// 7. 가드 (조건 추가)
match value {
    x if x > 0 => println!("양수"),
    _ => println!("0 이하"),
}
```

---

## 🎯 체크리스트

- [ ] 열거형 정의 이해
- [ ] Match 문법 완벽히 이해
- [ ] 데이터가 있는 열거형 사용
- [ ] Match 표현식으로 값 반환
- [ ] 가드로 조건 추가
- [ ] if-let으로 간단하게 처리
- [ ] while-let으로 반복 처리
- [ ] 세 가지의 차이 구분

이제 Rust의 열거형과 패턴 매칭이 완벽하게 이해되셨을 거예요! 🦀