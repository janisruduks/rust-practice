fn main() {
    let a = 9;
    let b = 153;
    let c = 154;
    println!("is {} an Armstrong number: {}", a, validate(a));
    println!("is {} an Armstrong number: {}", b, validate(b));
    println!("is {} an Armstrong number: {}", c, validate(c));
}

fn validate(value: u32) -> bool {
    let characters: Vec<char> = value.to_string().chars().collect();
    let char_count: usize = characters.len();
    let mut sum = 0;
    for character in characters {
        sum += character.to_digit(10).unwrap().pow(char_count as u32);
    }
    sum == value
}
