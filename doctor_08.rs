extern crate regex;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;

fn main() {
    let path = Path::new("input.arnoldc");
    let display = path.display();
    let mut file = match File::open(&path){
    	Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    	Ok(file) => file,
    };

    let mut s = String::new();
    let mut s = match file.read_to_string(&mut s){
    	Err(why) => panic!("couldn't read {}: {}", display, why.description()),
    	Ok(_) => s,
    };

    let mut count = 0;

    // print contents of file
    // print!("File contains: \n{}", s);

    // Menu
	println!("[1] Show all numbers");
	println!("[2] Show all keywords");
	println!("[3] Show all strings");
	println!("[4] Show all non-keyword identifiers");
	println!("Enter choice: ");

	// Code to accept input from user
	let mut choice = String::new();
	io::stdin().read_line(&mut choice);
	let choice:i32 = choice.trim().parse().expect("Error");

	if choice == 1 {
		let re = Regex::new(r"([^a-z]-?\d+)").unwrap();

	    
	    for cap in re.captures_iter(&s){
	    	count = count + 1;
	    }
	    println!("Count: {}", count);
	    for cap in re.captures_iter(&s){
	    	println!("Detected integer: {}", cap.at(1).unwrap());
	    }
	} else if choice == 2 {
		let re = Regex::new(r"(([A-Z']+ ?)+)").unwrap();

	    
	    for cap in re.captures_iter(&s){
	    	count = count + 1;
	    }
	    println!("Count: {}", count);
	    for cap in re.captures_iter(&s){
	    	println!("Detected keywords: {}", cap.at(1).unwrap());
	    }
	} else if choice == 3 {
	    let re = Regex::new(r#"(")(\w+)(")"#).unwrap();

	    
	    for cap in re.captures_iter(&s){
	    	count = count + 1;
	    }
    	println!("Count: {}", count);
	    for cap in re.captures_iter(&s){
	    	println!("Detected string literal: {}", cap.at(2).unwrap());
	    }
	} else if choice == 4 {
		let re = Regex::new(r"([a-z]+[a-zA-z0-9_]*)").unwrap();

	    
	    for cap in re.captures_iter(&s){
	    	count = count + 1;
	    }
	    println!("Count: {}", count);
	    for cap in re.captures_iter(&s){
	    	println!("Detected identifier: {}", cap.at(1).unwrap());
	    }
	} else {
		println!("Invalid choice.");
	}

}