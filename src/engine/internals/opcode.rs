use super::extensions::RegisterValue;
use super::svm_engine_state::SVMEngineState;
use super::svm_error::SVMError;

use std::io::{Read,Write};

pub trait OpcodeValue {
    fn get_opcode(&self) -> Result<SVMOpCode, SVMError>;
}

impl OpcodeValue for u16 {
    fn get_opcode(&self) -> Result<SVMOpCode, SVMError> {
        match *self {
            0 => Ok(SVMOpCode::Halt),
            1 => Ok(SVMOpCode::Set),
            2 => Ok(SVMOpCode::Push),
            3 => Ok(SVMOpCode::Pop),
            4 => Ok(SVMOpCode::Eq),
            5 => Ok(SVMOpCode::Gt),
            6 => Ok(SVMOpCode::Jmp),
            7 => Ok(SVMOpCode::Jt),
            8 => Ok(SVMOpCode::Jf),
            9 => Ok(SVMOpCode::Add),
            10 => Ok(SVMOpCode::Mult),
            11 => Ok(SVMOpCode::Mod),
            12 => Ok(SVMOpCode::And),
            13 => Ok(SVMOpCode::Or),
            14 => Ok(SVMOpCode::Not),
            15 => Ok(SVMOpCode::Rmem),
            16 => Ok(SVMOpCode::Wmem),
            17 => Ok(SVMOpCode::Call),
            18 => Ok(SVMOpCode::Ret),
            19 => Ok(SVMOpCode::Out),
            20 => Ok(SVMOpCode::In),
            21 => Ok(SVMOpCode::NoOp),
            _ => Err(SVMError::InvalidOpCode)
        }
    }
}

pub trait OpCode {
    fn dispatch(&self, engine_state: &mut SVMEngineState) -> Result<(), SVMError>;
}

pub enum SVMOpCode {
    Halt,
    Set,
    Push,
    Pop,
    Eq,
    Gt,
    Jmp,
    Jt,
    Jf,
    Add,
    Mult,
    Mod,
    And,
    Or,
    Not,
    Rmem,
    Wmem,
    Call,
    Ret,
    Out,
    In,
    NoOp
}

impl OpCode for SVMOpCode {
    fn dispatch(&self, engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
        match *self {
            SVMOpCode::Halt => halt(engine_state),
            SVMOpCode::Set => set(engine_state),
            SVMOpCode::Push => push(engine_state),
            SVMOpCode::Pop => pop(engine_state),
            SVMOpCode::Eq => eq(engine_state),
            SVMOpCode::Gt => gt(engine_state),
            SVMOpCode::Jmp => jmp(engine_state),
            SVMOpCode::Jt => jt(engine_state),
            SVMOpCode::Jf => jf(engine_state),
            SVMOpCode::Add => add(engine_state),
            SVMOpCode::Mult => mult(engine_state),
            SVMOpCode::Mod => modulus(engine_state),
            SVMOpCode::And => and(engine_state),
            SVMOpCode::Or => or(engine_state),
            SVMOpCode::Not => not(engine_state),
            SVMOpCode::Rmem => rmem(engine_state),
            SVMOpCode::Wmem => wmem(engine_state),
            SVMOpCode::Call => call(engine_state),
            SVMOpCode::Ret => ret(engine_state),
            SVMOpCode::Out => output(engine_state),
            SVMOpCode::In => input(engine_state),
            SVMOpCode::NoOp => noop(engine_state),
        }
    }
}

//  Opcode Implementations as functions
//  NOTE: Some of the names are inconsistent. This is due to them being keywords as well
fn halt(_engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    println!("Halted.");
    std::process::exit(0);
}

fn set(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let register = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let potential_register =  engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let value = potential_register.unwrap_potential_register(&engine_state.registers)?;
    engine_state.registers.set_register(register, value)?;
    Ok(())
}

fn push(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let potential_register = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let value = potential_register.unwrap_potential_register(&engine_state.registers)?;
    engine_state.stack.push(value);
    Ok(())
}

fn pop(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let value = match engine_state.stack.pop() {
        Some(x) => x,
        None => { return Err(SVMError::StackEmpty); }
    };

    set_register_or_memory(engine_state, destination, value)?;
    Ok(())
}

