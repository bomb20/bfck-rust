//Author: Vincent Truchseß <redtux@posteo.net>
//License: GPLv3
//Date: 13.04.2018

use std::io::Read;
//use std::fmt;
//use std::io;
use std::env;
use std::fs::File;

mod bfck;

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
    let mut instructions: Vec<bfck::runtime::Instruction> = vec![];
    bfck::runtime::compiler::compile(&programm, &mut instructions);

    let mut runtime = bfck::runtime::Runtime::new();

    runtime.run(&instructions);

}
