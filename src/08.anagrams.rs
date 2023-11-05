fn main() {
    let value_1 = "rail safety";
    let value_2 = "fairy tales";
    let value_3 = "hello_ _world!";
    let value_4 = "HELLO WORLD";
    let value_5 = "Hugh";
    let value_6 = "Jass";
    println!(
        "is '{}' and '{}' an anagram: {}",
        value_1,
        value_2,
        anagrams(value_1, value_2)
    );
    println!(
        "is '{}' and '{}' an anagram: {}",
        value_3,
        value_4,
        anagrams(value_3, value_4)
    );
    println!(
        "is '{}' and '{}' an anagram: {}",
        value_5,
        value_6,
        anagrams(value_5, value_6)
    );
}

fn anagrams(value_1: &str, value_2: &str) -> bool {
    if clean_and_sort(value_1) == clean_and_sort(value_2) {
        return true;
    }
    false
}

fn clean_and_sort(value: &str) -> String {
    let mut char_vec: Vec<char> = value
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    char_vec.sort();
    char_vec.into_iter().collect()
}
