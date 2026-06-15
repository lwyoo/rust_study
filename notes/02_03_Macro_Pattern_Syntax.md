# Rust 매크로 패턴 문법 (Macro Pattern Syntax)

---

## 1️⃣ 정규식 vs 매크로 패턴

### 정규식 (Regular Expression)

```
문자열에서 패턴을 찾기 위한 표기법
예: ^[a-z]+@[a-z]+\.[a-z]+$  (이메일 유효성 검사)
```

### 매크로 패턴 문법 (완전히 다른 것!)

```
Rust 매크로에서 입력값의 형태를 지정하는 표기법
예: ($x:expr, $y:expr)
```

---

## 2️⃣ 매크로 패턴의 기본 구조

```rust
macro_rules! my_macro {
    (패턴) => {
        코드
    }
}
```

### 예제

```rust
macro_rules! add {
    ($x:expr, $y:expr) => {
        $x + $y
    }
}
         ↑        ↑
      패턴 부분   코드 부분
```

---

## 3️⃣ 매크로 패턴의 주요 기호들

| 기호 | 의미 | 예시 |
|------|------|------|
| `$` | 매개변수 표시 | `$x` |
| `:expr` | 표현식 타입 | `$x:expr` |
| `:ident` | 식별자 타입 | `$name:ident` |
| `:ty` | 타입 | `$t:ty` |
| `:pat` | 패턴 | `$p:pat` |
| `*` | 0회 이상 반복 | `$($x:expr),*` |
| `+` | 1회 이상 반복 | `$($x:expr),+` |
| `?` | 0회 또는 1회 | `$($x:expr)?` |
| `()` | 그룹화 | `($x:expr, $y:expr)` |

---

## 4️⃣ 주요 매개변수 타입 (Fragment Specifiers)

### :expr (표현식)

```rust
macro_rules! add {
    ($x:expr, $y:expr) => {
        $x + $y
    }
}

fn main() {
    add!(5, 3);              // ✅ 상수
    add!(x, y);              // ✅ 변수
    add!(5 + 3, 2 * 4);      // ✅ 복잡한 표현식
    add!(func(), array[0]);  // ✅ 함수 호출, 배열 접근
}
```

### :ident (식별자)

```rust
macro_rules! declare_variable {
    ($name:ident) => {
        let $name = 42;
    }
}

fn main() {
    declare_variable!(x);  // let x = 42;
    declare_variable!(my_var);  // let my_var = 42;
}
```

### :ty (타입)

```rust
macro_rules! create_variable {
    ($name:ident, $type:ty) => {
        let $name: $type = Default::default();
    }
}

fn main() {
    create_variable!(x, i32);      // let x: i32 = 0;
    create_variable!(s, String);   // let s: String = "";
}
```

### :pat (패턴)

```rust
macro_rules! match_pattern {
    ($pat:pat, $value:expr) => {
        match $value {
            $pat => println!("매칭됨"),
            _ => println!("매칭 안 됨"),
        }
    }
}

fn main() {
    match_pattern!(5, 5);      // 매칭됨
    match_pattern!(10, 5);     // 매칭 안 됨
}
```

---

## 5️⃣ 반복 문법

### * (0회 이상 반복)

```rust
macro_rules! print_all {
    ($($x:expr),*) => {
        // 쉼표로 구분된 0개 이상의 표현식
        $(println!("{}", $x);)*
    }
}

fn main() {
    print_all!();              // 아무것도 출력 안 함
    print_all!(1);             // 1 출력
    print_all!(1, 2, 3);       // 1, 2, 3 각각 출력
}
```

### + (1회 이상 반복)

```rust
macro_rules! min {
    ($first:expr) => {
        $first
    };
    ($first:expr, $($rest:expr),+) => {
        // 최소 2개 이상 필요
        std::cmp::min($first, min!($($rest),+))
    }
}

fn main() {
    println!("{}", min!(5));           // 5
    println!("{}", min!(5, 3));        // 3
    println!("{}", min!(5, 3, 7));     // 3
}
```

### ? (0회 또는 1회)

```rust
macro_rules! optional {
    ($x:expr $(, $y:expr)?) => {
        // $y는 선택사항
        if let Some(y) = ($($y)?) {
            println!("x={}, y={}", $x, y);
        } else {
            println!("x={}", $x);
        }
    }
}

fn main() {
    optional!(5);      // x=5
    optional!(5, 3);   // x=5, y=3
}
```

