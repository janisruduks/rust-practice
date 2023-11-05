fn main() {
    println!("{}", reverse("Greetings!"))
}

fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}
