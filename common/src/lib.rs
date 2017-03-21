use std::fmt;

#[derive(Clone, Copy)]
pub enum Instruction {
    NOP,
    Load(Data, Register),
}
use Instruction::*;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::NOP => write!(f, "{}", "NOP"),
            Instruction::Load(data, register) => write!(f, "load {}{}", data, register),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Data {
    Value,
}
use Data::*;

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
use Register::*;

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
