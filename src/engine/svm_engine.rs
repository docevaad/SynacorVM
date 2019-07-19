use super::internals::svm_engine_state::SVMEngineState;
use super::internals::svm_error::SVMError;
use super::internals::opcode::{OpcodeValue, OpCode};
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
                Ok(opcode_value) => { /*println!("IP:{}, Opcode Value:{}\n", self.engine_state.instruction_pointer.get_ip(), opcode_value); */match opcode_value.get_opcode() {
                    Ok(opcode) => match opcode.dispatch(&mut self.engine_state) {
                        Err(error) => self.print_error(error),
                        _ => {}
                    },
                    Err(error) => self.print_error(error)
                }},
                Err(error) => self.print_error(error)
            };
        }
    }

    fn print_error(&self, error: SVMError) {
        let ip = self.engine_state.instruction_pointer.get_ip();
        let opcode = match self.engine_state.memory.load_memory(ip) {
            Ok(x) => x,
            _ => 0,
        };
        println!("Error at instruction: {}, opcode: {} ", ip, opcode);
        match error {
            SVMError::InvalidMemory => println!("Memory Error"),
            SVMError::InvalidOpCode => println!("Invalid Opcode"),
            SVMError::InvalidRegister => println!("Invalid Register"),
            SVMError::ReadError => println!("Read error"),
            SVMError::StackEmpty => println!("Stack error"),
            SVMError::WriteError => println!("Write error"),
        }
        std::process::exit(1);
    }
}