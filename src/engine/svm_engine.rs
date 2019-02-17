use super::internals::svm_engine_state::SVMEngineState;
use super::svm_program::SVMProgram;

pub struct SVMEngine {
    state: SVMEngineState
}

impl SVMEngine {
    pub fn new(program: SVMProgram) -> SVMEngine {
        SVMEngine {
            state: SVMEngineState::new(program.get_bytecode())
        }
    }

    pub fn run() {

    }
}