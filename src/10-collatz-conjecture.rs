fn main() {
    let num_1 = 12;
    let num_2 = 16;
    let num_3 = 0;
    println!(
        "for number {}, {} steps required",
        num_1,
        collatz_conjecture(num_1)
    );
    println!(
        "for number {}, {} steps required",
        num_2,
        collatz_conjecture(num_2)
    );
    println!(
        "for number {}, {} steps required",
        num_3,
        collatz_conjecture(num_3)
    );
}

fn collatz_conjecture(value: i32) -> i32 {
    if value <= 0 {
        panic!("Number cannot be negative!");
    }
    let mut number = value;
    let mut steps = 0;
    while number != 1 {
        if number % 2 == 1 {
            number = number * 3 + 1;
        } else {
            number /= 2;
        }
        steps += 1;
    }
    steps
}
