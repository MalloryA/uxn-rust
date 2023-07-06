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

    pub fn pop(&mut self) -> Option<u8> {
        self.vec.pop()
    }

    pub fn dup(&mut self) {
        self.vec.push(*self.vec.last().unwrap())
    }

    pub fn dupk(&mut self) {
        let value = *self.vec.last().unwrap();
        self.vec.push(value);
        self.vec.push(value);
    }
}
