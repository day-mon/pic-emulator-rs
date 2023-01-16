//take filename from the command line and pass it to the assembler
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;
use std::str::FromStr;
use std::string::String;
use std::vec::Vec;
use pic_emulator_rs::nbitnumber;

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

