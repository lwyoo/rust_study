# Rust 튜플, 구조체, 어휘규칙

---

## 1️⃣ 튜플 (Tuple)

### 개념

**여러 타입의 데이터를 하나의 컬렉션에 저장하는 자료형입니다.**

### 사용처

- **서로 다른 데이터 타입을 하나의 변수에 저장할 때**
- 함수에서 여러 값 반환
- 임시 데이터 묶음

### 기본 사용 방법

```rust
fn main() {
    // 튜플 선언
    let person: (String, i32, f64) = ("Alice".to_string(), 25, 5.8);
    
    // 접근
    println!("이름: {}", person.0);      // Alice
    println!("나이: {}", person.1);      // 25
    println!("키: {}", person.2);        // 5.8
    
    // 언팩
    let (name, age, height) = person;
    println!("{}, {}, {}", name, age, height);
}
```

### 빈 튜플 (Unit Type)

```rust
fn main() {
    let empty: () = ();
    println!("빈 튜플: {:?}", empty);  // ()
}
```

### 튜플 장점 ✅

| 장점 | 설명 | 예시 |
|------|------|------|
| **다양한 타입** | 데이터 타입이 달라도 가능 | `(i32, String, f64, bool)` |
| **단일 객체** | 하나의 단일 객체에 여러 데이터 타입 저장 | 함수 반환값 |
| **필드명 불필요** | 필드명 없이 간단하게 사용 | `person.0` |
| **필드 수 적음** | 필드의 수가 적을 때 주로 사용 | 2~3개 데이터 |

### 튜플 단점 ❌

| 단점 | 설명 | 해결책 |
|------|------|--------|
| **데이터 추가 복잡** | 나중에 필드를 추가하면 타입을 일일이 수정 | 구조체 사용 |
| **접근 불편** | 필드 접근 시 번호를 세어야 함 | 구조체 사용 (필드명) |
| **유지보수 어려움** | 중간에 필드 추가 시 기존 데이터 모두 변경 | 구조체 사용 |

### 튜플 단점 예제

```rust
fn main() {
    // 처음: (이름, 나이)
    let person = ("Alice", 25);
    
    // 나중에 키를 추가해야 함!
    // 기존: let person: (&str, i32) = ("Alice", 25);
    // 수정 후: let person: (&str, i32, f64) = ("Alice", 25, 5.8);
    
    // 모든 코드를 수정해야 함 ❌
    let person = ("Alice", 25, 5.8);
    println!("{}, {}, {}", person.0, person.1, person.2);
}
```

---

## 2️⃣ 구조체 (Struct)

### 개념

**Rust는 클래스가 없습니다! 그 대신 구조체를 사용합니다.**

### 기본 문법

```rust
// 구조체 정의
struct StructName {
    field1: Type1,
    field2: Type2,
    field3: Type3,
}

// 구조체 생성
let data = StructName {
    field1: value1,
    field2: value2,
    field3: value3,
};

// 필드 접근
println!("{}", data.field1);

// 필드 수정 (mut 필요)
let mut data = StructName { ... };
data.field1 = new_value;
```

### 사용처

- **복잡한 데이터 모델링**
- **객체 지향 프로그래밍**
- **필드가 많을 때**
- **필드명이 중요한 경우**

### 기본 예제

```rust
struct Person {
    name: String,
    age: i32,
    height: f64,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 25,
        height: 5.8,
    };
    
    println!("이름: {}", person.name);    // Alice
    println!("나이: {}", person.age);     // 25
    println!("키: {}", person.height);    // 5.8
}
```

### 구조체 수정

```rust
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let mut person = Person {
        name: "Bob".to_string(),
        age: 30,
    };
    
    // 필드 수정
    person.age = 31;
    person.name = "Charlie".to_string();
    
    println!("{}, {}", person.name, person.age);  // Charlie, 31
}
```

### 필드 간단 초기화

```rust
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let name = "Alice".to_string();
    let age = 25;
    
    // 필드명과 변수명이 같으면 간단하게
    let person = Person { name, age };
    
    println!("{}, {}", person.name, person.age);
}
```

### 구조체 장점 ✅

