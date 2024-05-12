use crate::rite::{self, Error, Irep};
use crate::vm::VM;

use std::collections::HashMap;
use std::ffi::CString;
use std::rc::Rc;

// https://github.com/mrubyc/mrubyc/blob/5fab2b85dce8fc0780293235df6c0daa5fd57dce/src/vm.h#L41-L62
#[derive(Debug)]
pub struct VMIrep<'insn> {
    pub nlocals: usize,
    pub nregs: usize,
    pub rlen: usize,
    pub clen: usize,
    pub ilen: usize,
    pub plen: usize,
    pub slen: usize,

    pub pool: Vec<RPool>,
    pub syms: Vec<RSym>,

    pub inst_head: &'insn [u8],

    pub return_val: Option<RObject>,
}

impl<'insn> VMIrep<'insn> {
    pub fn from_raw_record(irep: Irep<'insn>) -> VMIrep<'insn> {
        let nlocals = rite::be16_to_u16(irep.header.nlocals) as usize;
        let nregs = rite::be16_to_u16(irep.header.nregs) as usize;
        let rlen = rite::be16_to_u16(irep.header.rlen) as usize;
        let clen = rite::be16_to_u16(irep.header.clen) as usize;
        let ilen = rite::be32_to_u32(irep.header.ilen) as usize;

        let plen = irep.plen;
        let pool = irep
            .strvals
            .iter()
            .map(|v| RPool::StaticStr(v.to_owned()))
            .collect();

        let slen = irep.slen;
        let syms = irep
            .syms
            .iter()
            .map(|v| RSym {
                value: v.to_owned(),
            })
            .collect();

        let inst_head = irep.insn;

        let return_val = None;

        VMIrep {
            nlocals,
            nregs,
            rlen,
            clen,
            ilen,
            plen,
            slen,
            pool,
            syms,
            inst_head,
            return_val,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum RPool {
    StaticStr(CString),
    Dummy,
}

#[derive(Debug)]
pub struct RSym {
    pub value: CString,
}

#[non_exhaustive]
#[derive(Debug)]
pub enum RObject {
    Class { class_index: usize },
    RInstance { class_index: usize },
    RString(String),
    RProc { irep_index: usize },
    RInteger(i64),
    RFloat(f64),
    RBool(bool),
    Nil,
    // ...
}

impl From<&RPool> for RObject {
    fn from(value: &RPool) -> Self {
        match value {
            RPool::StaticStr(s) => {
                let s = <CString as Clone>::clone(&s).into_string().unwrap();
                RObject::RString(s)
            }
            _ => {
                unimplemented!("From<&RPool>")
            }
        }
    }
}

impl TryFrom<&RObject> for i32 {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match value {
            RObject::RInteger(i) => Ok(*i as i32),
            RObject::RBool(b) => {
                if *b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            RObject::RFloat(f) => return Ok(*f as i32),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl TryFrom<&RObject> for f32 {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match value {
            RObject::RInteger(i) => Ok(*i as f32),
            RObject::RBool(b) => {
                if *b {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            RObject::RFloat(f) => return Ok(*f as f32),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl TryFrom<&RObject> for String {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match value {
            RObject::RString(s) => Ok(s.to_owned()),
            v => Ok(format!("{:?}", v)),
        }
    }
}

impl TryFrom<&RObject> for () {
    type Error = Error;

    fn try_from(_: &RObject) -> Result<Self, Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct RClass<'insn> {
    pub sym_id: u32,
    // pub num_builtin_method: usize,
    pub super_klass: Rc<Option<Box<RClass<'insn>>>>,
    pub static_methods: HashMap<String, RMethod<'insn>>,
    pub methods: HashMap<String, RMethod<'insn>>,
}

#[derive(Debug)]
pub struct RMethod<'insn> {
    pub sym_id: u32,
    pub body: Method<'insn>,
}

type RFn = for<'insn> fn(&mut VM<'insn>, &[Rc<RObject>]) -> Rc<RObject>;

#[derive(Debug)]
pub enum Method<'insn> {
    RubyMethod(Rc<VMIrep<'insn>>),
    CMethod(RFn),
}

#[derive(Debug)]
pub struct CallInfo<'insn> {
    // https://github.com/mrubyc/mrubyc/blob/5fab2b85dce8fc0780293235df6c0daa5fd57dce/src/vm.h#L111-L126
    pub cur_pc: usize,
    pub cur_regs: HashMap<usize, Rc<RObject>>,
    pub cur_irep: Rc<VMIrep<'insn>>,
    pub target_class: Option<usize>,
}
