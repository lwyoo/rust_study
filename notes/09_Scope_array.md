# Rust 변수 범위, 배열, 벡터

---

## 1️⃣ 변수 범위 (Scope)

### 개념

**Scope는 변수가 유효한(사용 가능한) 영역입니다.**

- **선언한 블록 `{}` 내부**에서만 접근 가능
- 블록을 벗어나면 변수는 메모리에서 해제됨
- 같은 이름의 변수를 다른 블록에서 재선언 가능

### 기본 예제

```rust
fn main() {
    let x = 5;  // x의 scope 시작
    println!("{}", x);  // ✅ 가능 (5)
    
    {
        let y = 10;     // y의 scope 시작
        println!("{}", x);  // ✅ 가능 (부모 scope 접근)
        println!("{}", y);  // ✅ 가능 (10)
    }  // y의 scope 종료 → y 메모리 해제
    
    println!("{}", y);  // ❌ 에러! y는 scope 밖
}
```

### 함수 단위 Scope

```rust
fn main() {
    let x = 5;  // main 함수의 scope
    println!("{}", x);  // ✅ 가능
    
    my_function();
}

fn my_function() {
    println!("{}", x);  // ❌ 에러! x는 main 함수 내부에만 존재
}
```

### Scope와 메모리

```rust
fn main() {
    {
        let s = String::from("hello");
        println!("{}", s);  // hello
    }  // ← 여기서 s가 scope를 벗어나면서 자동으로 메모리 해제됨
    
    println!("{}", s);  // ❌ 에러! s는 더 이상 존재하지 않음
}
```

### Scope 시각화

```rust
fn main() {          // ← main scope 시작
    let a = 1;       // a scope: main 내부
    
    {                // ← 새로운 scope 시작
        let b = 2;   // b scope: 이 블록 내부
        let c = 3;   // c scope: 이 블록 내부
        
        println!("{} {} {}", a, b, c);  // ✅ 가능 (모두 접근 가능)
    }                // ← scope 종료: b, c 메모리 해제
    
    println!("{} {}", a, b);  // ❌ 에러! b는 더 이상 없음
}
```

---

## 2️⃣ 배열 (Array) - 고정 크기

### 특징

| 항목 | 설명 |
|------|------|
| **크기** | ✅ 고정 (컴파일 타임에 결정) |
| **타입** | ✅ 모두 같은 타입 |
| **속도** | ⭐⭐⭐⭐⭐ 빠름 (스택에 할당) |
| **메모리** | 스택에 할당 |
| **유연성** | ❌ 낮음 |

### 선언 방법

#### 방법 1: 타입과 크기 명시

```rust
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    //       ↑   ↑
    //      타입  크기
    
    println!("{:?}", arr);  // [1, 2, 3, 4, 5]
}
```

#### 방법 2: 자동 추론

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];  // 타입과 크기 자동 추론
    println!("{:?}", arr);
}
```

#### 방법 3: 초기값 반복

```rust
fn main() {
    let arr = [0; 5];  // 0을 5번 반복
    // arr = [0, 0, 0, 0, 0]
    println!("{:?}", arr);
}
```

### 배열 접근

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];
    
    // 인덱스로 접근
    println!("첫 번째: {}", numbers[0]);   // 10
    println!("두 번째: {}", numbers[1]);   // 20
    
    // 길이 확인
    println!("길이: {}", numbers.len());   // 5
}
```

### 배열 순회

```rust
fn main() {
    let scores = [85, 92, 78, 95, 88];
    
    // for 루프로 순회
    for score in scores {
        println!("점수: {}", score);
    }
}
```

### ❌ 배열 에러: 범위 초과

```rust
fn main() {
    let arr = [1, 2, 3];
    
    println!("{}", arr[0]);   // ✅ 가능
    println!("{}", arr[2]);   // ✅ 가능
    println!("{}", arr[5]);   // ❌ 에러! (범위 초과)
}
```

---

## 3️⃣ 벡터 (Vector) - 가변 크기

### 특징

| 항목 | 설명 |
|------|------|
| **크기** | ✅ 가변 (런타임에 변경 가능) |
| **타입** | ✅ 모두 같은 타입 |
| **속도** | ⭐⭐⭐ 느림 (힙에 할당) |
| **메모리** | 힙에 할당 |
| **유연성** | ⭐⭐⭐⭐⭐ 높음 |

### 선언 방법

#### 방법 1: 타입 명시

```rust
fn main() {
    let mut vec: Vec<i32> = Vec::new();
    println!("{:?}", vec);  // []
}
```

