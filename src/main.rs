
use std::io::Read;
//use std::fmt;
//use std::io;
use std::env;
use std::fs::File;

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("No source-file path found!");
        return;
    }

    let path = &args[1];
    let mut file = File::open(path).expect("unable to open file");
    let mut programm: Vec<u8> = Vec::new();
    file.read_to_end(&mut programm).expect("could not read programm file");
    let mut instruction_pointer = 0;
    let mut tape = Tape::new();
    let mut jmp_stack: Vec<usize> = Vec::new();

    while instruction_pointer < programm.len() {
        match char::from(programm[instruction_pointer]) {
            '>' => tape.go_right(),
            '<' => tape.go_left(),
            '+' => tape.inc(),
            '-' => tape.dec(),
            '[' => {
                if tape.get() != 0 {
                    jmp_stack.push(instruction_pointer);
                } else {
                    let mut bcount = 1;
                    while bcount != 0 {
                        instruction_pointer += 1;
                        if char::from(programm[instruction_pointer]) == '[' {
                            bcount += 1;
                        } else if char::from(programm[instruction_pointer]) == ']' {
                            bcount -= 1;
                        }
                    }
                }
            },
            ']' => {
                instruction_pointer = jmp_stack.pop().expect("return jump error!");
                continue;
            },
            ',' => {
                match std::io::stdin().bytes().next().expect("stdin IO error!") {
                    Ok(value) => tape.put(value),
                    Err(_) => {},
                };
            },
            '.' => print!("{}", char::from(tape.get())),
            _ => {},
        }
        instruction_pointer += 1;
    }

}

struct Tape {
    right_tape: Vec<u8>,
    left_tape: Vec<u8>,
    switch: TapeSwitch,
    data_pointer: usize,
}

impl Tape {
    fn go_right(&mut self){
        match self.switch {
            TapeSwitch::RIGHT => {
                if self.data_pointer == self.right_tape.len() - 1 {
                    self.right_tape.push(0);
                }
                self.data_pointer += 1;
            }
            TapeSwitch::LEFT => {
                if self.data_pointer == 0 {
                    self.switch = TapeSwitch::RIGHT;
                } else if self.data_pointer == self.left_tape.len() - 1
                    && self.left_tape[self.data_pointer] == 0 {
                    self.left_tape.pop();
                    self.data_pointer -= 1;
                } else {
                    self.data_pointer -= 1;
                }
            }
        }
    }
    fn go_left(&mut self) {
        match self.switch {
            TapeSwitch::RIGHT => {
                if self.data_pointer == 0 {
                    self.switch = TapeSwitch::LEFT;
                } else if self.data_pointer == self.right_tape.len() - 1
                    && self.right_tape[self.data_pointer] == 0 {
                    self.right_tape.pop();
                    self.data_pointer -= 1;
                } else {
                    self.data_pointer -= 1;
                }
            }
            TapeSwitch::LEFT => {
                if self.data_pointer == self.left_tape.len() - 1 {
                    self.left_tape.push(0);
                }
                self.data_pointer += 1;
            }
        }
    }
    fn inc(&mut self){
        let cell: &mut u8;
        match self.switch {
            TapeSwitch::RIGHT => cell = &mut self.right_tape[self.data_pointer],
            TapeSwitch::LEFT => cell = &mut self.left_tape[self.data_pointer],
        }
        *cell = (*cell).wrapping_add(1);
    }
    fn dec(&mut self) {
        let cell: &mut u8;
        match self.switch {
            TapeSwitch::RIGHT => cell = &mut self.right_tape[self.data_pointer],
            TapeSwitch::LEFT => cell = &mut self.left_tape[self.data_pointer],
        }
        *cell = (*cell).wrapping_sub(1);
    }
    fn get(&self) -> u8 {
        match self.switch {
            TapeSwitch::RIGHT => self.right_tape[self.data_pointer],
            TapeSwitch::LEFT => self.left_tape[self.data_pointer],
        }
    }
    fn put(&mut self, value: u8) {
        match self.switch {
            TapeSwitch::RIGHT => self.right_tape[self.data_pointer] = value,
            TapeSwitch::LEFT => self.left_tape[self.data_pointer] = value,
        }
    }
    fn new() -> Tape {
        Tape{
            data_pointer: 0,
            right_tape: vec![0],
            left_tape: vec![0],
            switch: TapeSwitch::RIGHT,
        }
    }
}

enum TapeSwitch {
    RIGHT,
    LEFT,
}