pub struct Tape{
    tape_vec: Vec<u8>,
    pointer: usize,
}

impl Tape {

    pub fn new() -> Tape {
        Tape {
            tape_vec: vec![0, 0],
            pointer: 0,
        }
    }

    pub fn go_right(&mut self){
        if (self.pointer % 2) == 0 {
            if self.pointer >= (self.tape_vec.len() - 2) {
                self.tape_vec.push(0);
                self.tape_vec.push(0);
            }
            self.pointer += 2;
        }
        else {
            if self.pointer == 1 {
                self.pointer = 0;
            }
            self.pointer -= 2;
        }
    }

    pub fn go_left(&mut self){
        if (self.pointer % 2) == 0 {
            if self.pointer == 0 {
                self.pointer = 1;
            }
            self.pointer -= 2;
        }
        else {
            if self.pointer >= (self.tape_vec.len() - 1) {
                self.tape_vec.push(0);
                self.tape_vec.push(0);
            }
            self.pointer += 2;
        }
    }

    pub fn load(&self) -> u8 {
        self.tape_vec[self.pointer]
    }

    pub fn store(&mut self, value: u8) {
        self.tape_vec[self.pointer] = value;
    }
}