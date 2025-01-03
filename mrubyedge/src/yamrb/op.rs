use crate::rite::insn::{Fetched, OpCode};

#[derive(Debug, Clone, Copy)]
pub struct Op {
    pub code: OpCode,
    pub operand: Fetched,
    
    pub pos: usize,
    pub len: usize,
}

impl Op {
    pub fn new(code: OpCode, operand: Fetched, pos: usize, len: usize) -> Self {
        Self {
            code,
            operand,
            pos,
            len,
        }
    }
}