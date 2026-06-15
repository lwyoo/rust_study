# Rust Shadowing (쉐도잉) 🎭

**쉐도잉**은 같은 이름의 변수를 재선언하면, 새 변수가 이전 변수를 **"가린다(shadow)"** 는 개념입니다.

---

## 기본 예제

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);  // 출력: x = 5
    
    let x = x + 1;  // 쉐도잉: 새로운 x가 이전 x를 가림
    println!("x = {}", x);  // 출력: x = 6
    
    {
        let x = x * 2;  // 새로운 스코프에서 또 다시 쉐도잉
        println!("x = {}", x);  // 출력: x = 12
    }
    
    println!("x = {}", x);  // 출력: x = 6 (스코프 밖이므로 이전 값)
}
```

**출력:**
```
x = 5
x = 6
x = 12
x = 6
```

---

## 🔥 Shadowing의 강력한 점: 타입 변경

```rust
fn main() {
    // 문자열을 숫자로 변환
    let spaces = "   ";
    let spaces: usize = spaces.len();  // ✅ 타입 변경 가능!
    println!("{}", spaces);  // 출력: 3
}
```

### vs Mutable 변수 (타입 변경 불가)
```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();  // ❌ 에러! 타입이 다름
}
```

---

## 📊 Shadowing vs Mutable 비교

| 특징 | Shadowing | Mutable (`let mut`) |
|------|-----------|-------------------|
| **재선언** | ✅ 새 변수 생성 | ❌ 같은 변수 수정 |
| **타입 변경** | ✅ 가능 | ❌ 불가능 |
| **메모리** | 새 메모리 할당 | 같은 메모리 수정 |
| **선언** | `let x = ...` | `let mut x = ...` |

---

## 실제 예제: 타입 변환

```rust
fn main() {
    // 사용자 입력 받기 (문자열)
    let input = "42";
    println!("문자열: {}", input);  // 출력: 문자열: 42
    
    // 같은 이름으로 숫자로 변환
    let input: i32 = input.parse().unwrap();
    println!("숫자: {}", input);  // 출력: 숫자: 42
    
    // 다시 쉐도잉하여 불린으로 변환
    let input = input > 30;
    println!("30보다 큼: {}", input);  // 출력: 30보다 큼: true
}
```

---

## ⚠️ 주의사항

### 1. 스코프에서만 유효
```rust
fn main() {
    let x = 5;
    
    {
        let x = 10;  // 이 블록 내에서만 10
    }
    
    println!("{}", x);  // 출력: 5 (스코프 밖이므로 원래 값)
}
```

### 2. 이전 변수의 값은 사라짐
```rust
fn main() {
    let x = 5;
    let x = x + 1;  // 이전의 5는 더 이상 접근 불가
    
    // 이전 x를 참조할 방법이 없음!
}
```

---

## 💡 언제 사용할까?

✅ **좋은 사용 사례:**
```rust
// 타입 변환이 필요할 때
let number_str = "42";
let number = number_str.parse::<i32>().unwrap();

// 데이터 처리 파이프라인
let data = load_data();
let data = process_data(data);
let data = format_data(data);
```

❌ **피해야 할 사용:**
```rust
// 단순히 값을 여러 번 수정하려면 mut 사용
let mut x = 0;
x = 1;
x = 2;  // 이 경우 let로 계속 쉐도잉하는 것보다 mut가 명확함
```

---

## 🎯 핵심 정리

| 개념 | 설명 |
|------|------|
| **Shadowing** | 같은 이름으로 재선언 → 새 변수 생성 |
| **Type Change** | 타입을 완전히 다르게 변경 가능 |
| **Scope** | 스코프를 벗어나면 이전 변수가 다시 유효 |
| **Use Case** | 타입 변환, 데이터 파이프라인 처리 |

궁금한 부분이 있으면 더 설명해드릴게요! 🦀