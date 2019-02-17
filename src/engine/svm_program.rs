use std::fs::File;
use std::io::{Read, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use super::internals::svm_constants::MEMORY_SIZE_MAX as PROGRAM_SIZE_MAX;

pub type ByteCodeArray = [u16; PROGRAM_SIZE_MAX];

pub struct SVMProgram {
    bytecode_size: usize,
    bytecode: ByteCodeArray,
}

impl SVMProgram {
    pub fn new(mut file: &File) -> SVMProgram {
        let mut file_data = Vec::new(); 
        let file_size = file.read_to_end(&mut file_data).unwrap() as u64;
        let mut file_data_cursor = Cursor::new(file_data);
        let mut bytecode_size = 0;
        let mut bytecode: ByteCodeArray = [0; PROGRAM_SIZE_MAX];
        while file_data_cursor.position() < file_size {
            bytecode[bytecode_size] = file_data_cursor.read_u16::<LittleEndian>().unwrap();
            bytecode_size += 1;
        }
        SVMProgram {
            bytecode_size: bytecode_size,
            bytecode: bytecode
        }
    }

    pub fn print_program(&self) {
        for i in 0..self.bytecode_size {
            println!("{}: {}", i, self.bytecode[i]);
        }
    }

    pub(super) fn get_bytecode(&self) -> ByteCodeArray {
        self.bytecode
    }
}