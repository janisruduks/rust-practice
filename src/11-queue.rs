fn main() {
    let mut q = Queue::new();
    assert_eq!(q.remove(), None);
    let value = 100;
    q.add(value);
    assert_eq!(q.remove(), Some(value));
    q.add(value);
    q.add(value + value);
    assert_eq!(q.remove(), Some(value + value));
    assert_eq!(q.remove(), None);
}

struct Queue {
    store: Vec<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue { store: Vec::new() }
    }

    fn add(&mut self, value: i32) {
        self.store.push(value);
    }

    fn remove(&mut self) -> Option<i32> {
        self.store.pop()
    }
}
