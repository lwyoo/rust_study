fn main() {
    println!("Hello, world!");
		let mut arr = [5,4,3,2,11];
		println!("Array elements: {} {} {} {} {}", arr[0], arr[1], arr[2], arr[3], arr[4]);
		println!("array data size: {}", arr.len());
		arr[0] = 11;
		for n in arr {
			println!("{}", n);
		}
}
