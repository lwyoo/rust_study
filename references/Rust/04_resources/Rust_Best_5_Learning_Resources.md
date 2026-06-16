# 🦀 Rust를 빠르게 배우기 위한 최고의 자료 5개
## 책 · 영상 · 강의 · 인물 중 엄선된 필독 자료

---

## 📍 자료 선정 기준

**Dogyeom의 상황을 고려:**
- 10년 C/C++ 경력 (문법 기초 있음)
- 빠르게 실무 능력 필요 (42dot 입사 준비)
- 자동차 소프트웨어 배경 (시스템 프로그래밍 경험)
- 학습 시간 제한적 (집중 학습 필요)

**선정 기준:**
1. 신뢰도 (공식, 업계 표준)
2. 실용성 (실제 코드 작성 능력)
3. 깊이 (이해도 vs 학습 시간 효율)
4. Dogyeom 맞춤성 (경력 고려)

---

## 🥇 #1: The Rust Book (공식)
**저자:** Stevie Klabnik & Carol Nichols  
**형식:** 온라인 무료 책 + 오프라인 구매 가능  
**URL:** https://doc.rust-lang.org/book/  
**추정 학습 시간:** 40-50시간

### ⭐ 왜 시간을 들을 가치가 있는가?

**공식 표준 문서**
- Rust 팀이 직접 관리하는 유일한 권위 있는 학습 자료
- 모든 Rust 개념이 체계적으로 정렬됨
- 새 버전마다 업데이트 (2024-2025년 최신)

**C/C++ 개발자를 위해 최적화됨**
```
Ch. 4: Understanding Ownership
  → C의 포인터, C++의 unique_ptr와 비교하면서 설명
  → 메모리 안전성이 어떻게 다른지 명확히 함

Ch. 10: Generic Types, Traits, and Lifetimes
  → C++ template과 비교
  → Rust의 더 엄격한 타입 시스템 이해
```

**구조가 완벽**
1. 기초 (변수, 타입, 함수)
2. 메모리 관리 (Ownership - 가장 중요)
3. 고급 (Traits, Generics, Error Handling)
4. 실무 (modules, Cargo, testing)

### 📖 활용 방법

**Level 1-2 동안 (4주):**
```
주 1: Ch. 1-3 (기초 문법)
주 2-3: Ch. 4-5 (Ownership - 핵심!)
주 4: Ch. 6-8 (Enums, Collections)
```

**Level 3-4 동안 (참고용):**
```
Ch. 10: Traits & Generics
Ch. 9: Error Handling
Ch. 7: Modules
Ch. 16: Concurrency
```

### 💡 Pro Tip

```
❌ "책을 처음부터 끝까지 읽으려고 하지 마세요"
✅ "Ch. 4를 3-5번 반복해서 읽으세요"
  → Ownership을 완벽히 이해할 때까지
  → 모든 것이 여기에서 결정됨

❌ "이론만 읽지 마세요"
✅ "각 장의 예제를 직접 타이핑하고 실행하세요"
  → Playground에서 바로 실습 가능
```

### ⏱️ 학습 효율

```
투자: 40-50시간
회수: ∞ (평생 참고 문서)

이 자료 만으로:
✓ Ownership 완벽 이해
✓ Rust의 철학 내재화
✓ 안전성의 의미 깊이 있게 이해
✓ Level 2-3 완료 가능
```

---

## 🥈 #2: Jon Gjengset - "Crust of Rust" (YouTube)
**크리에이터:** Jon Gjengset (Tokio 코어 팀)  
**형식:** YouTube 영상 시리즈  
**URL:** https://www.youtube.com/@jonhoo  
**추정 학습 시간:** 20-30시간

### ⭐ 왜 시간을 들을 가치가 있는가?

**Rust 언어 설계자가 직접 설명**
- Tokio, async-trait 등 중요 라이브러리의 코어 개발자
- Rust 커뮤니티의 존경받는 인물
- 실제 프로덕션 코드 경험 바탕으로 설명

**깊이 있는 설명**
- "왜" 그렇게 작동하는지 설명 (개념 이해)
- 컴파일러가 내부적으로 하는 일 설명
- 일반적인 실수와 해결 방법 제시

