const STACK_SIZE: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct Stack {
    pub memory: [u16; STACK_SIZE],
    pub sp: usize,
}
impl Stack {
    pub fn new() -> Self {
        Stack {
            memory: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        if self.sp < STACK_SIZE {
            self.memory[self.sp] = value;
            self.sp += 1;
        } else {
            dbg!(self);
            panic!("Stack overflow!")
        }
    }

    fn pop(&mut self) -> Option<u16> {
        if self.sp > 0 {
            self.sp -= 1;
            let val = self.memory[self.sp];
            self.memory[self.sp] = 0;
            Some(val)
        } else {
            None
        }
    }
}
