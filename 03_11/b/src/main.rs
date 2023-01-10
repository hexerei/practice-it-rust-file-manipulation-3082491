fn main() {
    let file_path = "file_with_lines";

    let lines = read_file(&file_path).expect(&format!("Unable to read file <{}>", &file_path));
    println!("{:?}", lines);
}

fn read_file(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let contents = std::fs::read_to_string(path)?;
    Ok(
        contents
            .lines()
            .map(|line|
                line.split_whitespace()
                    .map(|word| word.to_string())
                    .collect::<Vec<String>>()
            ).collect()
        )
}