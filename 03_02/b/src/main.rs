fn read_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().map(|line| line.to_string()).collect())
}
fn main() {
    let file_path = "file_with_lines";

    let lines: Vec<String> = read_file(&file_path).expect(&format!("Error reading file <{}>", &file_path));
    println!("{:#?}", lines);
}
