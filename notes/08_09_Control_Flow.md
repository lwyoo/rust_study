# Rust 제어문 (Control Flow)

---

## 1️⃣ if 문 (조건문)

### 언제 사용하는가?
- **특정 조건에 따라 다른 코드를 실행**
- 조건이 참/거짓에 따라 분기

### 기본 구조

#### 형태 1: 일반 if 문 (명령문)

```rust
fn main() {
    let a = 15;
    let mut n: i32;
    
    if a > 10 {
        n = a % 3;  // 조건 참일 때
    } else {
        n = a % 2;  // 조건 거짓일 때
    }
    println!("n = {}", n);  // 출력: n = 0
}
```

#### 형태 2: if를 표현식으로 사용 (값 반환)

```rust
fn main() {
    let a = 15;
    
    // if 표현식으로 직접 값 할당
    let n = if a > 10 {
        a % 3       // 세미콜론 없음! (값)
    } else {
        a % 2       // 세미콜론 없음! (값)
    };
    println!("n = {}", n);  // 출력: n = 0
}
```

### if-else if-else 체인

```rust
fn main() {
    let score = 85;
    
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "F"
    };
    
    println!("성적: {}", grade);  // 출력: 성적: B
}
```

### ⚠️ 주의: 삼항 연산자는 없음!

```rust
// ❌ C/C++ 스타일 삼항 연산자 없음
// let result = (a > 10) ? a % 3 : a % 2;  // 에러!

// ✅ Rust: if 표현식 사용
let result = if a > 10 { a % 3 } else { a % 2 };
```

---

## 2️⃣ while 문 (조건 반복)

### 언제 사용하는가?
- **조건이 참인 동안 반복 실행**
- 반복 횟수가 명확하지 않을 때
- 특정 조건을 만족할 때까지 반복

### 기본 구조

```rust
fn main() {
    let mut n = 0;
    
    while n < 5 {
        println!("n = {}", n);
        n += 1;  // 증감 필수! (무한 루프 방지)
    }
}
```

**출력:**
```
n = 0
n = 1
n = 2
n = 3
n = 4
```

### while 예제 - 사용자 입력 처리

```rust
use std::io;

fn main() {
    let mut input = String::new();
    
    while input.trim() != "quit" {
        println!("명령어를 입력하세요 (quit 입력시 종료):");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        println!("입력: {}", input.trim());
    }
    
    println!("프로그램 종료");
}
```

---

## 3️⃣ loop 문 (무한 루프)

### 언제 사용하는가?
- **명확한 종료 조건이 있는 무한 루프**
- `break`로 명시적으로 빠져나감
- `while true`보다 더 명확한 의도 표현

### 기본 구조

```rust
fn main() {
    let mut n = 0;
    
    loop {
        println!("n = {}", n);
        n += 1;
        
        if n >= 5 {
            break;  // 루프 탈출
        }
    }
}
```

**출력:**
```
n = 0
n = 1
n = 2
n = 3
n = 4
```

### loop 활용 - 값 반환

```rust
fn main() {
    let mut count = 0;
    
    // loop에서 break로 값 반환 가능
    let result = loop {
        count += 1;
        
        if count == 5 {
            break count * 2;  // 값과 함께 break
        }
    };
    
    println!("결과: {}", result);  // 출력: 결과: 10
}
```

### loop vs while true

```rust
// while true (읽기 불편)
while true {
    println!("반복");
    if condition {
        break;
    }
}

// loop (더 명확)
loop {
    println!("반복");
    if condition {
        break;
    }
}
```

---

## 4️⃣ for 문 (범위 반복)

### 언제 사용하는가?
- **정해진 범위만큼 반복**
- **컬렉션(벡터, 배열)의 각 요소 순회**
- 반복 횟수가 명확할 때

### 기본 구조

#### 형태 1: 범위(Range) 사용

```rust
fn main() {
    // 0부터 4까지 (5 미포함)
    for i in 0..5 {
        println!("i = {}", i);
    }
}
```

**출력:**
```
i = 0
i = 1
i = 2
i = 3
i = 4
```

#### 형태 2: 범위 포함 (..=)

```rust
fn main() {
    // 0부터 5까지 (5 포함)
    for i in 0..=5 {
        println!("i = {}", i);
    }
}
```

**출력:**
```
i = 0
i = 1
i = 2
i = 3
i = 4
i = 5
```

#### 형태 3: 벡터/배열 순회

```rust
fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    
    // 요소 순회
    for num in numbers {
        println!("num = {}", num);
    }
}
```

**출력:**
```
num = 10
num = 20
num = 30
num = 40
num = 50
```

#### 형태 4: 인덱스와 함께 순회

```rust
fn main() {
    let items = vec!["a", "b", "c", "d"];
    
    for (index, item) in items.iter().enumerate() {
        println!("[{}] = {}", index, item);
    }
}
```

**출력:**
```
[0] = a
[1] = b
[2] = c
[3] = d
```

#### 형태 5: 역순 순회

```rust
fn main() {
    for i in (0..5).rev() {
        println!("i = {}", i);
    }
}
```

**출력:**
```
i = 4
i = 3
i = 2
i = 1
i = 0
```

