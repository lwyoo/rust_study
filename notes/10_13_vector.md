# Rust 벡터 (Vector)

---

## 1️⃣ 벡터 (Vector) - 가변 크기

### 특징

| 항목 | 설명 |
|------|------|
| **크기** | ✅ 가변 (런타임에 변경 가능) |
| **타입** | ✅ 모두 같은 타입 |
| **속도** | ⭐⭐⭐ 느림 (힙에 할당) |
| **메모리** | 힙에 할당 |
| **유연성** | ⭐⭐⭐⭐⭐ 높음 |

### 벡터 vs 배열

| 항목 | 배열 | 벡터 |
|------|------|------|
| **크기** | 고정 | 가변 |
| **선언** | `[1, 2, 3]` | `vec![1, 2, 3]` |
| **타입** | `[i32; 3]` | `Vec<i32>` |
| **할당** | 스택 | 힙 |
| **속도** | ⭐⭐⭐⭐⭐ 빠름 | ⭐⭐⭐ 느림 |
| **유연성** | ❌ 낮음 | ⭐⭐⭐⭐⭐ 높음 |

---

## 2️⃣ 벡터 선언 방법

### 방법 1: 타입 명시

```rust
fn main() {
    let mut vec: Vec<i32> = Vec::new();
    println!("{:?}", vec);  // []
}
```

### 방법 2: vec! 매크로 (권장)

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);  // [1, 2, 3, 4, 5]
}
```

### 방법 3: 초기값 반복

```rust
fn main() {
    let vec = vec![0; 5];  // 0을 5번 반복
    // vec = [0, 0, 0, 0, 0]
    println!("{:?}", vec);
}
```

---

## 3️⃣ 벡터 기본 조작

### 요소 추가 (push)

```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    
    vec.push(4);
    vec.push(5);
    
    println!("{:?}", vec);  // [1, 2, 3, 4, 5]
}
```

### 요소 제거 (pop)

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    let removed = vec.pop();
    
    println!("남은 벡터: {:?}", vec);      // [1, 2, 3, 4]
    println!("제거된 값: {:?}", removed);  // Some(5)
}
```

### 길이 확인 (len)

```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    println!("길이: {}", vec.len());  // 5
}
```

### 용량 확인 (capacity)

```rust
fn main() {
    let mut vec = Vec::new();
    
    println!("초기 용량: {}", vec.capacity());  // 0
    
    vec.push(1);
    vec.push(2);
    println!("2개 후 용량: {}", vec.capacity());
    
    // 벡터는 필요할 때마다 자동으로 메모리 확장
}
```

---

## 4️⃣ 벡터 접근

### 인덱싱

```rust
fn main() {
    let vec = vec![10, 20, 30, 40, 50];
    
    println!("첫 번째: {}", vec[0]);  // 10
    println!("세 번째: {}", vec[2]);  // 30
}
```

### get() 메서드 (안전함)

```rust
fn main() {
    let vec = vec![10, 20, 30, 40, 50];
    
    // 존재하는 인덱스
    match vec.get(2) {
        Some(value) => println!("값: {}", value),  // 30
        None => println!("없음"),
    }
    
    // 존재하지 않는 인덱스
    match vec.get(10) {
        Some(value) => println!("값: {}", value),
        None => println!("인덱스 10은 없습니다"),  // 이것이 출력됨
    }
}
```

### ❌ 범위 초과 (패닉)

```rust
fn main() {
    let vec = vec![1, 2, 3];
    
    println!("{}", vec[10]);  // ❌ 패닉! (범위 초과)
}
```

---

## 5️⃣ 벡터 순회

### 값으로 순회

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    for num in &numbers {
        println!("{}", num);
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

### 인덱스와 함께 순회

```rust
fn main() {
    let items = vec!["a", "b", "c", "d"];
    
    for (idx, item) in items.iter().enumerate() {
        println!("[{}] = {}", idx, item);
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

### 가변 참조로 순회 (수정)

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    for num in &mut vec {
        *num *= 2;  // 각 요소를 2배로
    }
    
    println!("{:?}", vec);  // [2, 4, 6, 8, 10]
}
```

### 소유권으로 순회 (소비)

```rust
fn main() {
    let vec = vec![1, 2, 3];
    
    for num in vec {  // 소유권 이동
        println!("{}", num);
    }
    
    println!("{:?}", vec);  // ❌ 에러! vec는 더 이상 없음
}
```

---

## 6️⃣ 유용한 벡터 메서드들

### 정렬 (sort)

```rust
fn main() {
    let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    vec.sort();
    println!("정렬: {:?}", vec);  // [1, 1, 2, 3, 4, 5, 6, 9]
}
```

### 역순 (reverse)

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    vec.reverse();
    println!("역순: {:?}", vec);  // [5, 4, 3, 2, 1]
}
```

### 중복 제거 (dedup)

```rust
fn main() {
    let mut vec = vec![1, 1, 2, 2, 3, 3];
    
    vec.dedup();
    println!("중복 제거: {:?}", vec);  // [1, 2, 3]
}
```

### 포함 여부 (contains)

```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    println!("3 포함? {}", vec.contains(&3));  // true
    println!("10 포함? {}", vec.contains(&10));  // false
}
```

### 위치 찾기 (position)

```rust
fn main() {
    let vec = vec![10, 20, 30, 40, 50];
    
    match vec.iter().position(|&x| x == 30) {
        Some(idx) => println!("30의 위치: {}", idx),  // 2
        None => println!("찾지 못함"),
    }
}
```

### 합계 (sum)

```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    let sum: i32 = vec.iter().sum();
    println!("합계: {}", sum);  // 15
}
```

### 최대값 (max)

```rust
fn main() {
    let vec = vec![3, 7, 2, 9, 1];
    
    if let Some(max) = vec.iter().max() {
        println!("최대값: {}", max);  // 9
    }
}
```

### 최소값 (min)

```rust
fn main() {
    let vec = vec![3, 7, 2, 9, 1];
    
    if let Some(min) = vec.iter().min() {
        println!("최소값: {}", min);  // 1
    }
}
```

---

## 7️⃣ 벡터 요소 수정/제거

### 특정 위치 삽입 (insert)

```rust
fn main() {
    let mut vec = vec![1, 2, 4, 5];
    
    vec.insert(2, 3);  // 인덱스 2에 3 삽입
    println!("{:?}", vec);  // [1, 2, 3, 4, 5]
}
```

### 특정 위치 제거 (remove)

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    let removed = vec.remove(2);  // 인덱스 2 제거
    println!("제거된 값: {}", removed);  // 3
    println!("남은 벡터: {:?}", vec);  // [1, 2, 4, 5]
}
```