**실무 코드와 직결**
- 실제 사람들이 하는 실수 다룸
- 성능 최적화 팁 포함
- Rust 표준 라이브러리 설계 철학

### 📺 주요 에피소드

**필수 시청 (Level 2-3):**
```
1. "Closures" - Rust의 closure 시스템 완벽 설명
   → FnOnce, FnMut, Fn 구분
   → lifetime과의 관계
   시청 시간: 1시간 30분

2. "Interior Mutability" - 메모리 안전성 유지하면서 mutation
   → RefCell, Cell의 내부 구조
   → Rust의 "unsafe를 안전하게" 철학
   시청 시간: 1시간

3. "Trait Objects" - 동적 dispatch
   → dyn Trait 내부 작동
   → 성능 vs 유연성 트레이드오프
   시청 시간: 1시간

4. "Async Rust" (여러 에피소드)
   → Future, async/await 깊이 있는 설명
   → 실제로 어떻게 작동하는지 (Level 4)
   시청 시간: 3-4시간
```

**심화 (Level 4-5):**
```
- "Smart Pointers and Interior Mutability"
- "Function and Method Definitions"
- "Declarative Macros"
- 각 에피소드 1-1.5시간
```

### 💡 Pro Tip

```
❌ "모든 에피소드를 순서대로 봐야 한다"
✅ "필요한 토픽을 먼저 보세요"
  예: Ownership 다음 → Closures
      Error Handling 배운 후 → Option/Result

❌ "한 번 보고 넘어가기"
✅ "어려운 부분은 2-3번 반복 시청"
  → 실습하면서 보기 (30분 보고 코딩 1시간)

추천 시청 방식:
1. The Book 읽기
2. Crust of Rust 해당 에피소드 보기
3. rustlings로 실습
4. 자신의 코드에 적용
```

### 인물 신뢰도

**Jon Gjengset:**
- Rust 커뮤니티에서 "Rust 교사"로 알려짐
- 설명이 정확하고 깔끔함
- 자신이 실수를 인정하고 정정함 (신뢰도 높음)
- 2024년 "Rust for Rustaceans" 책 출판 (고급 개발자용)

**"그의 말이 Rust다" 수준**
- 언어 설계 과정에 참여
- 커뮤니티에서 기술 결정에 영향력 있음

### ⏱️ 학습 효율

```
투자: 20-30시간 (선택적 시청)
회수: 실무 능력 급상승

이 자료로:
✓ 개념을 "이해"에서 "체화"로 변환
✓ 실무 코드에서 하는 실수 사전 차단
✓ Rust 사상 깊이 있게 이해
✓ async/await, trait objects 마스터 가능
```

### 추천 학습 경로

```
Week 1-2: The Book Ch. 4-5 읽기
Week 2: Crust of Rust "Closures" 보기
Week 3-4: The Book Ch. 6-10 + 해당 Crust of Rust 에피소드
Week 5+: 고급 에피소드 (Async, Smart Pointers)
```

---

## 🥉 #3: rustlings (대화형 실습)
**형식:** CLI 기반 대화형 습제  
**URL:** https://github.com/rust-lang/rustlings  
**설치:** `curl --proto '=https' --tlsv1.2 -sSf https://win.rustup.rs | sh`  
**추정 학습 시간:** 15-20시간

### ⭐ 왜 시간을 들을 가치가 있는가?

**Rust 공식이 만든 대화형 교육 도구**
- 이론 + 실습을 완벽하게 통합
- 컴파일 → 실패 → 수정 의 과정을 반복
- 즉각적인 피드백

**"손으로 배우기" 가장 효과적인 방법**
```
The Book (읽기) vs rustlings (쓰기)

뇌의 기억 방식:
- 읽기: 15% 기억 (1주일 후)
- 쓰기: 90% 기억 (1주일 후)
- 쓰기 + 즉시 피드백: 95%+ 기억

rustlings는 이 모든 것을 제공합니다.
```

**구성이 완벽**
```
exercises/
├── 01_variables/
├── 02_functions/
├── 03_if/
├── 04_primitive_types/
├── 05_vecs/
├── 06_move_semantics/ ⭐ 가장 중요
├── 07_enums/
├── 08_structs/
├── 09_strings/
├── 10_generics/
├── 11_traits/
├── 12_types/
├── 13_error_handling/
├── 14_generics/
├── 15_smart_pointers/
├── 16_lifetimes/
├── 17_tests/
├── 18_iterators/
└── 19_algorithm_challenges/

총 100+ 문제
```

