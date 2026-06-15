# Rust 문자열 포맷팅 (String Formatting)

---

## 1️⃣ 기본 개념

### println! 매크로의 구조

```rust
println!("{} {}", "hello", "world");
            ↑        ↑        ↑
            │        │        └───── 데이터 2
            │        └────────────── 데이터 1
            └─────────────────────── 포맷 문자열
```

| 항목 | 설명 | 예시 |
|------|------|------|
| **포맷 문자열** | 출력 형식 정의 | `"{} {}"` |
| **위치 지정자** | 데이터를 삽입할 위치 | `{}` |
| **데이터** | 삽입할 실제 값 | `"hello"`, `"world"` |

---

## 2️⃣ 위치 지정자 (Placeholders)

### 기본: {} (순서대로)

```rust
fn main() {
    println!("{} {}", "hello", "world");
    // 출력: hello world
}
```

**동작:**
```
{}     ← 첫 번째 {} → "hello"
{}     ← 두 번째 {} → "world"
```

### 인덱스 지정: {0} {1}

```rust
fn main() {
    println!("{1} {0}", "world", "hello");
    // 출력: hello world
    
    println!("{0} {0} {1}", "A", "B");
    // 출력: A A B
}
```

**인덱스:**
```
{0}   ← 0번 인덱스: "world"
{1}   ← 1번 인덱스: "hello"
```

### 이름 지정: {name}

```rust
fn main() {
    println!("{greeting} {name}", greeting = "hello", name = "world");
    // 출력: hello world
    
    println!("{language} is {adjective}",
             language = "Rust",
             adjective = "awesome");
    // 출력: Rust is awesome
}
```

---

## 3️⃣ 포맷 문자열의 다양한 형태

### 간단한 출력

```rust
fn main() {
    let x = 42;
    let name = "Rust";
    
    println!("{}", x);                    // 42
    println!("{} {}", x, name);           // 42 Rust
    println!("{} {} {}", x, x, name);     // 42 42 Rust
}
```

### 인덱스 활용

```rust
fn main() {
    let nums = vec![10, 20, 30];
    
    // 순서 변경
    println!("{1} {0} {2}", nums[0], nums[1], nums[2]);
    // 출력: 20 10 30
    
    // 같은 값 반복 사용
    println!("{0} + {0} = {0}", 5);
    // 출력: 5 + 5 = 5
}
```

### 이름 지정 활용

```rust
fn main() {
    println!("{x} + {y} = {sum}",
             x = 5,
             y = 3,
             sum = 8);
    // 출력: 5 + 3 = 8
    
    // 이름 반복 사용
    println!("{msg} {msg} {msg}",
             msg = "Hello");
    // 출력: Hello Hello Hello
}
```

---

## 4️⃣ 포맷 옵션 (Format Specifiers)

### 기본 형태

```rust
{[index|name][:[[fill]align][width][.precision][type]]}
```

### 정렬 (Alignment)

```rust
fn main() {
    // 왼쪽 정렬 (<)
    println!("{:<10}", "hello");
    // 출력: hello     
    
    // 오른쪽 정렬 (>)
    println!("{:>10}", "hello");
    // 출력:      hello
    
    // 중앙 정렬 (^)
    println!("{:^10}", "hello");
    // 출력:   hello   
}
```

### 채움 문자 (Fill)

```rust
fn main() {
    // 기본 (공백)
    println!("{:>10}", "hi");
    // 출력: |        hi|
    
    // * 로 채우기
    println!("{:*>10}", "hi");
    // 출력: |********hi|
    
    // - 로 채우기
    println!("{:->10}", "hi");
    // 출력: |--------hi|
}
```

### 너비 (Width)

```rust
fn main() {
    // 너비 10으로 설정
    println!("{:10}", "hello");
    // 출력: |hello     |
    
    // 동적 너비
    let width = 10;
    println!("{:width$}", "hello", width = width);
    // 출력: |hello     |
}
```

### 소수점 정밀도 (Precision)

```rust
fn main() {
    let pi = 3.14159265;
    
    // 소수점 2자리
    println!("{:.2}", pi);
    // 출력: 3.14
    
    // 소수점 4자리
    println!("{:.4}", pi);
    // 출력: 3.1416
    
    // 문자열: 최대 3글자
    println!("{:.3}", "hello");
    // 출력: hel
}
```

### 타입 지정자 (Type)

```rust
fn main() {
    let num = 255;
    
    // 정수 (기본)
    println!("{}", num);
    // 출력: 255
    
    // 16진수 (0x)
    println!("{:x}", num);
    // 출력: ff
    
    // 8진수 (0o)
    println!("{:o}", num);
    // 출력: 377
    
    // 2진수 (0b)
    println!("{:b}", num);
    // 출력: 11111111
    
    // 과학 표기법
    println!("{:e}", 1234.5);
    // 출력: 1.2345e3
}
```

---

## 5️⃣ 실무 예제들

### 예제 1: 테이블 형식 출력