### 모든 요소 제거 (clear)

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    vec.clear();
    println!("{:?}", vec);     // []
    println!("길이: {}", vec.len());  // 0
}
```

---

## 💼 실무 예제

### 예제 1: 학생 점수 관리

```rust
fn main() {
    let mut scores = Vec::new();
    
    // 점수 입력
    scores.push(85);
    scores.push(92);
    scores.push(78);
    scores.push(95);
    
    println!("전체 점수: {:?}", scores);
    
    // 평균 계산
    let sum: i32 = scores.iter().sum();
    let average = sum / scores.len() as i32;
    println!("평균: {}", average);
    
    // 최고 점수
    if let Some(max) = scores.iter().max() {
        println!("최고 점수: {}", max);
    }
    
    // 정렬
    scores.sort();
    println!("정렬된 점수: {:?}", scores);
}
```

**출력:**
```
전체 점수: [85, 92, 78, 95]
평균: 87
최고 점수: 95
정렬된 점수: [78, 85, 92, 95]
```

### 예제 2: 동적 리스트 관리

```rust
fn main() {
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
    println!("\n남은 항목: {}", todo_list.len());
    for (idx, task) in todo_list.iter().enumerate() {
        println!("{}. {}", idx + 1, task);
    }
}
```

**출력:**
```
할 일 목록:
1. 공부하기
2. 운동하기
3. 쇼핑하기

남은 항목: 2
1. 운동하기
2. 쇼핑하기
```

### 예제 3: 사용자 입력 수집

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
        let avg = sum as f64 / numbers.len() as f64;
        println!("\n합계: {}", sum);
        println!("평균: {:.2}", avg);
    }
}
```

### 예제 4: 데이터 필터링

```rust
fn main() {
    let scores = vec![45, 78, 92, 55, 88, 62, 95, 75];
    
    // 70점 이상인 점수만 필터링
    let passing: Vec<i32> = scores
        .iter()
        .filter(|&&score| score >= 70)
        .copied()
        .collect();
    
    println!("전체 점수: {:?}", scores);
    println!("합격한 점수: {:?}", passing);
    println!("합격율: {:.1}%", 
             passing.len() as f64 / scores.len() as f64 * 100.0);
}
```

**출력:**
```
전체 점수: [45, 78, 92, 55, 88, 62, 95, 75]
합격한 점수: [78, 92, 88, 95, 75]
합격율: 62.5%
```

### 예제 5: 문자열 벡터

```rust
fn main() {
    let mut fruits = vec!["apple", "banana", "cherry"];
    
    println!("과일 목록:");
    for fruit in &fruits {
        println!("- {}", fruit);
    }
    
    // 새로운 과일 추가
    fruits.push("date");
    fruits.push("elderberry");
    
    println!("\n추가 후:");
    for (idx, fruit) in fruits.iter().enumerate() {
        println!("{}. {}", idx + 1, fruit);
    }
}
```

**출력:**
```
과일 목록:
- apple
- banana
- cherry

추가 후:
1. apple
2. banana
3. cherry
4. date
5. elderberry
```

---

## 📊 벡터 메모리 관리

### 용량 vs 길이

```rust
fn main() {
    let mut vec = Vec::new();
    
    println!("초기 - 길이: {}, 용량: {}", vec.len(), vec.capacity());
    
    for i in 0..10 {
        vec.push(i);
        if i == 0 || i == 3 || i == 7 || i == 9 {
            println!("{}개 추가 후 - 길이: {}, 용량: {}", 
                     i + 1, vec.len(), vec.capacity());
        }
    }
}
```

**설명:**
- **길이 (len)**: 실제 요소 개수
- **용량 (capacity)**: 할당된 메모리 공간
- 벡터는 필요시 자동으로 메모리 2배 확장

---

## 📌 핵심 정리

```rust
// 벡터 선언
let mut vec = vec![1, 2, 3];

// 요소 추가
vec.push(4);

// 요소 제거
vec.pop();

// 길이
println!("{}", vec.len());

// 접근
println!("{}", vec[0]);

// 순회
for num in &vec {
    println!("{}", num);
}

// 정렬
vec.sort();

// 합계
let sum: i32 = vec.iter().sum();
```

---

## 🎯 벡터 선택 기준

| 상황 | 배열 | 벡터 |
|------|------|------|
| **크기 고정** | ✅ 추천 | ❌ 과함 |
| **크기 변함** | ❌ 불가능 | ✅ 추천 |
| **속도 중요** | ✅ 추천 | ❌ 느림 |
| **유연성** | ❌ 낮음 | ✅⭐⭐⭐⭐⭐ |
| **사용자 입력** | ❌ | ✅ 추천 |
| **동적 수집** | ❌ | ✅ 추천 |

이제 Rust 벡터가 완벽하게 이해되셨을 거예요! 🦀