use std::{fmt};
use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth{
    stack : Vec<Value>,
    definitions : HashMap<String, Vec<String>>,
}

pub enum ForthWord {
    Number(Value),
    AddOp,
    SubOp,
    MultOp,
    DivOp,
    DupOp,
    DropOp,
    SwapOp,
    OverOp,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DivisionByZero => write!(f, "Division by zero error"),
            Error::StackUnderflow => write!(f, "Stack underflow error"),
            Error::UnknownWord => write!(f, "Unknown word error"),
            Error::InvalidWord => write!(f, "Invalid word error"),
        }
    }
}

impl Forth {
    pub fn new() -> Forth {
        Self{}
    }

    pub fn stack(&self) -> &[Value] {
        unimplemented!()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        if input.len() < 5 {
            return Err(Error::StackUnderflow)
        }
        let vec = input
            .replace(" ", "")
            .chars()
            .map(|ch| ch.to_string())
            .collect::<Vec<_>>();

        for win in vec.chunks(3).next() {

        }

        Ok(())
    }
}
