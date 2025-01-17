fn main() {
    let file_path = "words_to_file";
    let words = vec![
        "Words".to_string(),
        "of".to_string(),
        "the".to_string(),
        "first".to_string(),
        "line".to_string(),
    ];

    // Implement the write_words_to_file function
    write_words_to_file(&file_path, &words).expect("Could not write to file...");
}

fn write_words_to_file(path: &str, words: &Vec<String>) -> Result<(), std::io::Error> {
    let text: String = words.join(" ");
    std::fs::write(path, text)?;
    Ok(())
}