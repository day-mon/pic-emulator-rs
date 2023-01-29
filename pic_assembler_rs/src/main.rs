mod assembler;
mod tests;

//take filename from the command line and pass it to the assembler
use std::env;
use std::path::Path;
use std::process;
use std::string::String;
use std::vec::Vec;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || !args[1].ends_with(".pic") {
        println!("❌ Usage: {} <filename.pic>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let path = Path::new(filename);
    let display = path.display();

    
}