### 💡 Pro Tip

```
문제를 푸는 방법:

1단계: 문제 읽기
2단계: 스스로 풀이 시도 (10분)
3단계: 실패해도 좋음 - `rustlings hint` 로 힌트 보기
4단계: 힌트 바탕으로 다시 시도
5단계: 통과! `rustlings next` 다음 문제

⚠️ 자주 하는 실수:

❌ "힌트를 먼저 본다"
❌ "정답을 복사한다"
❌ "이해하지 않고 넘어간다"

✅ "스스로 20분 고민한 후 힌트 본다"
✅ "왜 컴파일 에러가 났는지 이해할 때까지 고민한다"
✅ "자신의 코드를 설명할 수 있을 때까지"
```

### 📊 추천 진행 속도

**Level 1-2 동안 (4주):**
```
Week 1:
  - 01_variables (3개 문제)
  - 02_functions (5개 문제)
  - 03_if (3개 문제)
  하루 30분 × 5일

Week 2-3: 
  - 06_move_semantics (10개 문제) ⭐ 느리게!
  - 각 문제에 30분 이상 시간 투자
  - 이 부분이 가장 중요
  
Week 4:
  - 07_enums (5개 문제)
  - 08_structs (3개 문제)
  - 09_strings (5개 문제)
```

**Level 3-4 동안:**
```
- 11_traits (6개 문제)
- 13_error_handling (5개 문제)
- 15_smart_pointers (여러 문제)
- 16_lifetimes (여러 문제)
- 18_iterators (5개 문제)
```

### ⏱️ 학습 효율

```
투자: 15-20시간
회수: 매우 높음 (손에 익은 코드)

이 자료로:
✓ 컴파일 에러를 두려워하지 않게 됨
✓ 손으로 배운 개념은 잊지 않음
✓ 문제 해결 능력 발달
✓ 자신감 상승
```

### 특징: 무료 + 오프라인 가능

```
rustlings는:
- 공식 무료 자료
- 설치 후 인터넷 불필요
- 진행 상황 저장됨
- 스스로 속도 조절 가능
```

---

## #4: Comprehensive Rust (Google)
**개발사:** Google  
**형식:** 온라인 무료 강의 + 대화형 실습  
**URL:** https://github.com/google/comprehensive-rust  
**추정 학습 시간:** 24시간 강의 + 실습

### ⭐ 왜 시간을 들을 가치가 있는가?

**최신 기업 관점의 Rust 교육**
- Google이 내부 직원 교육용으로 만듦
- 2024년 최신 내용
- 실무 프로젝트를 기반으로 구성

**빠르게 배우고 싶은 사람들을 위해 설계됨**
```
The Book: 포괄적이지만 느림 (40-50시간)
Comprehensive Rust: 집중도 높고 빠름 (24시간)

같은 내용을 절반 시간에!
```

**C++에서 온 개발자를 위해 최적화**
```
C++ → Rust 마이그레이션
- 포인터 → References
- RAII → Ownership
- Templates → Generics with Traits

도그염의 C/C++ 경력이 장점이 됨
```

### 📚 커리큘럼 구조

**Day 1: Rust Basics (2시간)**
- Variables and references
- Compound data types

**Day 2: Borrowing & Lifetimes (2시간)**
- Mutable and immutable references
- Lifetimes

**Day 3: Smart Pointers (2시간)**
- Box, Rc, Arc
- Interior mutability

**Day 4: Concurrency (2시간)**
- Threads
- Channels
- Async

**Day 5: Unsafe Code (1시간)**
- Raw pointers
- FFI

### 💡 특징

**Interactive Activities:**
- 실시간 코드 편집 가능
- 즉시 컴파일 및 실행
- 즉시 피드백

**실제 사례 중심:**
```
"타이어 게이지 측정 시스템"을 예제로 사용
→ 하드웨어 제어, 자동차 소프트웨어 관련
→ 도그염의 배경과 정확히 일치!
```

