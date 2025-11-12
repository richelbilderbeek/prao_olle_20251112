use std::{env::args, fs::{self}, io::Read};
mod mixed;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = args().collect();
    let file = args.get(1);
    let output_file = args.get(2);
    if file != None  && output_file != None{
        let file_path = file.expect("No input file was supplied");
        let mut file = fs::File::open(file_path)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let mut split: Vec<String> = file_contents.split("\n").map(|v| v.to_string()).collect();
        split.retain(|v| v != "");
        split.remove(0);
        split.remove(0);
        #[allow(unused_assignments)]
        let mut final_string = String::new();
        final_string += "|**From**|**To**|**Course name**|**Course site**|**Provider site**|**Provider name**|\n|:----------|:----------|:---------------------------------------------------------------------------|:---------------|:-----------------|:-----------------|\n";
        for part in split {
            let mut restitched: String = String::new();
            let mut pipes_split: Vec<String> = part.split("|").map(|v| v.to_owned()).collect();
            pipes_split.retain(|v| v != "");
            pipes_split[2] = "<!-- DO NOT EDIT -->".to_owned() + &pipes_split[2];
            let mut i = 0;
            let len = pipes_split.len();
            for mut x in pipes_split {
                if  i < len{
                    x += "|";
                }
                restitched += &x;
                i += 1;
            }
            final_string += &(restitched + "\n");
        }
        fs::write(output_file.expect("No output file specified"), &final_string)?;

        return Ok(());
    }
    panic!("Either output or input file was not specified")
}
