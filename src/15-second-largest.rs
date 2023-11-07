fn main() {
    let input = vec![10, 20, 50, 5, 100];
    assert_eq!(second_largest(input), Some(50));
    let input = vec![3, 1, 2];
    assert_eq!(second_largest(input), Some(2));
}

fn second_largest(mut input: Vec<i32>) -> Option<i32> {
    input.sort_by(|a, b| b.cmp(a));
    input.get(1).cloned()
}