---

## 5️⃣ match 문 (패턴 매칭)

### 언제 사용하는가?
- **여러 경우의 수를 처리**
- **Switch 문 대체** (Rust는 switch 없음)
- **패턴 매칭으로 복잡한 조건 처리**

### 기본 구조

```rust
fn main() {
    let num = 2;
    
    match num {
        1 => println!("하나"),
        2 => println!("둘"),
        3 => println!("셋"),
        _ => println!("기타"),  // _ = 모든 경우
    }
}
```

**출력:**
```
둘
```

### match 표현식

```rust
fn main() {
    let num = 2;
    
    let text = match num {
        1 => "하나",
        2 => "둘",
        3 => "셋",
        _ => "기타",
    };
    
    println!("{}", text);  // 출력: 둘
}
```

### match with 범위

```rust
fn main() {
    let score = 85;
    
    let grade = match score {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        _        => "F",
    };
    
    println!("성적: {}", grade);  // 출력: 성적: B
}
```

---

## 6️⃣ break와 continue

### break (루프 탈출)

```rust
fn main() {
    for i in 0..10 {
        if i == 5 {
            break;  // 루프 즉시 종료
        }
        println!("i = {}", i);
    }
    println!("루프 종료");
}
```

**출력:**
```
i = 0
i = 1
i = 2
i = 3
i = 4
루프 종료
```

### continue (다음 반복으로)

```rust
fn main() {
    for i in 0..5 {
        if i == 2 {
            continue;  // i=2는 건너뛰고 다음으로
        }
        println!("i = {}", i);
    }
}
```

**출력:**
```
i = 0
i = 1
i = 3
i = 4
```

### 라벨과 함께 사용

```rust
fn main() {
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                break 'outer;  // 외부 루프 탈출
            }
            println!("({}, {})", i, j);
        }
    }
}
```

**출력:**
```
(0, 0)
(0, 1)
(0, 2)
(1, 0)
```

---

## 7️⃣ 제어문 비교표

| 제어문 | 용도 | 언제 사용 | 예시 |
|--------|------|---------|------|
| **if** | 조건 분기 | 조건 판단 | 성적 등급 판정 |
| **while** | 조건 반복 | 조건이 참인 동안 | 사용자 입력 대기 |
| **loop** | 무한 반복 | 명확한 탈출 조건 있을 때 | 게임 메인 루프 |
| **for** | 범위/컬렉션 반복 | 정해진 횟수 반복 | 배열 순회 |
| **match** | 패턴 매칭 | 여러 경우의 수 | 열거형 처리 |

---

## 💼 실무 예제

### 예제 1: 성적 처리 시스템

```rust
fn main() {
    let scores = vec![85, 92, 78, 95, 88];
    
    for (idx, score) in scores.iter().enumerate() {
        let grade = match score {
            90..=100 => "A",
            80..=89  => "B",
            70..=79  => "C",
            _        => "F",
        };
        
        println!("학생 {}: {} ({})", idx + 1, score, grade);
    }
}
```

**출력:**
```
학생 1: 85 (B)
학생 2: 92 (A)
학생 3: 78 (C)
학생 4: 95 (A)
학생 5: 88 (B)
```

### 예제 2: 유효성 검사

```rust
fn main() {
    let password = "Pass123!";
    let mut is_valid = true;
    
    // 조건 검사
    if password.len() < 8 {
        println!("❌ 8글자 이상이어야 합니다");
        is_valid = false;
    }
    
    if !password.chars().any(|c| c.is_uppercase()) {
        println!("❌ 대문자가 포함되어야 합니다");
        is_valid = false;
    }
    
    if !password.chars().any(|c| c.is_numeric()) {
        println!("❌ 숫자가 포함되어야 합니다");
        is_valid = false;
    }
    
    if is_valid {
        println!("✅ 비밀번호가 유효합니다");
    }
}
```

### 예제 3: 구구단 출력

```rust
fn main() {
    let dan = 3;
    
    println!("{}단", dan);
    for i in 1..=9 {
        println!("{} × {} = {}", dan, i, dan * i);
    }
}
```

**출력:**
```
3단
3 × 1 = 3
3 × 2 = 6
3 × 3 = 9
...
3 × 9 = 27
```

---

## 📌 핵심 정리

```rust
// if: 조건 분기
if condition { } else { }

// while: 조건 반복
while condition { }

// loop: 무한 반복
loop { break; }

// for: 범위/컬렉션 반복
for i in 0..5 { }
for item in collection { }

// match: 패턴 매칭
match value {
    pattern => { },
    _ => { },
}

// break: 루프 탈출
break;

// continue: 다음 반복으로
continue;
```

---

## 🎯 체크리스트

- [ ] if 문으로 조건 분기 가능
- [ ] if를 표현식으로 사용 가능
- [ ] while로 조건 반복 구현 가능
- [ ] loop로 무한 루프 구현 가능
- [ ] for로 범위 순회 가능
- [ ] match로 패턴 매칭 가능
- [ ] break/continue 이해
- [ ] 각 제어문의 사용 시기 구분 가능

이제 Rust의 모든 제어문을 마스터했습니다! 🦀