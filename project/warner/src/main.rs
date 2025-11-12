use std::{env::args, fs::{self, OpenOptions}, io::{Read, Write}};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = args().collect();
    let file = args.get(1);
    if file != None {
        let file_path = file.expect("No file was supplied");
        let mut file = fs::File::open(file_path)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let mut split: Vec<String> = file_contents.split("\n").map(|v| v.to_string()).collect();
        split.retain(|v| v != "");
        #[allow(unused_assignments)]
        let mut i = 0;
        while i < split.len() {
            split[i] = "# DO NOT EDIT".to_owned() + &split[i];
            i += 1;
        }
        let mut options = OpenOptions::new();
        options.append(true);
        options.create_new(true);
        let mut f = options.open("./output.md")?;
        for v in split {
            f.write((v + "\n").as_bytes())?;
        }

        return Ok(());
    }
    panic!("No file was supplied")
}
