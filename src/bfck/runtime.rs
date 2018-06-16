use bfck::tape::Tape;
use std::io::Read;
use std::io;

pub struct Runtime{
    tape: Tape,
    pc: usize,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            tape: Tape::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self, instructions: &Vec<Instruction>) {
        while self.pc < instructions.len() {
            match instructions[self.pc] {
                Instruction::RIGHT => self.tape.go_right(),
                Instruction::LEFT => self.tape.go_left(),
                Instruction::INC => {
                    let n = self.tape.load();
                    self.tape.store(n.wrapping_add(1));
                },
                Instruction::DEC => {
                    let n = self.tape.load();
                    self.tape.store(n.wrapping_sub(1));
                },
                Instruction::BNQZ(t) => {
                    if self.tape.load() == 0 {
                        self.pc = t;
                    }
                },
                Instruction::JMP(t) => self.pc = t,
                Instruction::READ => print!("{}", char::from(self.tape.load())),
                Instruction::WRITE => match io::stdin().bytes().next().expect("IO error") {
                    Ok(value) => self.tape.store(value),
                    Err(_) => {},
                },
                Instruction::NOP => {},
            }
            self.pc +=
                1;
        }
    }

}

pub mod compiler {
    use bfck::runtime::Instruction;
    pub fn compile(program_code: &Vec<u8>, instructions: &mut Vec<Instruction>) {
        let mut jmp_stack: Vec<Instruction> = Vec::new();
        let mut unoptimized: Vec<Instruction> = Vec::new();
        for i in 0..program_code.len() {
            match char::from(program_code[i]) {
                '>' => unoptimized.push(Instruction::RIGHT),
                '<' => unoptimized.push(Instruction::LEFT),
                '+' => unoptimized.push(Instruction::INC),
                '-' => unoptimized.push(Instruction::DEC),
                '[' => {
                    let mut j = i + 1;
                    let mut bcount = 1;
                    while bcount != 0 {
                        match char::from(program_code[i]) {
                            '[' => bcount += 1,
                            ']' => bcount -= 1,
                            _ => (),
                        }
                        j += 1;
                    }
                    jmp_stack.push(Instruction::JMP(i));
                    unoptimized.push(Instruction::BNQZ(j))
                },
                ']' => unoptimized.push(jmp_stack.pop().expect("compilation error")),
                '.' => unoptimized.push(Instruction::READ),
                ',' => unoptimized.push(Instruction::WRITE),
                _ => unoptimized.push(Instruction::NOP),
            }
        }
        optimize(&unoptimized, instructions);
    }


    fn optimize(instructions: &Vec<Instruction>, opt_instructions: &mut Vec<Instruction>) {
        let mut nop_count = 0;
        for ins in instructions.iter() {
            match *ins {
                Instruction::NOP => nop_count += 1,
                Instruction::JMP(v) => opt_instructions.push(Instruction::JMP(v - nop_count)),
                Instruction::BNQZ(v) => opt_instructions.push(Instruction::BNQZ(v - nop_count)),
                Instruction::READ => opt_instructions.push(Instruction::READ),
                Instruction::WRITE => opt_instructions.push(Instruction::WRITE),
                Instruction::LEFT => opt_instructions.push(Instruction::LEFT),
                Instruction::RIGHT => opt_instructions.push(Instruction::RIGHT),
                Instruction::INC => opt_instructions.push(Instruction::INC),
                Instruction::DEC => opt_instructions.push(Instruction::DEC),
            }
        }
    }
}
pub enum Instruction{
    RIGHT,
    LEFT,
    INC,
    DEC,
    JMP(usize),
    BNQZ(usize),
    READ,
    WRITE,
    NOP,
}

