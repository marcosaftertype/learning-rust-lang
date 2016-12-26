use std::{i32};

fn main() {
	say_hello("Marcos");

	println!("5 + 4 = {}", get_sum(5,4));

	let sum = get_sum;

	println!("6 + 4 = {}", sum(6,4));

	let sum_nums = |x: i32, y: i32| x + y;
	println!("7 + 8 = {}", sum_nums(7,8));

	let num_ten = 10;

	let add_10 = |x: i32| x + num_ten;
	println!("Add ten to 5: {}", add_10(5));

}

fn say_hello(name: &str) {
	println!("Hello {}", name);
}

fn get_sum(num1: i32, num2: i32) -> i32 {
	// If not semi-colon at the end, it returns
	num1 + num2
}