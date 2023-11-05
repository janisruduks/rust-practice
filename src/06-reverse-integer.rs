fn main() {
    println!("{}", reverse(-90));
    println!("{}", reverse(-55));
    println!("{}", reverse(132));
}

fn reverse(value: i32) -> i32 {
    if value >= 0 {
        value
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    } else {
        let abs_value = value.abs();
        -abs_value
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
}