### ⏱️ 학습 효율

```
투자: 24시간 강의 + 실습
회수: 높음 (빠른 진도)

장점:
✓ 짧고 집중적
✓ 최신 정보
✓ 실무 사례
✓ C++ 개발자 맞춤

단점:
- 깊이가 The Book보다 얕을 수 있음
- 이론 설명이 적을 수 있음

추천: The Book과 함께 병행하기
```

### 추천 사용 방법

```
옵션 A: The Book 먼저 (깊이)
  The Book Ch. 1-10 (40시간)
  → Comprehensive Rust 빠른 복습 (6시간)
  → rustlings로 실습 (15시간)
  총 61시간

옵션 B: 빠른 습득 (속도)
  Comprehensive Rust (24시간)
  → The Book로 깊이 있게 (30시간)
  → rustlings로 실습 (15시간)
  총 69시간

도그염 추천: 옵션 B
이유: 빠르게 전체 개요를 잡은 후 깊이 있게 학습
```

---

## #5: "Rust in Action" (책)
**저자:** Tim McNamara  
**출판:** Manning Publications  
**출판일:** 2021년 (최신 개정판 2024년)  
**형식:** 실무 중심 책  
**가격:** ~35달러 (또는 무료 온라인 버전)  
**추정 학습 시간:** 30-40시간

### ⭐ 왜 시간을 들을 가치가 있는가?

**시스템 프로그래밍 중심**
- "Ownership으로 뭘 만들 수 있나?"를 보여줌
- 개념 이후의 실무 능력 개발
- 자동차, 임베디드, 시스템 프로그래밍 프로젝트들

**C/C++ 개발자에게 최적**
```
C/C++의 "저수준 프로그래밍" 지식을 직접 활용
- 메모리 레이아웃 이해
- 성능 최적화
- 하드웨어 제어
- 네트워크 프로그래밍

도그염의 배경과 완벽히 일치
```

**The Book과의 차이**
```
The Rust Book:
- "Rust의 문법과 개념은 무엇인가?"
- 이론 중심
- 초보자 친화

Rust in Action:
- "Rust로 실제로 뭘 만들 수 있나?"
- 실무 프로젝트 중심
- 경험 있는 개발자 대상
```

### 📚 주요 내용

**Part 1: 기초 (자동차 프로젝트)**
```
Ch. 1: A gentle introduction to systems programming
Ch. 2: Language fundamentals
Ch. 3: Compound data types
```

**Part 2: 메모리와 포인터 (도그염에게 가장 유용)**
```
Ch. 4: Lifetimes, ownership, and borrows
  → C 포인터와 비교하면서 설명
  → 메모리 안전성의 실제 의미

Ch. 5: Structs, enums, and pattern matching
  → 복잡한 데이터 구조 설계
  → 상태 머신 구현
```

**Part 3: 시스템 프로그래밍 (자동차 소프트웨어 관련)**
```
Ch. 6: Error handling
Ch. 7: Files and storage
Ch. 8: Networking
  → TCP/UDP 프로토콜 구현
  → 자동차의 통신 모듈과 유사

Ch. 9: Multithreading
  → ROS2의 multi-node 아키텍처와 연결 가능

Ch. 10: Unsafe code
  → FFI로 C/C++ 라이브러리 연동 (자동차 미들웨어)
```

**Part 4: 성능과 고급 주제**
```
Ch. 11: Async programming
Ch. 12: Web development
```

### 💡 특징

**실제 프로젝트들:**
```
프로젝트 예시:
- 파일 시스템 인터페이스 작성
- TCP 서버 구현
- 이미지 인코딩 시스템
- 네트워크 패킷 파싱

모두 시스템 프로그래밍 경험이 있는 사람에게 의미 있는 프로젝트
```

**깊이 있는 설명:**
```
단순히 "어떻게 쓰는가"가 아니라
"왜 이렇게 설계되었나"를 설명

예: 왜 Rust는 null pointer를 없앴나?
   → C의 실수를 피하기 위함
   → Option<T>의 장점
```

### ⏱️ 학습 효율

```
투자: 30-40시간
회수: 매우 높음 (실무 능력)

이 자료로:
✓ 개념 → 실무 코드 전환
✓ 자동차 소프트웨어와 연결
✓ 성능 최적화 기초
✓ FFI로 C/C++ 연동 가능
✓ Level 4-5 진입 가능
```

