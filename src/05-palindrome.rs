fn main() {
    let word_one = "abba";
    let word_two = "asdvsads";
    println!("Is {} a palindrome: {}", word_one, palindrome(word_one));
    println!("Is {} a palindrome: {}", word_two, palindrome(word_two));
}

fn palindrome(input: &str) -> bool {
    let x = input.chars().rev().collect::<String>();
    if input.to_string() == x {
        return true;
    }
    return false;
}
