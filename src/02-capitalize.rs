fn main() {
    println!("{}", capitalize("i love breakfast at bill miller bbq"));
}

fn capitalize(value: &str) -> String {
    let capitalized_words: Vec<String> = value
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first_char) => {
                    let rest_of_word: String = chars.collect();
                    first_char.to_ascii_uppercase().to_string() + &rest_of_word
                }
            }
        })
        .collect();
    capitalized_words.join(" ")
}
