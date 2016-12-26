use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
    println!("Hello World!");

    let num = 10;

    let mut age: i32 = 40;

    let is_it_true: bool = true;

    let let_x: char = 'x';

    println!("I am {} years old", age);

    let (f_name, l_name) = ("Marcos", "Navarro");

    println!("Is it {0} that {1} is {2} years old", is_it_true, f_name, age);

    println!("{:.2}", 1.2345);

    println!("Binary: {:b}, Hex: {:x} O: {:o}", 10, 10, 10);

    println!("{ten:>ws$}", ten=10, ws=5);

    println!("{ten:>0ws$}", ten=10, ws=5);

    println!("5 + 4 = {}", 5 + 4);

    let age_old = 6;

    if age_old == 5 {
    	println!("Go to kindergarten");
    } else if age_old > 5 && age_old < 18 {
        println!("Go to grade {}", age_old - 5);
    }
}