use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let wanted_string = "th";
    let file_path = "file_with_lines";
    let file: File = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = String::from(line.unwrap());
        if line.contains(wanted_string) {
            println!("{}", line);
        }
    }
}
