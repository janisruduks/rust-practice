fn main() {
    pyramid(6);
}

fn pyramid(mut size: i32) {
    let symbol = "#";
    let space = " ";
    for count in 0..size + 1 {
        println!(
            "{}{}{}",
            space.repeat(size as usize),
            symbol.repeat(count as usize * 2),
            space.repeat(size as usize)
        );
        size -= 1;
    }
}
