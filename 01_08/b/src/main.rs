use std::fs;

fn main() {
    let file_path = "my_file";
    let contents = "This is my new file! Congrats!!!".to_string();
    fs::write(file_path, contents).unwrap();
}
