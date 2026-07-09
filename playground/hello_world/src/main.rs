fn main() {
    println!("Hello, world!");
		let mut grade ;
		let mut score;
		score = 100.0;
		grade = calculcate_grade(score);
		println!("score: {}, grade: {}", score, grade);


		score = 90.0;
		grade = calculcate_grade(score);
		println!("score: {}, grade: {}", score, grade);


		score = 80.0;
		grade = calculcate_grade(score);
		println!("score: {}, grade: {}", score, grade);


		score = 70.0;
		grade = calculcate_grade(score);
		println!("score: {}, grade: {}", score, grade);


		score = 60.0;
		grade = calculcate_grade(score);
		println!("score: {}, grade: {}", score, grade);

}

// /home/ldg/Study/rust_study/playground/hello_world/Cargo.toml 에 추가함
#[cfg(not(features = "owned_grade"))]
// lifetime 으로 &str 로 반환 하면 안됨 &'static str 로 반환 필요
fn calculcate_grade(score:f64) -> &'static str {
	println!("features: not owned_grade");
	if score >= 90.0 {
		"A"
	} else if score >= 80.0 {
		"B"
	} else if score >= 70.0 {
		"C"
	} else {
		"F"
	}
}
#[cfg(features = "owned_grade")]
fn calculcate_grade(score:f64) -> String {
	println!("features: owned_grade");
	if score >= 90.0 {
		"A".to_string()
	} else if score >= 80.0 {
		"B".to_string()
	} else if score >= 70.0 {
		"C".to_string()
	} else {
		"F".to_string()
	}
}
