fn main() {
    let mut q = Stack::new();
    assert_eq!(q.pop(), None);
    assert_eq!(q.peak(), None);
    let value = 100;
    assert_eq!(q.push(value), 100);
    assert_eq!(q.pop(), Some(value));
    assert_eq!(q.push(value), 100);
    assert_eq!(q.push(value + value), value + value);
    assert_eq!(q.peak(), Some(value + value));
    assert_eq!(q.pop(), Some(value + value));
    assert_eq!(q.pop(), Some(value));
    assert_eq!(q.pop(), None);
}

struct Stack {
    store: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { store: Vec::new() }
    }

    fn push(&mut self, value: i32) -> i32 {
        self.store.push(value);
        value
    }

    fn peak(&self) -> Option<i32> {
        self.store.last().cloned()
    }

    fn pop(&mut self) -> Option<i32> {
        self.store.pop()
    }
}
