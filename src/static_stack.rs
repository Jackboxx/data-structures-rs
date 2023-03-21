#![allow(dead_code)]
#[derive(Debug)]
pub struct StaticStack<T: Copy, const S: usize> {
    data: [T; S],
    top_of_stack: i32,
}

#[derive(Debug)]
pub enum StaticStackError {
    StackIsFull,
}

impl<T: Default + Copy, const S: usize> StaticStack<T, S> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); S],
            top_of_stack: -1,
        }
    }
}

impl<T: Copy, const S: usize> StaticStack<T, S> {
    pub fn with_data(data: [T; S]) -> Self {
        let top_of_stack = data.len() as i32 - 1;
        Self { data, top_of_stack }
    }

    pub fn push(&mut self, item: T) -> Result<(), StaticStackError> {
        if self.top_of_stack >= self.data.len() as i32 - 1 {
            return Err(StaticStackError::StackIsFull);
        }

        self.top_of_stack += 1;
        self.data[self.top_of_stack as usize] = item;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top_of_stack < 0 {
            return None;
        }

        let item = self.data[self.top_of_stack as usize];
        self.top_of_stack -= 1;

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_stack() {
        const LEN: usize = 5;
        let mut static_stack = StaticStack::<i32, LEN>::new();

        assert_eq!(static_stack.data.len(), LEN);

        for i in 0..LEN {
            static_stack.push(i as i32).unwrap();
        }

        assert!(static_stack.push(1).is_err());

        for i in 0..LEN {
            assert_eq!(static_stack.pop(), Some((LEN - (i + 1)) as i32));
        }

        assert_eq!(static_stack.pop(), None);
    }
}
