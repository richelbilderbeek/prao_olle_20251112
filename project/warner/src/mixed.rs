use std::fs;

pub fn warn(input_file_contents: String, output_file_path: String) {
    let split: Vec<String> = input_file_contents.split("\n").map(|v| v.to_owned()).collect();
    let mut final_string = String::new();
    for line in  split{
        final_string += &(line + "\n<!-- DO NOT EDIT -->\n");
    }
    fs::write(output_file_path, final_string).unwrap();
}