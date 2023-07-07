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

    fn _pop(&mut self) -> Result<u8, Error> {
        match self.vec.pop() {
            Some(value) => Ok(value),
            None => Err(Error::Underflow),
        }
    }

    pub fn pop(&mut self) -> Result<(), Error> {
        match self._pop() {
            Ok(value) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn ovr(&mut self) -> Result<(), Error> {
        if self.vec.len() < 2 {
            Err(Error::Underflow)
        } else {
            let value = self.vec.get(self.vec.len() - 2).unwrap();
            self.vec.push(*value);
            Ok(())
        }
    }

    pub fn dup(&mut self) -> Result<(), Error> {
        let value = self._pop()?;
        self.vec.push(value);
        self.vec.push(value);
        Ok(())
    }

    pub fn dupk(&mut self) -> Result<(), Error> {
        self.dup()?;
        self.dup()?;
        Ok(())
    }

    pub fn swp(&mut self) -> Result<(), Error> {
        if self.vec.len() < 2 {
            Err(Error::Underflow)
        } else {
            let a = self._pop()?;
            let b = self._pop()?;
            self.vec.push(a);
            self.vec.push(b);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EMPTY: Vec<u8> = vec![];

    #[test]
    fn pop_non_empty() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Ok(()), stack.pop());
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

    #[test]
    fn ovr_two_values() {
        let mut stack = Stack::new(vec![1, 2]);
        assert_eq!(Ok(()), stack.ovr());
        assert_eq!(vec!(1, 2, 1), stack.as_vec());
    }

    #[test]
    fn ovr_one_value() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Err(Error::Underflow), stack.ovr());
        assert_eq!(vec!(1), stack.as_vec());
    }

    #[test]
    fn ovr_empty() {
        let mut stack = Stack::new(vec![]);
        assert_eq!(Err(Error::Underflow), stack.ovr());
        assert_eq!(EMPTY, stack.as_vec());
    }

    #[test]
    fn swp_two_values() {
        let mut stack = Stack::new(vec![1, 2]);
        assert_eq!(Ok(()), stack.swp());
        assert_eq!(vec!(2, 1), stack.as_vec());
    }

    #[test]
    fn swp_one_value() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Err(Error::Underflow), stack.swp());
        assert_eq!(vec!(1), stack.as_vec());
    }

    #[test]
    fn swp_empty() {
        let mut stack = Stack::new(vec![]);
        assert_eq!(Err(Error::Underflow), stack.swp());
        assert_eq!(EMPTY, stack.as_vec());
    }
}
