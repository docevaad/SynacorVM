use super::internals::svm_engine_state::SVMEngineState;
use super::internals::svm_error::SVMError;
use super::internals::opcode::{OpcodeValue, SVMOpCode, OpCode};
use super::svm_program::SVMProgram;

pub struct SVMEngine {
    engine_state: SVMEngineState
}

impl SVMEngine {
    pub fn new(program: SVMProgram) -> SVMEngine {
        SVMEngine {
            engine_state: SVMEngineState::new(program.get_bytecode())
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode_number = self.engine_state.instruction_pointer.get_next_memory_value(&self.engine_state.memory);
            match opcode_number {
                Ok(opcode_value) => match opcode_value.get_opcode() {
                    Ok(opcode) => match opcode.dispatch(&mut self.engine_state) {
                        Err(error) => self.print_error(error),
                        _ => {}
                    },
                    Err(error) => self.print_error(error)
                },
                Err(error) => self.print_error(error)
            };
        }
    }

    fn print_error(&self, error: SVMError) {
        println!("Error at instruction: {}", self.engine_state.instruction_pointer.get_ip());
        std::process::exit(1);
    }
}