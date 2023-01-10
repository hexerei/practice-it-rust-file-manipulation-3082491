use std::fs;
use std::io::Error;

fn read_file(file_path: &str) -> Result<String, Error> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

fn main() {
    let file_path = "test_file";

    // Implement the read_file function
    let contents = read_file(&file_path).expect(&format!("unable to read file <{}>", file_path));
    println!("{}", contents);
}
