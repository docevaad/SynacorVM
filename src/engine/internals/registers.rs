use super::extensions::RegisterValue;
use super::svm_constants::NUM_OF_REGISTERS;
use super::svm_error::SVMError;

pub struct Registers {
    registers: [u16; NUM_OF_REGISTERS],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: [0; NUM_OF_REGISTERS],
        }
    }

    pub fn get_register(&self, register: u16) -> Result<u16, SVMError> {
        if !register.is_valid_register() {
            Err(SVMError::InvalidRegister)
        } else {
            let register_index = register.get_register_index() as usize;
            Ok(self.registers[register_index])
        }
    }

    pub fn set_register(&mut self, register: u16, value: u16) -> Result<(), SVMError> {
        if !register.is_valid_register() {
            Err(SVMError::InvalidRegister)
        } else {
            let register_index = register.get_register_index() as usize;
            self.registers[register_index] = value;
            Ok(())
        }
    }
}