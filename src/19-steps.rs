fn main() {
    steps(3, "#");
}

fn steps(mut size: usize, symbol: &str) {
    for char_count in 1..size + 1 {
        println!("{}{}", symbol.repeat(char_count), " ".repeat(size));
        size -= 1;
    }
}
