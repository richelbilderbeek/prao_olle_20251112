use std::{env::args, fs::{self, rename}, io::Read, process::exit};


fn main() {
    let path = "courses.md";
    let read = fs::read_to_string(path).unwrap();
    let mut split: Vec<String> = read.split("\n").map(|v|v.to_string()).collect();
    let mut i = 0;
    let mut to_change: Vec<usize> = Vec::new();
    for line in &split {
        if line == "<!-- courses_2.md is machine-generated and pasted below this file, courses_1.md -->" {
            break;
        }
        if line == "" && !(split[i-1].starts_with(" ")) {
            to_change.push(i.try_into().unwrap());
        }
        i += 1;
    }
    for x in to_change {
        split[x] = "<!-- DO NOT EDIT-->".to_owned() 
    }
    let mut f = String::new();
    for x in split {
        f += &(x + "\n");
    }
    fs::write("courses.md", f).unwrap();
}
