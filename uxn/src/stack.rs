use crate::error::Error;

pub struct Stack {
    vec: Vec<u8>,
}

impl Stack {
    pub fn new(vec: Vec<u8>) -> Stack {
        Stack { vec }
    }

    pub fn as_vec(self) -> Vec<u8> {
        self.vec
    }

    pub fn pop(&mut self) -> Result<u8, Error> {
        match self.vec.pop() {
            Some(value) => Ok(value),
            None => Err(Error::Underflow),
        }
    }

    pub fn dup(&mut self) -> Result<(), Error> {
        let value = self.pop()?;
        self.vec.push(value);
        self.vec.push(value);
        Ok(())
    }

    pub fn dupk(&mut self) -> Result<(), Error> {
        self.dup()?;
        self.dup()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EMPTY: Vec<u8> = vec![];

    #[test]
    fn pop_non_empty() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Ok(1), stack.pop());
        assert_eq!(EMPTY, stack.as_vec());
    }

    #[test]
    fn pop_empty() {
        let mut stack = Stack::new(vec![]);
        assert_eq!(Err(Error::Underflow), stack.pop());
        assert_eq!(EMPTY, stack.as_vec());
    }

    #[test]
    fn dup_non_empty() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Ok(()), stack.dup());
        assert_eq!(vec!(1, 1), stack.as_vec());
    }

    #[test]
    fn dup_empty() {
        let mut stack = Stack::new(vec![]);
        assert_eq!(Err(Error::Underflow), stack.dup());
        assert_eq!(EMPTY, stack.as_vec());
    }

    #[test]
    fn dupk_non_empty() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Ok(()), stack.dupk());
        assert_eq!(vec!(1, 1, 1), stack.as_vec());
    }

    #[test]
    fn dupk_empty() {
        let mut stack = Stack::new(vec![]);
        assert_eq!(Err(Error::Underflow), stack.dupk());
        assert_eq!(EMPTY, stack.as_vec());
    }
}
