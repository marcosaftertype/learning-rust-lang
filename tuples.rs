use std::{i32};

fn main() {

	let tuple = ("Marcos", 27);

	let tuple2: (&str, i8) = ("Marcos", 27);

	println!("Name : {}", tuple2.0);

}