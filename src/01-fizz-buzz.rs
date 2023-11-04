fn main() {
    fizz_buzz(100);
}

fn fizz_buzz(number: i32) {
    for i in 1..=number {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}
