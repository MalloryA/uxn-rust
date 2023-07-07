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

    fn _remove(&mut self, offset: usize) -> Result<u8, Error> {
        if offset > self.vec.len() {
            Err(Error::Underflow)
        } else {
            let index = self.vec.len() - offset;
            Ok(self.vec.remove(index))
        }
    }

    fn _get(&mut self, offset: usize) -> Result<u8, Error> {
        if offset > self.vec.len() {
            Err(Error::Underflow)
        } else {
            let index = self.vec.len() - offset;
            Ok(*self.vec.get(index).unwrap())
        }
    }

    pub fn pop(&mut self) -> Result<(), Error> {
        match self._remove(1) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn nip(&mut self) -> Result<(), Error> {
        match self._remove(2) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn ovr(&mut self) -> Result<(), Error> {
        let value = self._get(2)?;
        self.vec.push(value);
        Ok(())
    }

    pub fn ovrk(&mut self) -> Result<(), Error> {
        let a = self._get(1)?;
        let b = self._get(2)?;
        self.vec.push(b);
        self.vec.push(a);
        self.vec.push(b);
        Ok(())
    }

    pub fn dup(&mut self) -> Result<(), Error> {
        let value = self._get(1)?;
        self.vec.push(value);
        Ok(())
    }

    pub fn dupk(&mut self) -> Result<(), Error> {
        self.dup()?;
        self.dup()?;
        Ok(())
    }

    pub fn swp(&mut self) -> Result<(), Error> {
        let value = self._remove(2)?;
        self.vec.push(value);
        Ok(())
    }

    pub fn swpk(&mut self) -> Result<(), Error> {
        let a = self._get(1)?;
        let b = self._get(2)?;
        self.vec.push(a);
        self.vec.push(b);
        Ok(())
    }

    pub fn rotk(&mut self) -> Result<(), Error> {
        let a = self._get(1)?;
        let b = self._get(2)?;
        let c = self._get(3)?;
        self.vec.push(b);
        self.vec.push(a);
        self.vec.push(c);
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
    fn ovrk_two_values() {
        let mut stack = Stack::new(vec![1, 2]);
        assert_eq!(Ok(()), stack.ovrk());
        assert_eq!(vec!(1, 2, 1, 2, 1), stack.as_vec());
    }

    #[test]
    fn ovrk_one_value() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Err(Error::Underflow), stack.ovrk());
        assert_eq!(vec!(1), stack.as_vec());
    }

    #[test]
    fn ovrk_empty() {
        let mut stack = Stack::new(vec![]);
        assert_eq!(Err(Error::Underflow), stack.ovrk());
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

    #[test]
    fn nip_two_values() {
        let mut stack = Stack::new(vec![1, 2]);
        assert_eq!(Ok(()), stack.nip());
        assert_eq!(vec!(2), stack.as_vec());
    }

    #[test]
    fn nip_one_value() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Err(Error::Underflow), stack.nip());
        assert_eq!(vec!(1), stack.as_vec());
    }

    #[test]
    fn nip_empty() {
        let mut stack = Stack::new(vec![]);
        assert_eq!(Err(Error::Underflow), stack.nip());
        assert_eq!(EMPTY, stack.as_vec());
    }

    #[test]
    fn swpk_two_values() {
        let mut stack = Stack::new(vec![1, 2]);
        assert_eq!(Ok(()), stack.swpk());
        assert_eq!(vec!(1, 2, 2, 1), stack.as_vec());
    }

    #[test]
    fn swpk_one_value() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Err(Error::Underflow), stack.swpk());
        assert_eq!(vec!(1), stack.as_vec());
    }

    #[test]
    fn swpk_empty() {
        let mut stack = Stack::new(vec![]);
        assert_eq!(Err(Error::Underflow), stack.swpk());
        assert_eq!(EMPTY, stack.as_vec());
    }

    #[test]
    fn rotk_three_values() {
        let mut stack = Stack::new(vec![1, 2, 3]);
        assert_eq!(Ok(()), stack.rotk());
        assert_eq!(vec!(1, 2, 3, 2, 3, 1), stack.as_vec());
    }

    #[test]
    fn rotk_two_values() {
        let mut stack = Stack::new(vec![1, 2]);
        assert_eq!(Err(Error::Underflow), stack.rotk());
        assert_eq!(vec!(1, 2), stack.as_vec());
    }

    #[test]
    fn rotk_one_value() {
        let mut stack = Stack::new(vec![1]);
        assert_eq!(Err(Error::Underflow), stack.rotk());
        assert_eq!(vec!(1), stack.as_vec());
    }

    #[test]
    fn rotk_empty() {
        let mut stack = Stack::new(vec![]);
        assert_eq!(Err(Error::Underflow), stack.rotk());
        assert_eq!(EMPTY, stack.as_vec());
    }
}