### 추천 읽기 순서

```
The Book을 먼저 읽고
(Ownership과 기본 개념 이해 후)

Rust in Action 읽기
(개념을 실무로 전환)

→ "아, 이제 뭘 만들 수 있겠네?" 깨달음
```

### 가격 고려사항

```
종이책: ~35달러
Kindle: ~25달러
온라인 (Manning): 무료로 일부 챕터 제공

비용 대비 효과: 매우 높음
→ 한 권의 책으로 수십 시간 학습
→ 평생 참고서
```

---

## 🎯 학습 순서 (도그염을 위한 맞춤 로드맵)

### 지금부터 시작 (Week 1-2)

```
병렬 진행:
├─ The Rust Book Ch. 1-3 (읽기) - 4시간
├─ Comprehensive Rust Day 1-2 (보기) - 4시간
└─ rustlings exercises 01-05 (실습) - 8시간

목표: 기초 문법 확실히 하기
총 16시간
```

### Week 3-4: Ownership 집중

```
병렬 진행:
├─ The Rust Book Ch. 4-5 (읽기, 여러 번) - 12시간
├─ Jon Gjengset Closures (보기) - 1.5시간
├─ rustlings 06_move_semantics (실습) - 10시간
└─ Comprehensive Rust Day 2 (다시 보기) - 2시간

목표: Ownership 완벽 이해
총 25시간

경고: 이 부분에서 절대 서두르지 마세요!
```

### Week 5-6: 타입 시스템

```
병렬 진행:
├─ The Rust Book Ch. 6-10 (읽기) - 12시간
├─ Jon Gjengset Trait Objects (보기) - 1.5시간
├─ rustlings 07-14 (실습) - 12시간
└─ Comprehensive Rust Day 3 (보기) - 2시간

목표: Traits, Generics, Error Handling 이해
총 27시간
```

### Week 7-8: 실무 능력

```
병렬 진행:
├─ Rust in Action Ch. 1-4 (읽기) - 10시간
├─ Jon Gjengset Async (보기) - 3시간
├─ rustlings 15-18 (실습) - 8시간
└─ Comprehensive Rust Day 4-5 (보기) - 3시간

목표: 실제 프로젝트 시작
총 24시간
```

**누적 학습 시간: 92시간 (약 10주)**

---

## 📊 자료별 최적 사용 시점

```
타임라인:

The Rust Book
├─ Ch. 1-3: Week 1-2 (기초 문법)
├─ Ch. 4-5: Week 3-4 (Ownership) ⭐ 여러 번
├─ Ch. 6-10: Week 5-6 (타입 시스템)
└─ Ch. 11-21: Week 7+ (고급 주제, 참고용)
  
Comprehensive Rust
├─ Day 1-2: Week 1-2 (빠른 개요)
├─ Day 2-3: Week 3-4 (깊이 있게)
├─ Day 4-5: Week 7+ (async/concurrency)
└─ 역할: 책의 내용을 다른 관점에서 설명

rustlings
├─ Ex 01-05: Week 1-2 (기초)
├─ Ex 06: Week 3-4 (move semantics - 절대 서두르지 말기!)
├─ Ex 07-14: Week 5-6 (중급)
└─ Ex 15-19: Week 7+ (고급)
  역할: "손으로 배우기" - 가장 효과적

Jon Gjengset (Crust of Rust)
├─ Closures: Week 3-4 (Ownership 이해 후)
├─ Trait Objects: Week 5-6 (Traits 배운 후)
├─ Interior Mutability: Week 6-7
├─ Async: Week 8+ (async/await 배운 후)
  역할: 깊이 있는 설명, 실무 노하우

Rust in Action
├─ Ch. 1-4: Week 7-8 (기초 개념 충분한 후)
├─ Ch. 5-7: Week 8-10 (데이터 구조, 파일 I/O)
├─ Ch. 8-10: Week 10+ (네트워킹, concurrency, unsafe)
  역할: 실무 프로젝트, 개념을 코드로 전환
```

---

## 🌟 자료 추천 종합

