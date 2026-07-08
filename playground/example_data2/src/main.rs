
// Book 이라는 구조체 생성 
struct Book {
	title:String,
	author:String,
	// 생략 가능하도록 Oprtion 을 사용 
	location:Option<String>
}

// Book 이라는 구조체의 함수 정의
impl Book {
	fn new(title: String, author: String, location: Option<String>) -> Book {
		// 축약 문법
		Book {
			title, author, location : None
		}

		// // 명시적 매핑
		// Book {
		// 	title: title,
		// 	author: author,
		// 	location: location
		// }
	}

	// self 는 현재의 구조체 의미
	fn set_location(&mut self, location:String) {
		// Option 은 Some 을 이용해 데이터 입력 한다 
		self.location = Some(location);
	}

	// Result 는 반환 값이 Ok 와 Err 두가지 이다
	fn check_location(&self) -> Result<&String, String> {
		match &self.location {
			Some(location) => Ok(location),
			None => Err("도서 위치가 설정되어 있지 않습니다.".to_string())
		}
	}
}

fn main() {
	// 구조체의 구현 함수를 접근 하는 경우 '::' 으로 접근 한다 
	let mut data = Book::new("러스트 프로그래밍".to_string(), "홍길동".to_string(), None);

	data.set_location("거실 책장 2단".to_string());

	match data.check_location() {
		Ok(location) => {
			println!("Title: {}", location);
			println!("Author: {}", location);
			println!(":Location: {}", location);
		}
		Err(e) => println!("오류: {}", e),
	}
}