use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file_name = "file_with_lines";
    let mut file = File::open(file_name).expect("Could not open file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
      println!("{}", line.unwrap());
    }
}
