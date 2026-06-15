fn main() {
	// let n = 1;
	// if n > 0 {
	// 	println!("n은 양수 입니다."); 
	// } else {
	// 	println!("n은 0이거나 음수 입니다.");
	// }

	 // a 가 10 보다 크거나 같으면, 3으로 나눈 나머지
	 // 10보다 작으면 2로 나눈 나머지

	// let a = 14;
	// let n;
	//  // 일반 if 문
	// if a > 10 {
	// 	n = a % 3;
	// } else {
	// 	n = a % 2;
	// }
	// println!("n = {}", n);

	//  // 표현식
	// let n = if a > 10 {
	// 	a % 3
	// } else {
	// 	a % 2
	// };
	// println!("n = {}", n);	

	for x in 2..10 {
		for y in 1..10 {
			println!("{} * {} = {}", x, y, x * y);
		}
	}


}
