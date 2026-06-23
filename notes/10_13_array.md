# Rust 배열 (Array)

---

## 1️⃣ 배열 (Array) - 고정 크기

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
    println!("{}", arr[5]);   // ❌ 패닉! (범위 초과)
}
```

---

## 2️⃣ 가변 배열 (Mutable Array)

### 개념

**`mut` 키워드를 사용하여 배열의 요소를 수정할 수 있습니다.**

### 기본 예제

```rust
fn main() {
    let mut x = [1, 2, 3, 4, 5];
    
    // 원래 값 출력
    println!("{}, {}", x[0], x[1]);  // 1, 2
    
    // 요소 수정
    x[0] = 10;
    x[1] = 20;
    
    // 수정된 값 출력
    println!("{}, {}", x[0], x[1]);  // 10, 20
}
```

**출력:**
```
1, 2
10, 20
```

### 가변 배열 제약 조건

#### ❌ 데이터 타입이 다르면 안 됨

```rust
fn main() {
    let mut arr: [i32; 3] = [1, 2, 3];
    
    arr[0] = 10;     // ✅ 가능 (i32)
    arr[1] = 20;     // ✅ 가능 (i32)
    arr[2] = "hello";  // ❌ 에러! (문자열은 i32 아님)
}
```

#### ❌ 배열 크기는 변경 불가

```rust
fn main() {
    let mut arr = [1, 2, 3];  // 크기 3
    
    arr.push(4);  // ❌ 에러! 배열은 크기 고정
    // (벡터를 사용해야 함)
}
```

### 가변 배열 예제

```rust
fn main() {
    let mut scores = [70, 80, 90];
    
    println!("원래 점수: {:?}", scores);  // [70, 80, 90]
    
    // 각 점수를 10점씩 올리기
    scores[0] += 10;
    scores[1] += 10;
    scores[2] += 10;
    
    println!("올린 후: {:?}", scores);  // [80, 90, 100]
}
```

---

## 3️⃣ 배열 크기 명시적 지정

### 문법

```rust
let 배열변수명 = [데이터; 크기];
```

### 기본 예제

```rust
fn main() {
    let x = [10; 5];
    println!("{:?}", x);  // [10, 10, 10, 10, 10]
}
```

**동작:**
- `10` = 초기값
- `5` = 반복 횟수
- 결과: 10을 5번 반복

### 다양한 타입 예제

```rust
fn main() {
    // i32 배열
    let int_arr = [0; 4];
    println!("정수: {:?}", int_arr);  // [0, 0, 0, 0]
    
    // f64 배열
    let float_arr = [1.5; 3];
    println!("실수: {:?}", float_arr);  // [1.5, 1.5, 1.5]
    
    // bool 배열
    let bool_arr = [true; 2];
    println!("불린: {:?}", bool_arr);  // [true, true]
    
    // 문자 배열
    let char_arr = ['a'; 3];
    println!("문자: {:?}", char_arr);  // ['a', 'a', 'a']
}
```

**출력:**
```
정수: [0, 0, 0, 0]
실수: [1.5, 1.5, 1.5]
불린: [true, true]
문자: ['a', 'a', 'a']
```

### 크기 지정 예제

```rust
fn main() {
    // 0으로 초기화된 10개 요소
    let zeros = [0; 10];
    println!("길이: {}", zeros.len());  // 10
    
    // false로 초기화된 5개 요소
    let flags = [false; 5];
    println!("길이: {}", flags.len());  // 5
}
```

---

## 4️⃣ 다차원 배열 (2D Array)

### 개념

**배열 안에 또 다른 배열이 들어 있는 구조입니다.**

### 기본 문법

```rust
let array: [[타입; 열]; 행] = [[...], [...], [...]];
```

### 2차원 배열 예제

```rust
fn main() {
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    
    println!("{:?}", matrix);
    // [[1, 2, 3], [4, 5, 6]]
}
```

**구조:**
```
행(row) ↓
[[1, 2, 3],  ← 0번 행
 [4, 5, 6]]  ← 1번 행
  ↑ ↑ ↑
  열(column)
  0 1 2
```

### 2D 배열 접근

```rust
fn main() {
    let grid = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    
    // 첫 번째 행의 두 번째 요소 (2)
    println!("{}", grid[0][1]);  // 2
    
    // 세 번째 행의 첫 번째 요소 (7)
    println!("{}", grid[2][0]);  // 7
    
    // 전체 출력
    println!("{:?}", grid);
}
```

**출력:**
```
2
7
[[1, 2, 3], [4, 5, 6], [7, 8, 9]]
```

### 2D 배열 순회

```rust
fn main() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    
    // 각 행을 순회
    for row in matrix {
        println!("행: {:?}", row);
        
        // 각 요소를 순회
        for value in row {
            println!("  값: {}", value);
        }
    }
}
```

**출력:**
```
행: [1, 2, 3]
  값: 1
  값: 2
  값: 3
행: [4, 5, 6]
  값: 4
  값: 5
  값: 6
