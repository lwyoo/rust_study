enum Message {
    Quite,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("Hello, world!");
    // let mut arr = [5,4,3,2,11];
    // println!("Array elements: {} {} {} {} {}", arr[0], arr[1], arr[2], arr[3], arr[4]);
    // println!("array data size: {}", arr.len());
    // arr[0] = 11;
    // for n in arr {
    // 	println!("{}", n);
    // }

    // 생성자
    // let mut vec:Vec<i32> = Vec::new();
    // println!("{:?}", vec);

    // 초기화
    // let vec = vec![0,1,2,3,4];
    // println!("{:?}", vec);

    // 반복 생성
    // let vec = vec![0 ; 5];

    // let vec: Vec<i32> = (0..5).map(|i| i).collect();   // [2, 3, 4, 5, 6]
    // println!("{:?}", vec);

    //   let mut value = 1;
    //   let vec: Vec<i32> = std::iter::repeat_with(|| {
    //       value += 1;
    //       value
    //   })
    //   .take(5)
    //   .collect(); // [2, 3, 4, 5, 6]
    // println!("{:?}", vec);

    // vec 추가 & 조회
    // let mut vec = vec![1,11,111];
    // vec.push(1111);
    // println!("{:?}", vec);

    // vec.pop();
    // let rm_value = vec.pop();

    // println!("Removed value: {:?}", rm_value);
    // println!("Removed value: {:?}", rm_value.unwrap());
    // println!("{:?}", vec);

    // println!("vec[0]: {}", vec[0]);
    // println!("vec[0]: {:?}", vec.get(0));
    // println!("vec[0]: {}", vec.get(0).unwrap());

    // match vec.get(0) {
    // 	// Some(value) => println!("vec[0]: {}", value),
    // 	Some(_) => println!("vec[0]: {}", vec.get(0).unwrap()),
    // 	None => println!("vec[0] does not exist"),
    // }

    // 벡터 순환
    // let vec = vec![1,2,3,4,5];

    // // 값으로 순환
    // for num in &vec {
    // 	println!("{}", num);
    // }
    // // 값으로 순환
    // for num in vec {
    // 	println!("{}", num);
    // }

    // let mut array = [1,2,3,4];

    // for num in array {
    // 	println!("{}", num);
    // }
    // for num in &array {
    // 	println!("{}", num);
    // }

    // match array.get_mut(0) {
    // 	// Some(value) => println!("array[0]: {}", value),
    // 	// Some(value) => println!("array[0]: {}", *value),
    // 	Some(value) => *value = 11230,
    // 	None => println!("array[0] does not exist"),
    // }

    // for num in &array {
    // 	println!("{}", num);
    // }

    // let x = 30;
    // let r = &x;      // r은 x를 가리키는 참조 (&i32)
    // println!("{}", *r);   // *로 역참조 → 30
    // println!("{}", r);   // *로 역참조 → 30

    // let numbers = vec![1,2,3,4,5];

    // for num in &numbers {
    // 	println!("{}", num);
    // }

    // for num in numbers.iter() {
    // 	println!("{}", num);
    // }

    // println!("{:?}", numbers);

    // for (idx, num) in numbers.iter().enumerate() {
    //     println!("[{}] = {}", idx, num);
    // }

    // // 속성
    // #[deny(unused_variables)]
    // let x = 1;

    // #[warn(unused_variables)]
    // let y = 11;

    // #[allow(unused_variables)]
    // let z = 111;

    // // panic
    // let x = ["a"];
    // #[warn(unconditional_panic)]
    // let y = x[1];
    // println!("{}", y);

    // // 가변 배열
    // let mut x = [1,2,3,4,5];
    // println!("{}, {}, {}, {}, {}", x[0], x[1], x[2], x[3], x[4]);
    // x[0] = 10;
    // println!("{}, {}, {}, {}, {}", x[0], x[1], x[2], x[3], x[4]);

    // // 전체 변경시 크기와 데이터 타입은 일치 해야 한다
    // x = [5,4,3,2,1];
    // println!("{}, {}, {}, {}, {}", x[0], x[1], x[2], x[3], x[4]);

    // // 배열 크기 명시적 지정
    // let x = [10;5];
    // println!("{:?}", x);

    // // 다차원 배열
    // let x = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    // for i in 0..x.len() {
    // 	for j in 0..x[i].len() {
    // 		print!("x[{}][{}] = {}", i, j, x[i][j]);
    // 	}
    // 	println!("");
    // }

    // // 벡터
    // let mut x = vec![0;5];
    // println!("{:?}", x);

    // for i in 0..x.len() {
    // 	x[i] = i;
    // }
    // println!("{:?}", x);

    // x.insert(1, 222);
    // println!("{:?}", x);

    // x.remove(0);
    // println!("{:?}", x);

    // x.clear();

    // // for i in 0..x.len() {
    // // 	x.remove(0);
    // // }

    // // 벡터 초기화
    // println!("{:?}", x);

    // let y = vec![0;0];
    // println!("{:?}", y);

    // // 배열 복사
    // let mut a1 = [1,2,3];
    // let a2 = [3,2,1];

    // a1 = a2;
    // println!("a1: {:?}", a1);
    // println!("a2: {:?}", a2);

    // // 벡터 복사
    // let mut v1 = vec![10,20,30];
    // let v2 = vec![30,20,10];
    // println!("v1: {:?}", v1);
    // println!("v2: {:?}", v2);

    // v1 = v2.clone();
    // v1[0] = 100;
    // println!("v1: {:?}", v1);
    // println!("v2: {:?}", v2);
    // let mut vec = Vec::new();

    // println!("초기: len={}, capacity={}", vec.len(), vec.capacity());
    // // 초기: len=0, capacity=0

    // vec.push("1");
    // println!("1번째: len={}, capacity={}", vec.len(), vec.capacity());
    // // 1번째: len=1, capacity=4  ← 첫 push에서 4개 공간 할당!

    // vec.push("2");
    // vec.push("3");
    // vec.push("4");
    // println!("4번째: len={}, capacity={}", vec.len(), vec.capacity());
    // // 4번째: len=4, capacity=4  ← 아직도 4, 여유 있음

    // vec.push("5");
    // println!("5번째: len={}, capacity={}", vec.len(), vec.capacity());
    // // 5번째: len=5, capacity=8  ← 이제 2배로 확장 (4→8)

    // let mut vec = vec![1, 2, 3, 4, 5];

    // for num in &mut vec {
    //     *num *= 2;  // 각 요소를 2배로
    // }

    // println!("{:?}", vec);  // [2, 4, 6, 8, 10]

    // let a = &3;
    // let b = &3;

    // println!("a 주소: {:p}", a);
    // println!("b 주소: {:p}", b);
    // println!("같은가? {}", a as *const i32 == b as *const i32);

    // let a1 = 0x_0000_0010;
    // let a2 = 0o_0000_0010;
    // let a3 = 0b_0000_0010;
    // let a4 = 10;
    // println!("a1: {}, a2: {}, a3: {}, a4: {}", a1, a2, a3, a4);

    // let r1:f32 = 10.0/3.0;
    // let r2:f64 = 10.0/3.0;

    // println!("r1 (f32): {}", r1);
    // println!("r2 (f64): {}", r2);

    // // 상수
    // const N: usize = 20;
    // let array = [0; N];
    // println!("N: {}, array: {:?}", N, array);

    // let s = "Hello";

    // // 하지만 bytes()로 하면:
    // let byte = s.as_bytes()[0];  // 바이트!

    // println!("{}", byte);        // 72 출력
    // println!("{:?}", byte);      // 72 (숫자)
    // println!("타입: {}",
    // std::any::type_name_of_val(&byte));

    // let byte2 = &s[0..1];
    // println!("{}", byte2);        // H 출력
    // println!("{:?}", byte2);      // "H" (문자열)
    // println!("타입: {}",
    // 				std::any::type_name_of_val(&byte2));
    // // 타입: &str ← 문자열 슬라이스!

    // let x64: i64 = 64;
    // let x32 = x64 as i32;

    // let b1 = false;

    // println!("b1: {}", b1);
    // println!("x64: {}", x32);

    // // let b2 = x32 as bool;
    // // println!("b2: {}", b2);

    // let b2 = b1 as i32;
    // println!("b2: {}", b2);

    // let mut test = ("test", 1);
    // println!("Before: {:?}", test);
    // test.1 = 2;
    // println!("After: {:?}", test);

    // let mut  vec:Vec<(String, i32)> = vec![("test".to_string(), 1)];
    // println!("{:?}", vec);
    // vec.push(("name".to_string(), 111));
    // println!("{:?}", vec);

    // for i in &vec {
    // 	println!("{:?}", i);
    // }

    // // enum
    // #[allow(dead_code)]
    // enum Name {
    // 	Lee,
    // 	Kim,
    // 	Choi
    // }

    // // println!("{:?}", Name::Lee);
    // // println!("{:?}", Name::Kim);
    // // println!("{:?}", Name::Choi);

    // // let name = Name::Lee;
    // let name: Name = Name::Kim;
    // // let name = Name::Choi;
    // match name {
    // 	Name::Lee => println!("Lee"),
    // 	Name::Kim => println!("Kim"),
    // 	Name::Choi => println!("Choi"),
    // }

    // match 문의 구조
    // 표현식 + 문장

    // #[allow(dead_code)]
    // enum CardinalPoints {
    // 	East,
    // 	West,
    // 	South,
    // 	North
    // }

    // let my_point = CardinalPoints::North;

    // match my_point{
    // 	CardinalPoints::East => println!("East"),
    // 	CardinalPoints::West => println!("West"),
    // 	CardinalPoints::South => println!("South"),
    // 	CardinalPoints::North => {
    // 		println!("North");
    // 		println!("North!!");
    // 	},
    // 	_ => println!("Error")
    // }

    // let mut num = 11;
    // match num {
    // 	10 => println!("this is 10"),
    // 	_ => {
    // 		num = 1;
    // 		println!("this is {}", num);
    // 	}
    // }

    // println!("update {}", num);

    // // enum 을 사용한 자료형 지정
    // enum Result {
    // 	Success(u8),
    // 	Failure(u16,char),
    // 	Uncertaity,
    // }

    // let r = Result::Failure(0, 'a');
    // let r = Result::Success(0);

    // let a = 99;
    // match r {
    // 	Result::Success(0) => println!("Success code: 0"),
    // 	Result::Success(1) => println!("Success code: 1"),
    // 	Result::Success(2) => println!("Success code: 2"),
    // 	// 쉐도잉 SCOP (match block) 내 임시 변수
    // 	Result::Success(a) => println!("[Shadowing] Success code: {}", a),
    // 	// Result::Success(_) => println!("Success code: _"),
    // 	Result::Failure(_,_) => println!("Failure()"),
    // 	Result::Uncertaity => println!("Uncertaity"),
    // 	_ => println!("error")
    // }

    // println!("a is: {}", a);

    // // match 표현식
    // enum 방위 {동, 서, 남, 북}
    // let 진행방향 = 방위::남;

    // println!("나의 방향은: {}",
    // 		match 진행방향 {
    // 			방위::동 => "동쪽이요",
    // 			방위::서 => "서쪽이요",
    // 			방위::남 => "남쪽이요",
    // 			방위::북 => "북쪽이요",
    // 		}
    // );

    // // match 가드 (if, while)

    // for n in -10..11 {
    // 	println!("{}: {}", n, match n {
    // 		0 => "zero",
    // 		_ if n > 0 => {
    // 			"양수"
    // 		},
    // 		_ => "음수"
    // 	});
    // }

    // // if-let 구조

    // // match 형
    // enum E {
    // 	Case1(bool, i32),
    // 	Case2(bool, i32),
    // 	Case3(bool, i32),
    // }

    // let v = E::Case1(true, 1);

    // match v {
    // 	E::Case1(state, code) => if state {println!("Case1 (state: {}, code: {})", state, code)},
    // 	_ => {}
    // }

    // // if let 사용 하는 방식
    // // "한 가지 패턴만 특별히 처리하고 싶을 때 쓰는 간편 문법"
    // if let E::Case1(state, code) = v {
    // 	if state {
    // 		println!("Case1 (state: {}, code: {})", state, code)
    // 	} else {

    // 	}
    // } else {
    // 	print!("else")
    // }

    // // if 다음에 조건이 아니라 대입 연산자가 들어오네?
    // // -> 이건 if let 이라는 구문이다
    // // 변수 (v) 의 값을 enum 값의 인자로 넣을수 있는지(한 가지 패턴만 특별히 처리하고 싶을 때 쓰는 간편 문법) 확인 하는것
    // // "v라는 변수가 E::Case1 패턴과 일치하는지 확인하고,
    // //  일치하면 내부의 (state, code)를 추출해줘" 라는 의미
    // // enum 에서만 사용가능한것인가?
    // // -> Option<T> 도 가능
    // // -> Result<T, E> 도 가능
    // // -> 일반 값도 가능

    // // while-let
    // enum E {
    // 	Case1(u32),
    // 	Case2(char)
    // }

    // let mut v = E::Case1(1);

    // // v 값이 E::Case1 에 대입이 가능한 동안에
    // while let E::Case1(n) = v {
    // 	println!("{}", n );
    // 	if n == 5 {
    // 		break;
    // 	}
    // 	v = E::Case1(n+1);
    // }

    // 테스트

    let message = [
        Message::Move { x: 10, y: 20 },
        Message::Write("hello".to_string()),
        Message::ChangeColor(255, 255, 255),
        Message::Quite,
    ];

    // for msg in &message {
    // 	match msg {
    // 		Message::Move{x,y} => println!("Message::Move({},{})", x,y),
    // 		Message::Write(x) => println!("Message::Write({})", x),
    // 		Message::ChangeColor(x,y, z) => println!("Message::ChangeColor({}, {}, {})", x, y, z),
    // 		Message::Quite => println!("Message::Quite"),
    // 	}
    // }
    // for msg in &message {
    //     sub(msg);
    // }

    // for msg in &message {
    // 	if let Message::Quite = msg {
    // 		println!("Message::Quite!!!!!!!")
    // 	} else if let Message::Move{x, y} = msg {
    // 		println!("Message::Move {{{}, {}}}", x, y)
    // 	} else if let Message::ChangeColor(r, g, b) = msg {
    // 		println!("Message::ChangeColor( {}, {}, {} )", r, g, b)
    // 	} else if let Message::Write(x) =  msg {
    // 			println!("Message::Write( {} )", x)
    // 	}
    // }

    for msg in &message {
        sub2(msg);
    }
}

fn sub(msg: &Message) {
    match msg {
        Message::Move { x, y } => println!("Message::Move({},{})", x, y),
        Message::Write(x) => println!("Message::Write({})", x),
        Message::ChangeColor(x, y, z) => println!("Message::ChangeColor({}, {}, {})", x, y, z),
        Message::Quite => println!("Message::Quite"),
    }
}

fn sub2(msg: &Message) {
    if let Message::Quite = msg {
        println!("Message::Quite!!!!!!!")
    } else if let Message::Move { x, y } = msg {
        println!("Message::Move()");
				println!("positionX : {}, positionY : {}\n", x, y);
    } else if let Message::ChangeColor(r, g, b) = msg {
        println!("Message::ChangeColor()");
				println!("Red: {}, Green: {}, Blue: {}\n", r, g, b);
    } else if let Message::Write(x) = msg {
        println!("Message::Write()");
				println!("Text: {}\n", x);
    }
}
