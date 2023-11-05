fn main() {
    let result = longest_word("fun&!! time");
    println!("{}", result);
}

fn longest_word(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .fold(String::new(), |acc, word| {
            let trimmed_word: &str = word.trim_matches(|c: char| !c.is_alphanumeric());
            if trimmed_word.len() > acc.len() {
                trimmed_word.to_string()
            } else {
                acc
            }
        })
}