---

## 6️⃣ 실제 예제로 이해하기

### 예제 1: vec! 매크로 분석

```rust
// Rust 표준 라이브러리의 vec! 매크로
macro_rules! vec {
    () => {
        Vec::new()
    };
    ($($x:expr),+) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)+
            temp_vec
        }
    };
}

fn main() {
    let v1 = vec![];           // Vec::new()
    let v2 = vec![1];          // 1개 요소
    let v3 = vec![1, 2, 3];    // 3개 요소
}
```

### 예제 2: 여러 패턴 정의

```rust
macro_rules! test_type {
    ($x:expr) => {
        println!("단일 값: {}", $x);
    };
    ($x:expr, $y:expr) => {
        println!("두 개 값: {}, {}", $x, $y);
    };
    ($x:expr, $y:expr, $z:expr) => {
        println!("세 개 값: {}, {}, {}", $x, $y, $z);
    };
}

fn main() {
    test_type!(5);           // 단일 값: 5
    test_type!(5, 10);       // 두 개 값: 5, 10
    test_type!(5, 10, 15);   // 세 개 값: 5, 10, 15
}
```

---

## 7️⃣ 패턴 분해하기

### 복잡한 패턴 예제

```rust
macro_rules! complex_macro {
    ($($x:expr),*)  // ← 이게 뭐냐?
    =>
    { ... }
}
```

**분해:**

```
$($x:expr),*
↑ ↑ ↑     ↑
│ │ │     └─ 0회 이상 반복
│ │ └─────── 식별자 x는 expr 타입
│ └───────── 매개변수 표시
└─────────── 반복할 그룹
```

**의미:** "0개 이상의 표현식을 쉼표로 구분하여 입력받음"

---

## 8️⃣ 생활 코딩으로 이해하기

```rust
// 이해하기 쉬운 방식으로 설명

macro_rules! greet {
    //  패턴        => 코드
    ($name:ident) => {
        println!("Hello, {}!", stringify!($name));
    }
}

fn main() {
    greet!(Alice);  // ← Alice는 식별자(:ident)
    // 컴파일 단계에서:
    // println!("Hello, {}!", "Alice");
}
```

---

## 9️⃣ 패턴 문법 vs 정규식 비교

### 정규식 (문자열 검색)

```
패턴: ^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}$
입력: "192.168.1.1"
결과: 매칭 여부 (true/false)
```

### 매크로 패턴 (코드 입력 형태 지정)

```
패턴: ($x:expr, $y:expr)
입력: add!(5, 3)
결과: $x=5, $y=3 으로 바인딩되어 코드 생성
```

---

## 🔟 주요 Fragment Specifiers 전체 목록

| Specifier | 설명 | 예시 |
|-----------|------|------|
| `:expr` | 표현식 | `5 + 3`, `x`, `func()` |
| `:ident` | 식별자 (변수명 등) | `x`, `my_var`, `Point` |
| `:ty` | 타입 | `i32`, `String`, `Vec<i32>` |
| `:pat` | 패턴 | `5`, `(x, y)`, `Some(x)` |
| `:stmt` | 문장 | `let x = 5;`, `return 42;` |
| `:block` | 블록 | `{ ... }` |
| `:item` | 아이템 | `fn foo() { }`, `struct Point { }` |
| `:meta` | 속성 | `#[derive(Debug)]` |
| `:tt` | 토큰 트리 (뭐든지) | 거의 모든 것 |

---

## 1️⃣1️⃣ 정리: 매크로 패턴 vs 정규식

```
정규식:
- 텍스트/문자열 검색
- 패턴 매칭 (yes/no)
- 예: email@domain.com 형식 확인

매크로 패턴:
- Rust 코드의 입력 형태 지정
- 입력값 바인딩 및 코드 생성
- 예: add!(5, 3) → $x=5, $y=3
```

---

## 📌 핵심 정리

```rust
macro_rules! example {
    //     패턴        =>    코드
    ($x:expr, $y:expr) => {
        $x + $y
    }
}

// 각 부분:
// $x:expr      = "매개변수 x는 표현식 타입"
// $y:expr      = "매개변수 y는 표현식 타입"
// $x, $y       = "코드에서 사용할 변수명"
// :expr        = "어떤 형태의 입력을 받을지 지정" (정규식 ❌)
```

이제 차이가 명확하죠? 매크로 패턴은 **Rust만의 고유한 문법**입니다! 🦀