// use std::string;

// fn main() {
//     {
//         // tuple
//         // 타입이 달라도 동작 한다
//         println!("Hello, world!");
//         let data = ("튜플 테스트", 32);
//         println!("first data: {}", data.0);
//         println!("second data: {}", data.1);
//         println!("debug: {:?}", data);
//         // 튜플은 첨자 접근을 변수를 사용할 수 없다
//         let array = [19, 2, 3];
//         let index = 0;
//         println!("{}", array[index]);
//         // println!("{}", data.index); 안됨
//     }

// 		// 빈 튜플
// 		{
// 			let NoData = ();
// 		}

//     // struct
//     {
//         struct People {
//             name: String,
//             age: u32,
//             from: String,
//         }

//         let ldg = People {
//             name: String::from("이도겸"),
//             age: 38,
//             from: "Korea".to_string(),
//         };

//         println!("name: {}", ldg.name);
//         println!("age: {}", ldg.age);
//         println!("from: {}", ldg.from);
//     }

// 		// 빈 구조체
// 		{
// 			struct NoData {};
// 		}

// 		// 튜플-구조체
// 		/**
// 		 * 구조체 => 튜플 같이 사용
// 		 * => 구조체 장점: 타입정의
// 		 * => 튜플의 장점: 필드명 없이 사용 가능
// 		 * 선언 + 필드명 생략 가능 
// 		 */

// 		struct 튜플구조체 (
// 			String,
// 			u32,
// 			String,
// 		);

// 		let data = 튜플구조체("name".to_string(), 39, "Korea".to_string());
// }

fn main () {
	
	// /// struct test 
	// println!("[구조체 테스트]");
	// struct Info {
	// 	name:String,
	// 	age: u32,
	// }

	// let data = Info {
	// 	name:String::from("aa"),
	// 	age:10,
	// };

	// println!("[구조체 테스트] name: {}", data.name);
	// println!("[구조체 테스트] age: {}", data.age);

	/// tuple test 
	println!("[튜플 테스트]");

	let data = (String::from("this is tuple"), 33);
	println!("[튜플 테스트] first value: {}", data.0);
	println!("[튜플 테스트] second value: {}", data.1);

	let tuple_size = std::mem::size_of_val(&data);
	println!("tuple_size: {}", tuple_size)

	// // tuple struct test 
	// println!("[튜플 구조체 테스트]");

	// struct info2 (
	// 	String,
	// 	u32,
	// 	String
	// );

	// let data = info2("name ".to_string(), 123, "student".to_string());

	// println!("[튜플 구조체 테스트] {}", data.0);
	// println!("[튜플 구조체 테스트] {}", data.1);
	// println!("[튜플 구조체 테스트] {}", data.2);



}