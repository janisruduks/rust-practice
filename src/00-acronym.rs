fn main() {
    fn parse(input: &str) -> String {
        input
            .split_whitespace()
            .flat_map(|word| word.chars().filter(|&c| c.is_alphanumeric()).next())
            .collect::<String>()
            .to_uppercase()
    }

    let sentence: &str = "Three, _ _letter_ Acronyms";
    println!("{}", parse(sentence));
}
