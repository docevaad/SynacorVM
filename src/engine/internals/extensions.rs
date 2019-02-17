pub trait MemoryValue {
    fn is_valid_memory_address(&self) -> bool;
}

impl MemoryValue for u16 {
    fn is_valid_memory_address(&self) -> bool {
        if *self < (std::i16::MAX as u16 + 1) {
            true
        } else {
            false
        }
    }
}

pub trait RegisterValue {
    fn get_register_index(&self) -> u16;
    fn is_valid_register(&self) -> bool;
}

impl RegisterValue for u16 {
    fn get_register_index(&self) -> u16 {
        *self - (std::i16::MAX as u16 + 1)
    }

    fn is_valid_register(&self) -> bool {
        let register = self.get_register_index();
        if register <= 7 {
            true
        } else {
            false
        }
    }
}
