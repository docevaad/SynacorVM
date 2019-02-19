use super::extensions::RegisterValue;
use super::svm_engine_state::SVMEngineState;
use super::svm_error::SVMError;

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
fn halt(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
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
    Ok(())
}

fn jmp(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn jt(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn jf(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn add(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn mult(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn modulus(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn and(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn or(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn not(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn rmem(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn wmem(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn call(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn ret(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn output(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn input(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
    Ok(())
}

fn noop(engine_state: &mut SVMEngineState) -> Result<(), SVMError> {
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