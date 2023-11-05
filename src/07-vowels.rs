fn main() {
    let word_one = "Adam";
    println!("{} has {} vowels", word_one, vowels(word_one));
    let word_two = "aeiou";
    println!("{} has {} vowels", word_two, vowels(word_two));
    let word_three = "xxxxps";
    println!("{} has {} vowels", word_three, vowels(word_three));
}

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

fn vowels(input: &str) -> i32 {
    let mut count: i32 = 0;
    let input_to_lower = input.to_lowercase();
    let characters = input_to_lower.chars();
    for character in characters {
        if VOWELS.iter().any(|&x| x.contains(character)) {
            count += 1;
        }
    }
    return count;
}
