#[derive(Debug)]
enum Error {
    Unknown,
    EmptyString,
}
fn get_words(content: &str) -> Result<Vec<String>, Error> {
    if content.is_empty() {
        return Err(Error::EmptyString);
    }
    Ok(content.split_whitespace().map(|word| word.to_string()).collect())

}
fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");

    let words = get_words(&contents);
    println!("{:?}", words);
}
