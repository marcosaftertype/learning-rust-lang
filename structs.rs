use std::{i32, f64};

fn main() {
	let mut circle1 = Circle {
		x: 10.0, y: 10.0, radius: 10.0
	};

	println!("X: {}, Y: {}, R: {}", circle1.x, circle1.y, circle1.radius);

	println!("Circle Radius: {}", get_radius(&circle1));

	println!("Circle X: {}", circle1.get_x());

	let mut square1 = Square {
		side: 5.0
	};

	println!("Square area: {}", square1.area());
}

struct Circle {
	x: f64,
	y: f64,
	radius: f64,
}

struct Square {
	side: f64,
}

fn get_radius(circle: &Circle) -> f64 {
	circle.radius
}

impl Circle {
	pub fn get_x(&self) -> f64 {
		self.x
	}
}

impl Square {
	pub fn area(&self) -> f64 {
		self.side.powf(2.0)
	}
}