| 장점 | 설명 | 예시 |
|------|------|------|
| **필드명** | 명확한 필드명으로 접근 | `person.name` |
| **타입 안전** | 각 필드에 타입 명시 | `name: String` |
| **유지보수** | 필드 추가/제거가 명확 | 새 필드 추가 용이 |
| **확장성** | 메서드 추가 가능 | `impl Person { ... }` |

### 구조체 단점 ❌

| 단점 | 설명 |
|------|------|
| **코드 양** | 정의가 더 길고 복잡 |
| **간단한 데이터** | 2~3개 필드면 과할 수 있음 |

### 메서드 추가 예제

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("넓이: {}", rect.area());        // 1500
    println!("둘레: {}", rect.perimeter());   // 160
}
```

---

## 3️⃣ 튜플 구조체 (Tuple Struct)

### 개념

**구조체와 튜플의 장점을 결합합니다!**

- **구조체 장점:** 타입 정의
- **튜플 장점:** 필드명 없이 간단하게 사용

### 문법

```rust
struct TupleName(Type1, Type2, Type3);

let data = TupleName(value1, value2, value3);

// 접근
println!("{}", data.0);  // 첫 번째
println!("{}", data.1);  // 두 번째
println!("{}", data.2);  // 세 번째
```

### 예제

```rust
// 튜플 구조체 정의
struct Color(u8, u8, u8);
struct Point(f64, f64, f64);

fn main() {
    let red = Color(255, 0, 0);
    let origin = Point(0.0, 0.0, 0.0);
    
    println!("Red: ({}, {}, {})", red.0, red.1, red.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);
}
```

**출력:**
```
Red: (255, 0, 0)
Origin: (0, 0, 0)
```

### 튜플 구조체 장점 ✅

| 장점 | 설명 |
|------|------|
| **타입 정의** | 구조체처럼 타입 명시 |
| **필드명 간소** | 튜플처럼 필드명 불필요 |
| **간결함** | 코드가 간단 |
| **빠른 개발** | 간단한 데이터에 최적 |

### 튜플 구조체 단점 ❌

| 단점 | 설명 |
|------|------|
| **필드명 불명확** | `data.0`의 의미가 불명확 |
| **복잡한 데이터** | 많은 필드면 튜플 구조체 부적합 |

### 비교 예제

```rust
// 방법 1: 일반 튜플
let person1: (String, i32) = ("Alice".to_string(), 25);

// 방법 2: 튜플 구조체
struct Person(String, i32);
let person2 = Person("Alice".to_string(), 25);

// 방법 3: 구조체
struct PersonStruct {
    name: String,
    age: i32,
}
let person3 = PersonStruct {
    name: "Alice".to_string(),
    age: 25,
};

fn main() {
    // 모두 같은 데이터지만 다른 타입!
    println!("{}", person1.0);           // Alice
    println!("{}", person2.0);           // Alice
    println!("{}", person3.name);        // Alice
}
```

---

## 4️⃣ 튜플 vs 구조체 vs 튜플 구조체 비교

| 항목 | 튜플 | 튜플 구조체 | 구조체 |
|------|------|-----------|--------|
| **문법** | `(i32, String)` | `struct Color(i32, i32)` | `struct {...}` |
| **필드명** | ❌ 없음 | ❌ 없음 | ✅ 있음 |
| **타입** | 암묵적 | ✅ 명시적 | ✅ 명시적 |
| **접근** | `tuple.0` | `struct.0` | `struct.field` |
| **필드 수** | 적음 (2~3) | 적음 (2~3) | 많음 (4+) |
| **가독성** | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **유지보수** | 어려움 | 보통 | 쉬움 |
| **복잡도** | 낮음 | 낮음 | 높음 |

---

## 5️⃣ 실무 예제

### 예제 1: 좌표 (튜플 vs 튜플 구조체)

```rust
// 튜플: 간단하지만 의미 불명확
let point1 = (10.5, 20.3);

// 튜플 구조체: 타입 안전 + 간단
struct Point(f64, f64);
let point2 = Point(10.5, 20.3);

fn main() {
    println!("점 1: ({}, {})", point1.0, point1.1);
    println!("점 2: ({}, {})", point2.0, point2.1);
}
```

### 예제 2: 사용자 정보 (구조체)

```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

