#![allow(dead_code)]
#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { data: vec![] }
    }

    pub fn with_data(data: Vec<T>) -> Stack<T> {
        Stack { data }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();

        assert_eq!(stack.pop(), None);

        stack.push(1);
        stack.push(2);
        assert_eq!(stack.data.len(), 2);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.data.len(), 1);
    }
}