#### 방법 2: vec! 매크로

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);  // [1, 2, 3, 4, 5]
}
```

#### 방법 3: 초기값 반복

```rust
fn main() {
    let vec = vec![0; 5];  // 0을 5번 반복
    // vec = [0, 0, 0, 0, 0]
    println!("{:?}", vec);
}
```

### 벡터 조작

```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    
    // 요소 추가
    vec.push(4);
    println!("{:?}", vec);  // [1, 2, 3, 4]
    
    // 요소 제거
    let removed = vec.pop();
    println!("{:?}", vec);     // [1, 2, 3]
    println!("제거된 값: {:?}", removed);  // Some(4)
    
    // 길이
    println!("길이: {}", vec.len());  // 3
    
    // 용량
    println!("용량: {}", vec.capacity());
}
```

### 벡터 접근

```rust
fn main() {
    let mut vec = vec![10, 20, 30, 40, 50];
    
    // 인덱싱
    println!("첫 번째: {}", vec[0]);  // 10
    
    // get() 메서드 (안전함)
    match vec.get(2) {
        Some(value) => println!("값: {}", value),  // 30
        None => println!("없음"),
    }
    
    // 범위 초과 접근
    match vec.get(10) {
        Some(value) => println!("값: {}", value),
        None => println!("인덱스 10은 없습니다"),
    }
}
```

### 벡터 순회

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 값으로 순회
    for num in &numbers {
        println!("{}", num);
    }
    
    // 인덱스와 함께 순회
    for (idx, num) in numbers.iter().enumerate() {
        println!("[{}] = {}", idx, num);
    }
    
    // 가변 참조로 순회 (수정)
    let mut vec = vec![1, 2, 3];
    for num in &mut vec {
        *num *= 2;  // 각 요소를 2배로
    }
    println!("{:?}", vec);  // [2, 4, 6]
}
```

---

## 4️⃣ 배열 vs 벡터 비교

| 항목 | 배열 | 벡터 |
|------|------|------|
| **크기** | 고정 | 가변 |
| **선언** | `[1, 2, 3]` | `vec![1, 2, 3]` |
| **타입** | `[i32; 3]` | `Vec<i32>` |
| **할당** | 스택 | 힙 |
| **속도** | ⭐⭐⭐⭐⭐ 빠름 | ⭐⭐⭐ 느림 |
| **유연성** | ❌ 낮음 | ⭐⭐⭐⭐⭐ 높음 |
| **메모리** | 고정 | 동적 |
| **용도** | 크기 정해짐 | 크기 변함 |

---

## 5️⃣ 실무 예제

### 예제 1: 학생 점수 관리

```rust
fn main() {
    // 반의 학생이 5명으로 정해짐
    let class_scores: [i32; 5] = [85, 92, 78, 95, 88];
    
    let mut total = 0;
    for score in class_scores {
        total += score;
    }
    
    let average = total / class_scores.len() as i32;
    println!("평균: {}", average);  // 87
}
```

### 예제 2: 동적 리스트 관리

```rust
fn main() {
    // 처음에는 비어있고, 나중에 추가됨
    let mut todo_list = Vec::new();
    
    todo_list.push("공부하기");
    todo_list.push("운동하기");
    todo_list.push("쇼핑하기");
    
    println!("할 일 목록:");
    for (idx, task) in todo_list.iter().enumerate() {
        println!("{}. {}", idx + 1, task);
    }
    
    // 완료된 항목 제거
    todo_list.remove(0);
    println!("남은 항목: {}", todo_list.len());
}
```

**출력:**
```
할 일 목록:

공부하기
운동하기
쇼핑하기

남은 항목: 2
```

### 예제 3: 동적 데이터 수집

```rust
use std::io;

fn main() {
    let mut numbers = Vec::new();
    
    println!("숫자를 입력하세요 (끝내려면 'q' 입력):");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "q" => break,
            num_str => {
                if let Ok(num) = num_str.parse::<i32>() {
                    numbers.push(num);
                    println!("추가됨. 현재 개수: {}", numbers.len());
                }
            }
        }
    }
    
    if !numbers.is_empty() {
        let sum: i32 = numbers.iter().sum();
        println!("합계: {}", sum);
    }
}
```

### 예제 4: 이미지 픽셀 (배열이 적합)

```rust
fn main() {
    // 8x8 픽셀 이미지 (크기 고정)
    let image: [[u8; 8]; 8] = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];
    
    println!("이미지 데이터 크기: {} bytes", image.len() * image[0].len());
}
```

---

## 6️⃣ 유용한 벡터 메서드들

```rust
fn main() {
    let mut vec = vec![3, 1, 4, 1, 5, 9];
    
    // 정렬
    vec.sort();
    println!("정렬: {:?}", vec);  // [1, 1, 3, 4, 5, 9]
    
    // 역순
    vec.reverse();
    println!("역순: {:?}", vec);  // [9, 5, 4, 3, 1, 1]
    
    // 중복 제거
    vec.dedup();
    println!("중복 제거: {:?}", vec);  // [9, 5, 4, 3, 1]
    
    // 특정 요소 포함 여부
    println!("3 포함? {}", vec.contains(&3));  // true
    
    // 특정 요소 위치
    println!("3의 위치: {:?}", vec.iter().position(|&x| x == 3));  // Some(2)
}
```

---

## 📌 핵심 정리

```rust
// Scope: 변수가 유효한 영역
{
    let x = 5;  // scope 시작
    println!("{}", x);  // 가능
}  // scope 종료, x 메모리 해제

// 배열: 고정 크기 (빠름)
let arr: [i32; 5] = [1, 2, 3, 4, 5];

// 벡터: 가변 크기 (유연함)
let mut vec = vec![1, 2, 3];
vec.push(4);
```

---

## 🎯 언제 뭘 사용할까?

| 상황 | 배열 | 벡터 |
|------|------|------|
| **크기 고정** | ✅ 추천 | ❌ 과함 |
| **크기 변함** | ❌ 불가능 | ✅ 추천 |
| **속도 중요** | ✅ 추천 | ❌ 느림 |
| **유연성 중요** | ❌ 낮음 | ✅ 추천 |
| **이미지/고정 데이터** | ✅ 추천 | ❌ |
| **사용자 입력 수집** | ❌ | ✅ 추천 |
