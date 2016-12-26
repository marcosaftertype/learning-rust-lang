use std::{i32};

use std::io::stdin;

fn main() {
    
    let rand_arr = [1,2,3];

    println!("{}", rand_arr[0]);

    println!("{}", rand_arr.len());

    println!("Second 2 : {:?}", &rand_arr[1..3]);
}