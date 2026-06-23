# Rust 변수 범위, 속성, 패닉

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

## 2️⃣ 속성 (Attributes)

### 개념

**속성은 변수나 함수 위에 선언하여 컴파일러 동작을 제어하는 메타데이터입니다.**

### 사용 미사용 변수 경고 제어

#### deny: 거부 (에러 표시)

```rust
fn main() {
    #[deny(unused_variables)]
    let x = 1;  // ❌ 에러! 사용하지 않은 변수
    
    println!("프로그램 실행");
}
```

**결과:**
```
error: unused variable: `x`
  |
  | let x = 1;
  |     ^ help: if this is intentional, consider using `_x`
```

#### warn: 경고 (기본값)

```rust
fn main() {
    #[warn(unused_variables)]
    let y = 11;  // ⚠️ 경고! 하지만 컴파일은 됨
    
    println!("프로그램 실행");
}
```

**결과:**
```
warning: unused variable: `y`
  |
  | let y = 11;
  |     ^ help: if this is intentional, consider using `_y`

warning: `hello` (bin "hello") generated 1 warning
```

#### allow: 허용

```rust
fn main() {
    #[allow(unused_variables)]
    let z = 111;  // ✅ 경고 없음
    
    println!("프로그램 실행");
}
```

**결과:**
```
프로그램 실행
(경고 없음)
```

### 함수 위에 속성 선언

**함수 내의 모든 변수에 속성이 적용됩니다.**

```rust
#[allow(unused_variables)]
fn my_function() {
    let a = 1;  // ✅ 경고 없음
    let b = 2;  // ✅ 경고 없음
    let c = 3;  // ✅ 경고 없음
}

fn main() {
    my_function();
}
```

### 자주 사용하는 속성들

| 속성 | 설명 |
|------|------|
| `#[allow(unused_variables)]` | 미사용 변수 허용 |
| `#[warn(unused_variables)]` | 미사용 변수 경고 (기본) |
| `#[deny(unused_variables)]` | 미사용 변수 에러 |
| `#[allow(dead_code)]` | 미사용 함수/코드 허용 |
| `#[deprecated]` | 함수 폐기 예정 표시 |
| `#[must_use]` | 반환값 반드시 사용 필요 |

### 속성 예제

```rust
#[deny(unused_variables)]
fn strict_function() {
    let x = 5;   // ❌ 에러!
    let _y = 10; // ✅ OK (언더스코어로 의도 표시)
    
    println!("{}", _y);
}

#[allow(unused_variables)]
fn lenient_function() {
    let a = 1;   // ✅ OK
    let b = 2;   // ✅ OK
    let c = 3;   // ✅ OK
}

fn main() {
    lenient_function();
}
```

---

## 3️⃣ 패닉 (Panic)

### 개념

**Panic은 프로그램이 복구 불가능한 에러를 만났을 때 발생합니다.**

- 프로그램이 즉시 중단됨
- 에러 메시지 출력 후 종료
- 대부분 범위 초과 접근 시 발생

### 패닉 발생 예제

```rust
fn main() {
    let x = ["a"];  // 길이 1인 배열
    let y = x[1];   // ❌ 패닉! 인덱스 범위 초과
    
    println!("{}", y);
}
```

**에러 메시지:**
```
thread 'main' panicked at 'index out of bounds: 
the len is 1 but the index is 1', src/main.rs:3:17
note: run with `RUST_BACKTRACE=1` environment variable 
to get a backtrace for more info.
```

### 다양한 패닉 원인

#### 1. 배열 인덱스 범위 초과

```rust
fn main() {
    let arr = [1, 2, 3];
    println!("{}", arr[5]);  // ❌ 패닉!
}
```

#### 2. 벡터 인덱스 범위 초과

```rust
fn main() {
    let vec = vec![1, 2, 3];
    println!("{}", vec[10]);  // ❌ 패닉!
}
```

#### 3. 명시적 패닉 호출

```rust
fn main() {
    panic!("프로그램을 강제 중단합니다!");
}
```

#### 4. unwrap() 실패

```rust
fn main() {
    let result: Option<i32> = None;
    let value = result.unwrap();  // ❌ 패닉!
}
```

### 패닉 경고 제어

#### 조건부 패닉 경고 활성화

```rust
#[warn(unconditional_panic)]
fn main() {
    let x = [1, 2, 3];
    let y = x[10];  // ⚠️ 경고 표시 (컴파일 타임에 감지)
}
```

#### 패닉 경고 비활성화

```rust
#[allow(unconditional_panic)]
fn main() {
    let x = [1, 2, 3];
    let y = x[10];  // ✅ 경고 없음 (하지만 런타임에 패닉)
}
```

### 안전한 접근: get() 메서드

**패닉 대신 Option 반환:**

```rust
fn main() {
    let arr = [1, 2, 3];
    
    // 안전한 접근
    match arr.get(5) {
        Some(value) => println!("값: {}", value),
        None => println!("❌ 인덱스가 범위를 초과했습니다"),
    }
    // 출력: ❌ 인덱스가 범위를 초과했습니다
}
```

### 패닉 vs 에러

| 항목 | 패닉 | 에러 |
|------|------|------|
| **복구** | ❌ 불가능 | ✅ 가능 |
| **발생** | 런타임 | 컴파일 또는 런타임 |
| **처리** | 프로그램 중단 | Result로 처리 |
| **사용** | 예상 불가능한 상황 | 예상 가능한 상황 |

---

## 📌 핵심 정리

```rust
// Scope: 변수가 유효한 영역
{
    let x = 5;  // scope 시작
    println!("{}", x);  // 가능
}  // scope 종료, x 메모리 해제

// 속성: 컴파일러 동작 제어
#[allow(unused_variables)]
let y = 10;  // 경고 무시

#[deny(unused_variables)]
let z = 20;  // 에러 표시

// 패닉: 복구 불가능한 에러
let arr = [1, 2, 3];
let val = arr[10];  // ❌ 패닉!

// 안전한 접근
match arr.get(10) {
    Some(v) => println!("{}", v),
    None => println!("범위 초과"),
}
```

---

## 🎯 체크리스트

- [ ] Scope 개념 이해
- [ ] 변수의 생명주기 이해
- [ ] 속성(Attributes)의 용도 이해
- [ ] deny/warn/allow 차이 구분
- [ ] 패닉 발생 원인 알기
- [ ] 패닉 처리 방법 알기
- [ ] get() 메서드로 안전한 접근

이제 변수 범위, 속성, 패닉이 완벽하게 이해되셨을 거예요! 🦀