mod engine;

use engine::svm_program::SVMProgram;
use engine::svm_engine::SVMEngine;
use std::fs::File;
use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let program = SVMProgram::new(&file);
    program.print_program();
}
