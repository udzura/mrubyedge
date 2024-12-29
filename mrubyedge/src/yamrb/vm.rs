use std::cell::Cell;

use super::value::{RClass, RObject, RSym};
use super::op::Op;

const MAX_REGS_SIZE: usize = 256;

#[derive(Debug, Clone)]
pub struct VM<'a> {
    pub irep: Box<IREP>,
    
    pub id: usize,
    pub bytecode: &'a [u8],
    pub current_irep: Box<IREP>,
    pub pc: usize,
    pub regs: [Box<RObject>; MAX_REGS_SIZE],
    pub current_reg_offset: usize,
    pub current_callinfo: Box<CALLINFO<'a>>,
    pub target_class: &'a RClass,
    pub error_code: u32,

    pub flag_preemption: Cell<bool>,

    // common class
    pub object_class: Box<RClass>,
}

#[derive(Debug, Clone)]
pub struct IREP {
    pub nlocals: usize,
    pub nregs: usize, // NOTE: is u8 better?
    pub rlen: usize,
    pub iren: usize,
    pub plen: usize,
    pub code: Vec<Op>,
    pub syms: Vec<RSym>,
    pub pool: Vec<RObject>,
    pub reps: Vec<IREP>
}

#[derive(Debug, Clone)]
pub struct CALLINFO<'a> {
    pub prev: Box<CALLINFO<'a>>,
    pub method_id: RSym,
    pub pc_irep: &'a IREP,
    pub ps: usize,
    pub current_regs_offset: usize,
    pub n_args: usize,
}