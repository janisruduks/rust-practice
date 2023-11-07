fn main() {
    assert_eq!(max_char("apple 1231111".to_string()), '1');
    assert_eq!(max_char("abcdefghijklmnaaaaa".to_string()), 'a');
    assert_eq!(max_char("ascxxc..xcsd.......we12..".to_string()), '.');
}

fn max_char(input: String) -> char {
    let letters: Vec<char> = input.chars().collect();
    let mut max_count = 0;
    let mut most_common_char = ' ';
    for &letter in &letters {
        let current = letters.iter().filter(|&&c| c == letter).count();
        if current > max_count {
            max_count = current;
            most_common_char = letter;
        }
    }
    most_common_char
}
