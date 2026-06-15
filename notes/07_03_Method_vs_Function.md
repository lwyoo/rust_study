# 메서드 vs 함수 (Method vs Function)

Rust는 **객체 지향 스타일**과 **함수형 스타일** 모두 지원합니다.

---

## 1️⃣ 메서드 문법 (Method) - 권장 ⭐

**`. (점)` 연산자를 사용하는 객체 지향 스타일**

```rust
fn main() {
    let mut l = 0;
    let s = "abcd";
    l = s.len();  // 메서드: 객체.메서드()
    println!("{}의 len: {}", s, l);  // 출력: abcd의 len: 4
}
```

---

## 2️⃣ 함수 문법 (Function) - 절차 지향

**`::`를 사용하여 명시적으로 함수 호출**

```rust
fn main() {
    let mut l = 0;
    let s = "abcd";
    l = str::len(s);  // 함수: 타입::함수(값)
    println!("{}의 len: {}", s, l);  // 출력: abcd의 len: 4
}
```

---

## 📊 비교표

| 항목 | 메서드 (`.`) | 함수 (`::`) |
|------|-----------|-----------|
| **문법** | `s.len()` | `str::len(s)` |
| **스타일** | 객체 지향 | 함수형/절차 지향 |
| **가독성** | ⭐⭐⭐⭐⭐ 더 직관적 | ⭐⭐⭐ 덜 직관적 |
| **권장도** | ✅ **권장** | ❌ 거의 사용 안 함 |
| **Rust 표준** | 메서드 호출이 표준 | 대체 방법 |

---

## 💡 주요 개념

### 메서드 (Method)
```rust
// 문자열 메서드들
let s = "hello";
s.len();           // 길이
s.to_uppercase();  // 대문자 변환
s.contains("ll");  // 포함 여부
```

### 함수 (Function)
```rust
// 타입::함수 형식 (잘 안 쓰임)
str::len(s);
Vec::new();  // 주로 생성자에 사용
```

---

## 🎯 실무 예제

```rust
fn main() {
    let text = "Rust Programming";
    
    // ✅ 메서드 문법 (권장)
    println!("길이: {}", text.len());
    println!("소문자: {}", text.to_lowercase());
    println!("시작 확인: {}", text.starts_with("Rust"));
    
    // ❌ 함수 문법 (거의 사용 안 함)
    // println!("{}", str::len(text));
}
```

**출력:**
```
길이: 16
소문자: rust programming
시작 확인: true
```

---

## 📌 결론

✅ **메서드(`.`)를 항상 사용하세요!**
- Rust의 표준 방식
- 더 읽기 쉽고 직관적
- IDE의 자동완성 지원 우수

❌ **함수(`::`) 문법은:**
- Rust에서 거의 사용되지 않음
- 생성자(`Vec::new()`) 정도만 사용
- 알아만 두면 됨