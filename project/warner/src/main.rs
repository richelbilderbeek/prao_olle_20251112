use std::{env::args, fs::{self}, io::Read, process::exit};

use crate::mixed::warn;
mod mixed;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = args().collect();
    if args.get(1) != None {
        fs::write(&args[1], warn(fs::read_to_string(&args[1])?))?;
        return Ok(());
    }
    panic!()
}
