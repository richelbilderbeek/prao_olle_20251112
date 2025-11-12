use std::{env::args, fs::{self}, io::Read, process::exit};


fn main() {
    let input_file_path = "courses.md";
    let split: Vec<String> = fs::read_to_string(input_file_path).unwrap().split("\n").map(|v| v.to_string()).collect();
    let mut i = 0;
    println!("{:?}", split);
    
}
