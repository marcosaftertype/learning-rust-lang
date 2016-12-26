use std::{i8};

use std::io::stdin;

fn main() {
    
    let rand_string = "A random string";

    println!("Length: {}", rand_string.len());

    let (first, second) = rand_string.split_at(8);

    println!("First: {} Second: {}", first, second);

    let count = rand_string.chars().count();
    let mut chars = rand_string.chars();

    let mut ind_char = chars.next();

    loop {
    	match ind_char {
    		Some(x) => println!("{}", x),
    		None => break,
    	}
    	ind_char = chars.next();
    }

    let mut iter = rand_string.split_whitespace();

    let mut ind_word = iter.next();

    loop {
        match ind_word {
        	Some(x) => println!("{}", x),
        	None => break,
        }

        ind_word = iter.next();
    }

    let rand_string2 = "Inmensae subtilitatis, obscuris et malesuada fames.\nQuae vero auctorem tractata ab fiducia dicuntur.\nUt enim ad minim veniam, quis nostrud exercitation.";

    let mut lines = rand_string2.lines();

    let mut ind_line = lines.next();

    loop {
        match ind_line {
        	Some(x) => println!("{}", x),
        	None => break,
        }

        ind_line = lines.next();
    }

    println!("Find auctorem: {}", rand_string2.contains("auctorem"));
}