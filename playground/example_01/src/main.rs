fn main() {
	// // 변수에 값을 변경하고 싶은 경우 
	// let mut n = 32;
	// println!("n: {}", n);
	// n = 64;
	// println!("n: {}", n);

	// // 지연된 초기화 - 선언 후 한번은 초기화 된다 
	// let n2:i32;
	// n2 = 12;
	// println!("n2: {}", n2);

	// // 지연된 초기화2
	// let n;
	// let n2 = 2;
	// n = n2;
	// println!("n: {}", n);
	// println!("n2: {}", n2);

	// // 선행 언더스코어: 변수 앞에 _를 붙이면 사용하지 않는 변수임을 나타낼 수 있다.
	// let _n = 21;
	// let n = 22;
	// println!("n: {}", n);

	// // _ 의 사용
	// // 매개변수로 무엇인가 받는데 사용하지 않겠다, default 매개변수
	// // 값을 받을 수 는 있지만 넣을수는 없다 
	// let _ = 30;

		// let b_value_a = !false;
		// let b_value_b = true;
		// println!("{}", b_value_a);
		// println!("{}", b_value_b);
		// println!("{}", b_value_b && b_value_a);

		// let mut n1;
		// let mut n2;
		// let mut n3;
		// let mut n4;
		
		// n4 = 1.22;
		// n3 = n4 -1.0;
		// n2 = n3-1.0;
		// n1 = n2-1.0;

		// print!("n1: {}, n2: {}, n3: {}, n4: {}", n1, n2, n3, n4);

		// n4 = 99.9;
		// n3 = n4 -1.0;
		// n2 = n3-1.0;
		// n1 = n2-1.0;

		// print!("n1: {}, n2: {}, n3: {}, n4: {}", n1, n2, n3, n4); 
		// let mut n = 1;
		// n = 32;
		// println!("n: {}", n);

		// //쉐도우잉
		// let n = 11;
		// println!("n: {}", n);

		//표준 라이브러리 함수 사용 - 인스턴스.함수()
		// let strlen;
		// let s = "가나";
		// strlen = s.len();
		// println!("str: {}, strlen: {}", s, strlen); // 문자들의 byte 수

		//표준 라이브러리 함수 사용 - 모듈1::모듈2::자료형::함수()
		// let strlen = str::len(s);
		// println!("str: {}, strlen: {}", s, strlen); // 문자들의 byte 수

		// let mut n = 1;
		// println!("n: {}", n);
		// n += 1;
		// println!("n: {}", n);
		


}