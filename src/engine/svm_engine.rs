use super::internals::svm_engine_state::SVMEngineState;
use super::svm_program::SVMProgram;

pub struct SVMEngine {
    state: SVMEngineState
}

// impl SVMEngine {
//     pub fn new(program: SVMProgram) -> SVMEngine {
//         SVMEngine {
//             memory: Memory::new(program.get_bytecode()),
//         }
//     }
// }