fn eq(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    let potential_left_register = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let left = potential_left_register.unwrap_potential_register(&engine_state.registers)?;

    let potential_right_register = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let right = potential_right_register.unwrap_potential_register(&engine_state.registers)?;

    let mut value = 0;
    if left == right {
        value = 1;
    }

    set_register_or_memory(engine_state, destination, value)?;
    Ok(())
}

fn gt(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    let potential_left_register = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let left = potential_left_register.unwrap_potential_register(&engine_state.registers)?;

    let potential_right_register = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let right = potential_right_register.unwrap_potential_register(&engine_state.registers)?;

    let mut value = 0;
    if left > right {
        value = 1;
    }

    set_register_or_memory(engine_state, destination, value)?;

    Ok(())
}

fn jmp(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let address = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    // println!("Jumping to {}", address);
    engine_state.instruction_pointer.set_ip(address)?;
    Ok(())
}

fn jt(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let value = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    let address = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    if value != 0 {
        engine_state.instruction_pointer.set_ip(address)?;
    }

    Ok(())
}

fn jf(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let value = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    let address = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    if value == 0 {
        engine_state.instruction_pointer.set_ip(address)?;
    }

    Ok(())
}

fn add(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    let left = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;

    let right = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;

    let result = (left + right) % (std::i16::MAX as u16 + 1);

    // print!("add: destination={}, left={}, right={}, result={}\n", destination, left, right, result);

    engine_state.registers.set_register(destination, result)?;

    Ok(())
}

fn mult(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    let left = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;

    let right = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    
    // A little messy here but we don't want to overflow
    let result = (left as u32 * right as u32) % (std::i16::MAX as u32 + 1);
    engine_state.registers.set_register(destination, result as u16)?;

    Ok(())
}

fn modulus(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    let left = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;

    let right = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    
    let result = left % right;

    engine_state.registers.set_register(destination, result)?;

    Ok(())
}

fn and(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    let left = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;

    let right = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    
    let result = left & right;

    engine_state.registers.set_register(destination, result)?;

    Ok(())
}

fn or(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    let left = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;

    let right = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    
    let result = left | right;

    engine_state.registers.set_register(destination, result)?;

    Ok(())
}

fn not(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;

    let value = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;

    let result = !value & 0x7FFF;

    engine_state.registers.set_register(destination, result)?;

    Ok(())
}

fn rmem(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination_reg = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let source_address = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    let value = engine_state.memory.load_memory(source_address)?;
    engine_state.registers.set_register(destination_reg, value)?;
    Ok(())
}

fn wmem(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination_address = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    let value = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    engine_state.memory.store_memory(destination_address, value)?;
    Ok(())
}

fn call(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let jump_address =  engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    engine_state.stack.push(engine_state.instruction_pointer.get_ip());
    engine_state.instruction_pointer.set_ip(jump_address)?;
    Ok(())
}

fn ret(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let return_address = match engine_state.stack.pop() {
        Some(x) => Ok(x),
        None => Err(SVMError::StackEmpty),
    }?;

    engine_state.instruction_pointer.set_ip(return_address)?;

    Ok(())
}

fn output(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let out_char = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?
        .unwrap_potential_register(&engine_state.registers)?;
    let out_array : [u8; 1] = [out_char as u8; 1];
    match std::io::stdout().write(&out_array) {
        Ok(_) => { return Ok(()) },
        Err(_) => { return Err(SVMError::WriteError); }
    };

}

fn input(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    let destination = engine_state.instruction_pointer.get_next_memory_value(&engine_state.memory)?;
    let mut input_buffer : [u8; 1] = [0; 1];
    match std::io::stdin().read_exact(&mut input_buffer) {
        Ok(_) => {},
        Err(_) => { return Err(SVMError::ReadError); }
    };
    if input_buffer[0] == 13   
    {
        // We need to consume carriage returns if we're on Windows
        match std::io::stdin().read_exact(&mut input_buffer) {
            Ok(_) => {},
            Err(_) => { return Err(SVMError::ReadError); }
        };
    }
    engine_state.registers.set_register(destination, input_buffer[0] as u16)?;
    Ok(())
}

fn noop(_engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn set_register_or_memory(engine_state: &mut SVMEngineState, destination: u16, value: u16) -> Result<(), SVMError> {
    if destination.is_valid_register() {
        engine_state.registers.set_register(destination, value)?;
    } else {
        engine_state.memory.store_memory(destination, value)?;
    }
    Ok(())
}