fn main() {
    let user = User {
        username: "alice123".to_string(),
        email: "alice@example.com".to_string(),
        age: 25,
        active: true,
    };
    
    println!("사용자: {}", user.username);
    println!("이메일: {}", user.email);
    println!("나이: {}", user.age);
    println!("활성: {}", user.active);
}
```

### 예제 3: RGB 색상 (튜플 구조체)

```rust
struct Color(u8, u8, u8);

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b)
    }
    
    fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

fn main() {
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255, 0);
    let blue = Color::new(0, 0, 255);
    
    println!("빨강: {}", red.to_hex());      // #ff0000
    println!("초록: {}", green.to_hex());    // #00ff00
    println!("파랑: {}", blue.to_hex());     // #0000ff
}
```

### 예제 4: 복합 구조체

```rust
struct Address {
    street: String,
    city: String,
    country: String,
}

struct Person {
    name: String,
    age: i32,
    address: Address,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 25,
        address: Address {
            street: "123 Main St".to_string(),
            city: "Seoul".to_string(),
            country: "Korea".to_string(),
        },
    };
    
    println!("이름: {}", person.name);
    println!("주소: {} {}, {}", 
             person.address.street,
             person.address.city,
             person.address.country);
}
```

---

## 6️⃣ 구조체 메서드

### 기본 메서드

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self: 불변 참조 메서드
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // &mut self: 가변 참조 메서드
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    // self: 소유권 이동 메서드
    fn into_square(self) -> Rectangle {
        Rectangle {
            width: self.width,
            height: self.width,
        }
    }
    
    // 생성자 역할
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let mut rect = Rectangle::new(30, 40);
    
    println!("넓이: {}", rect.area());       // 1200
    
    rect.set_width(50);
    println!("새 넓이: {}", rect.area());    // 2000
    
    let square = rect.into_square();
    println!("정사각형: {}", square.area()); // 2500
}
```

---

## 7️⃣ 어휘 규칙 (Naming Conventions)

### Rust 명명 규칙

| 항목 | 규칙 | 예시 |
|------|------|------|
| **상수** | UPPER_CASE | `const MAX_SIZE: usize = 100;` |
| **구조체** | PascalCase | `struct Rectangle { }` |
| **구조체 필드** | snake_case | `first_name: String` |
| **함수/메서드** | snake_case | `fn calculate_area()` |
| **변수** | snake_case | `let user_age = 25;` |
| **모듈** | snake_case | `mod user_module` |
| **트레이트** | PascalCase | `trait Iterator { }` |

### 예제

```rust
const MAX_USERS: usize = 1000;

struct UserProfile {
    user_id: u32,
    user_name: String,
    email_address: String,
    is_active: bool,
}

impl UserProfile {
    fn new_user(user_id: u32, user_name: String) -> UserProfile {
        UserProfile {
            user_id,
            user_name,
            email_address: String::new(),
            is_active: true,
        }
    }
    
    fn set_email(&mut self, email: String) {
        self.email_address = email;
    }
}

fn main() {
    let mut user = UserProfile::new_user(1, "alice".to_string());
    user.set_email("alice@example.com".to_string());
}
```

---

## 📌 핵심 정리

```rust
// 1. 간단한 데이터 2~3개: 튜플
let person = ("Alice", 25);
println!("{}", person.0);

// 2. 타입 안전한 간단한 데이터: 튜플 구조체
struct Point(f64, f64);
let point = Point(10.5, 20.3);

// 3. 필드명이 중요한 복잡한 데이터: 구조체
struct Person {
    name: String,
    age: i32,
}
let person = Person { 
    name: "Alice".to_string(), 
    age: 25,
};
```

---

## 🎯 언제 뭘 사용할까?

```
튜플:
- 임시 데이터
- 함수 반환값
- 2~3개 필드

튜플 구조체:
- 타입 안전이 필요
- 필드명 불필요
- 간단한 데이터 래핑

구조체:
- 복잡한 데이터 모델
- 필드명이 명확해야 함
- 메서드 추가 필요
- 4개 이상 필드
```

이제 Rust의 튜플, 구조체, 튜플 구조체를 완벽하게 이해하셨을 거예요! 🦀