```

### 2D 배열 가변 (수정 가능)

```rust
fn main() {
    let mut grid: [[i32; 2]; 2] = [
        [1, 2],
        [3, 4],
    ];
    
    println!("원래: {:?}", grid);
    // [[1, 2], [3, 4]]
    
    // 요소 수정
    grid[0][0] = 10;
    grid[1][1] = 40;
    
    println!("수정 후: {:?}", grid);
    // [[10, 2], [3, 40]]
}
```

### 3차원 배열

```rust
fn main() {
    let cube: [[[i32; 2]; 2]; 2] = [
        [
            [1, 2],
            [3, 4],
        ],
        [
            [5, 6],
            [7, 8],
        ],
    ];
    
    // [깊이][행][열]
    println!("{}", cube[0][0][0]);  // 1
    println!("{}", cube[1][1][1]);  // 8
}
```

---

## 5️⃣ 실무 예제

### 예제 1: 성적표 (2D 배열)

```rust
fn main() {
    // [학생][과목] = 점수
    let grades: [[i32; 3]; 4] = [
        [85, 90, 88],  // 학생 1: 국어, 영어, 수학
        [92, 85, 90],  // 학생 2
        [78, 92, 85],  // 학생 3
        [88, 88, 92],  // 학생 4
    ];
    
    // 각 학생의 총점 계산
    for (student_idx, scores) in grades.iter().enumerate() {
        let total: i32 = scores.iter().sum();
        let average = total / scores.len() as i32;
        println!("학생 {}: 총점 {}, 평균 {}", student_idx + 1, total, average);
    }
}
```

**출력:**
```
학생 1: 총점 263, 평균 87
학생 2: 총점 267, 평균 89
학생 3: 총점 255, 평균 85
학생 4: 총점 268, 평균 89
```

### 예제 2: 이미지 픽셀 (2D 배열)

```rust
fn main() {
    // 8x8 픽셀 이미지 (1 = 검정, 0 = 흰색)
    let image: [[u8; 8]; 8] = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 1, 0, 0, 1],
        [1, 0, 0, 1, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];
    
    // 이미지 출력
    for row in image {
        for pixel in row {
            print!("{}", if pixel == 1 { "█" } else { " " });
        }
        println!();
    }
}
```

**출력:**
```
████████
█      █
█      █
█  ██  █
█  ██  █
█      █
█      █
████████
```

### 예제 3: 초기값 반복으로 배열 생성

```rust
fn main() {
    // 3x3 게임판 (0 = 빈칸, 1 = X, 2 = O)
    let mut board: [[i32; 3]; 3] = [[0; 3]; 3];
    
    println!("초기 게임판:");
    println!("{:?}", board);
    // [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
    
    // 게임진행
    board[0][0] = 1;  // X
    board[1][1] = 2;  // O
    board[0][2] = 1;  // X
    
    println!("\n게임 진행 후:");
    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            let symbol = match cell {
                0 => "·",
                1 => "X",
                2 => "O",
                _ => "?",
            };
            print!("{} ", symbol);
        }
        println!();
    }
}
```

**출력:**
```
초기 게임판:
[[0, 0, 0], [0, 0, 0], [0, 0, 0]]

게임 진행 후:
X · X 
· O · 
· · · 
```

---

## 📊 배열 타입별 크기

```rust
fn main() {
    let arr_i8: [i8; 4] = [1, 2, 3, 4];      // 4 bytes
    let arr_i32: [i32; 4] = [1, 2, 3, 4];    // 16 bytes
    let arr_i64: [i64; 4] = [1, 2, 3, 4];    // 32 bytes
    let arr_bool: [bool; 8] = [true; 8];     // 8 bytes
    let arr_char: [char; 4] = ['a'; 4];      // 16 bytes
    
    println!("i8[4]: {} bytes", std::mem::size_of_val(&arr_i8));
    println!("i32[4]: {} bytes", std::mem::size_of_val(&arr_i32));
    println!("i64[4]: {} bytes", std::mem::size_of_val(&arr_i64));
    println!("bool[8]: {} bytes", std::mem::size_of_val(&arr_bool));
    println!("char[4]: {} bytes", std::mem::size_of_val(&arr_char));
}
```

---

## 📌 핵심 정리

```rust
// 기본 배열
let arr: [i32; 5] = [1, 2, 3, 4, 5];

// 가변 배열
let mut arr = [1, 2, 3];
arr[0] = 10;

// 초기값 반복
let arr = [0; 5];  // [0, 0, 0, 0, 0]

// 2D 배열
let matrix: [[i32; 3]; 2] = [
    [1, 2, 3],
    [4, 5, 6],
];

// 2D 배열 접근
println!("{}", matrix[0][1]);  // 2

// 2D 배열 순회
for row in matrix {
    for value in row {
        println!("{}", value);
    }
}
```

---

## 🎯 배열 선택 기준

| 상황 | 배열 | 벡터 |
|------|------|------|
| **크기 고정** | ✅ 추천 | ❌ 과함 |
| **크기 변함** | ❌ 불가능 | ✅ 추천 |
| **속도 중요** | ✅ 추천 | ❌ 느림 |
| **2D 데이터** | ✅ 추천 | ⚠️ 복잡 |
| **이미지/게임** | ✅ 추천 | ❌ |

이제 Rust 배열이 완벽하게 이해되셨을 거예요! 🦀