use std::fs;

pub fn warn(input_file_contents: String) -> String{
    let split: Vec<&str> = input_file_contents.split("\n").collect();
    let mut final_string = String::new();
    for line in split {
        final_string += &(line.to_owned() + "<!-- DO NOT EDIT -->" + "\n");
    }
    println!("{}", final_string);
final_string
}