| 자료 | 형식 | 투자 시간 | 회수율 | 추천도 | 언제 시작? |
|------|------|---------|--------|--------|----------|
| **The Rust Book** | 책 | 40-50h | ⭐⭐⭐⭐⭐ | 필수 | 지금 |
| **Crust of Rust** | 영상 | 20-30h | ⭐⭐⭐⭐ | 거의 필수 | Week 3 |
| **rustlings** | 대화형 | 15-20h | ⭐⭐⭐⭐⭐ | 필수 | 지금 |
| **Comprehensive Rust** | 온라인 강의 | 24h | ⭐⭐⭐⭐ | 추천 | Week 1 |
| **Rust in Action** | 책 | 30-40h | ⭐⭐⭐⭐ | 강력 추천 | Week 7 |

---

## ⚠️ 피해야 할 것들

```
❌ "모든 자료를 다 봐야 한다"
→ 선택적으로 사용하세요

❌ "영상만 봐서 배운 줄 안다"
→ 직접 코드를 작성해야 합니다

❌ "이해 안 되면 다른 자료를 찾는다"
→ 같은 내용을 다른 관점에서 3-5번 반복하세요

❌ "빠르게 진행하려고 한다"
→ Ownership (The Book Ch. 4)에서는 절대 서두르지 마세요
→ 1주일을 더 투자해도 괜찮습니다

❌ "온라인 강의를 배속으로 본다"
→ 1배속으로 보고, 필요하면 같은 부분을 여러 번 보세요
```

---

## 🎓 최종 추천

### 시간이 충분한 경우 (14주+)

```
1단계 (Week 1-2): The Book Ch. 1-3 + rustlings 01-05 + Comprehensive Rust Day 1
2단계 (Week 3-4): The Book Ch. 4-5 + Jon Gjengset Closures + rustlings 06
3단계 (Week 5-6): The Book Ch. 6-10 + Jon Gjengset Trait Objects + rustlings 07-14
4단계 (Week 7-8): The Book Ch. 11-16 + Comprehensive Rust Day 4-5 + rustlings 15-18
5단계 (Week 9-14): Rust in Action + Jon Gjengset Async + 프로젝트 구현

총 학습 시간: ~100시간
최종 수준: Level 4 (실무 능력)
```

### 시간이 제한적인 경우 (8주, 도그염 상황)

```
1주-2주: Comprehensive Rust Day 1-2 + rustlings 01-05 + The Book 스캔
3주-4주: The Book Ch. 4-5 (집중!) + rustlings 06 + Jon Gjengset Closures
5주-6주: The Book Ch. 6-10 요약 + rustlings 07-14 + Jon Gjengset Trait Objects
7주-8주: Rust in Action Ch. 1-4 + rustlings 15-18 + 간단한 프로젝트

총 학습 시간: ~60시간
최종 수준: Level 2-3 (충분한 기초 + 초급 실무)
42dot 입사 후: Level 4-5로 빠르게 상승 가능
```

---

## 🚀 지금 시작하세요!

```
Today:
☐ The Rust Book 북마크: https://doc.rust-lang.org/book/
☐ rustlings 설치: https://github.com/rust-lang/rustlings
☐ Jon Gjengset 채널 구독: https://www.youtube.com/@jonhoo
☐ Comprehensive Rust 북마크: https://github.com/google/comprehensive-rust

This Week:
☐ The Book Ch. 1-3 읽기 (4시간)
☐ rustlings 시작 (2시간)
☐ Comprehensive Rust Day 1 보기 (1시간)
☐ 간단한 프로그램 작성 (3시간)

Next Week:
☐ The Book Ch. 4-5 읽기 (집중 읽기!)
☐ rustlings 06_move_semantics 시작 (이 부분에 시간을 아끼지 마세요)
☐ Jon Gjengset "Closures" 보기
```

**이 5가지 자료로 Rust를 완벽하게 배울 수 있습니다.** 🦀

각각의 장점:
- **The Rust Book**: 완벽한 기초
- **Crust of Rust**: 깊이 있는 이해
- **rustlings**: 손으로 배우기
- **Comprehensive Rust**: 빠른 진도
- **Rust in Action**: 실무 능력

모두를 활용하면 → **3-4개월 후 실무 준비 완료** ✅