```rust
fn main() {
    println!("{:15} {:10} {:8}", "이름", "나이", "점수");
    println!("{:-<15} {:-<10} {:-<8}", "", "", "");
    println!("{:15} {:10} {:8}", "김철수", "25", "95");
    println!("{:15} {:10} {:8}", "이영희", "23", "87");
    println!("{:15} {:10} {:8}", "박민준", "26", "92");
}
```

**출력:**
```
이름            나이         점수    
--------------- ---------- --------
김철수          25         95      
이영희          23         87      
박민준          26         92      
```

### 예제 2: 가격 포맷팅

```rust
fn main() {
    let prices = vec![1000.50, 2500.99, 15000.00];
    
    println!("가격 목록:");
    for (i, price) in prices.iter().enumerate() {
        println!("상품 {}: ￥{:>8.2}", i + 1, price);
    }
}
```

**출력:**
```
가격 목록:
상품 1: ￥ 1000.50
상품 2: ￥ 2500.99
상품 3: ￥15000.00
```

### 예제 3: 진행 상황 표시

```rust
fn main() {
    let progress = 75.5;
    
    println!("진행: {:5.1}%", progress);
    // 출력: 진행:  75.5%
    
    println!("진행: {:<20} {:5.1}%", "█████████░░░░░░░░░░", progress);
    // 출력: 진행: █████████░░░░░░░░░░  75.5%
}
```

### 예제 4: 여러 타입 포맷팅

```rust
fn main() {
    let int_val = 42;
    let float_val = 3.14159;
    let string_val = "Rust";
    let bool_val = true;
    
    println!("정수: {}", int_val);
    println!("실수: {:.2}", float_val);
    println!("문자열: {:>10}", string_val);
    println!("불린: {}", bool_val);
    
    // 16진수, 2진수
    println!("16진수: {:X}", 255);     // FF
    println!("2진수: {:b}", 15);       // 1111
}
```

**출력:**
```
정수: 42
실수: 3.14
문자열:       Rust
불린: true
16진수: FF
2진수: 1111
```

---

## 6️⃣ 포맷 문자와 데이터의 관계

### 단계적 매칭

```rust
println!("{} + {} = {}", 5, 3, 8);
          ↑    ↑    ↑    ↑  ↑  ↑
          │    │    │    │  │  └── 3번째 데이터: 8
          │    │    │    │  └───── 2번째 데이터: 3
          │    │    │    └──────── 1번째 데이터: 5
          │    │    └───────────── 위치 지정자 3
          │    └────────────────── 위치 지정자 2
          └─────────────────────── 위치 지정자 1

출력: 5 + 3 = 8
```

### 잘못된 예제

```rust
// ❌ 위치 지정자 부족
println!("{} {} {}", 1, 2);  // 에러! 3번째 {} 에 대한 데이터 없음

// ❌ 데이터 부족
println!("{} {}", 1);  // 에러! 2번째 {} 에 대한 데이터 없음

// ✅ 올바름
println!("{} {}", 1, 2);  // 1 2
```

---

## 7️⃣ 실용적인 포맷 조합

| 용도 | 코드 | 결과 |
|------|------|------|
| **오른쪽 정렬 너비 8** | `{:>8}` | `   hello` |
| **왼쪽 정렬 너비 8** | `{:<8}` | `hello   ` |
| **중앙 정렬 너비 8** | `{:^8}` | ` hello  ` |
| **0으로 채운 숫자** | `{:05}` | `00042` |
| **소수점 2자리** | `{:.2}` | `3.14` |
| **16진수** | `{:x}` | `ff` |
| **왼쪽 정렬 * 채움 너비 10** | `{:*<10}` | `hello*****` |
| **이름 지정** | `{name:>10}` | `      Rust` |

---

## 8️⃣ 자주 사용하는 패턴

### 패턴 1: 금전 표시

```rust
fn main() {
    let amount = 1234567.89;
    println!("₩{:,.2}", amount);
    // 출력: ₩1,234,567.89
}
```

### 패턴 2: 시간 표시

```rust
fn main() {
    let hours = 2;
    let minutes = 5;
    let seconds = 30;
    
    println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    // 출력: 02:05:30
}
```

### 패턴 3: 디버깅 정보

```rust
fn main() {
    let x = 42;
    let name = "test";
    
    println!("DEBUG: x={}, name={}", x, name);
    // 또는 더 간단히:
    println!("DEBUG: x={:?}, name={:?}", x, name);
}
```

---

## 📌 핵심 정리

```rust
println!("{포맷문자열}", 데이터1, 데이터2, ...);

포맷 문자열:
- "{}" = 순서대로 삽입
- "{0} {1}" = 인덱스로 지정
- "{name}" = 이름으로 지정

포맷 옵션:
- "{:10}" = 너비
- "{:.2}" = 소수점
- "{:>10}" = 오른쪽 정렬
- "{:x}" = 16진수
```

이제 Rust의 문자열 포맷팅이 완벽하게 이해되셨을 거예요! 🦀