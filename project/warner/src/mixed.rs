use std::fs::{self, rename};

pub fn warn(input_file_contents: String) -> String{
    "<!-- DO NOT EDIT -->".to_owned() + &input_file_contents
}