pub struct StackArray<T> {
    elems: Vec<T>,
    capacity: usize,
    top: usize,
}

impl<T> StackArray<T> {
    pub fn new(size: usize) -> Self {
        StackArray {
            elems: Vec::with_capacity(size),
            capacity: size,
            top: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.is_full() {
            println!("Stack is full!");
            return;
        }
        self.top = self.top + 1;
        self.elems.push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            println!("Stack is empty!");
        }
        self.top = self.top - 1;
        self.elems.pop()
    }

    pub fn peek(&self) -> &T {
        if self.is_empty() {
            println!("Stack is empty!");
        }
        &self.elems[self.top - 1]
    }

    pub fn size(&self) -> usize {
        return self.top;
    }

    pub fn is_empty(&self) -> bool {
        return self.elems.is_empty();
    }

    pub fn is_full(&self) -> bool {
        return self.top == self.capacity - 1;
    }
}

#[test]
fn test_push() {
    let mut stack = StackArray::new(2);
    stack.push(9);
    assert_eq!(stack.size(), 1);
    assert_eq!(*stack.peek(), 9);
    assert_eq!(stack.pop(), Some(9));
    assert_eq!(stack.size(), 0);
}
