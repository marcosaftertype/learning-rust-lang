use std::{i32};

use std::io::stdin;

fn main() {
	
	let mut vect1 = vec![1,2,3,4,5];

	println!("Item 2: {}", vect1[1]);

	for i in &vect1 {

		println!("Vect: {}", i);

	}

	vect1.push(10);
	vect1.push(20);
	vect1.push(30);

	vect1.pop();

	for i in &vect1 {

		println!("Vect: {}", i);

	}
}