use std::fs;

fn main() {
  let file_path = "test_file";
  let content = fs::read_to_string(file_path).expect("Unable to read file");
  println!("{}", content);
}