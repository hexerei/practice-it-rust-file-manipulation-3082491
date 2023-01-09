use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_name = "file_with_lines";
    let mut file = File::open(file_name).expect("Could not open file!");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    for line in content.lines() {
        println!("{}", line);
    }
}
