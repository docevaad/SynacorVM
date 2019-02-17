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
    fn dispatch(mut engine_state: &SVMEngineState);
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
