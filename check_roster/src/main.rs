use std::env;
use std::fs;
//use std::io::prelude::*;

// A challenge to write and read from files.
fn main() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }

    //file path
    let arg1 = env::args().nth(1).unwrap();
    //println!("First argument is: {}", arg1);
    //name to check from file
    let arg2 = env::args().nth(2).unwrap();

    let contents = fs::read_to_string(arg1).unwrap();

    for line in contents.lines() {
        if line == arg2 {
            println!("{} was on the roster and has walked the moon!", arg2);
            return;
        }
    }

    println!("Name was not on roster.");
    //let mut speech = String::new() 
    
}
