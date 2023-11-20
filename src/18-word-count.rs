use std::collections::HashMap;

fn main() {
    let x = word_count("olly olly in come free");
    let mut hashmap1: HashMap<&str, i32> = HashMap::new();
    hashmap1.insert("free", 1);
    hashmap1.insert("in", 1);
    hashmap1.insert("come", 1);
    hashmap1.insert("olly", 2);
    assert_eq!(x, hashmap1);

    let y = word_count("car : carpet as java : javascript!!&@$%^&");
    let mut hashmap2: HashMap<&str, i32> = HashMap::new();
    hashmap2.insert("car", 1);
    hashmap2.insert("carpet", 1);
    hashmap2.insert("java", 1);
    hashmap2.insert(":", 2);
    hashmap2.insert("as", 1);
    hashmap2.insert("javascript!!&@$%^&", 1);
    assert_eq!(y, hashmap2);
}

fn word_count(input: &str) -> HashMap<&str, i32> {
    let mut char_count = HashMap::new();
    let words = input.split(' ');

    for word in words {
        *char_count.entry(word).or_insert(0) += 1;
    }
    char_count
}
