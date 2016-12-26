use std::{i8};

use std::io::stdin;

fn main() {
    
    let mut x: i8 = 1;

    loop {
        if (x % 2) == 0 {
        	println!("{}", x);
        }
        if x > 10 {
        	break;
        }
    	x += 1;
    	continue;
    }

    let mut y: i8 = 1;

    while y <= 5 {
    	println!("{}", y);
    	y += 1;
    }

    for z in 1..5 {
    	println!("FOR : {}", z);
    }
}