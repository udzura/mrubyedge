use std::{collections::HashMap, ffi::c_void, rc::Rc};

use super::vm::IREP;

#[derive(Debug, Clone, Copy)]
pub enum RType {
    Bool,
    Symbol,
    Integer,
    Float,
    Class,
    Instance,
    Proc,
    Array,
    Hash,
    String,
    Range,
    Data,
    Nil,
}

#[derive(Debug, Clone)]
pub enum RValue {
    Bool(bool),
    Symbol(RSym),
    Integer(i64),
    Float(f64),
    Class(Rc<RClass>),
    Instance(RInstance),
    Proc(RProc),
    Array(Vec<RObject>),
    Hash(HashMap<String, RObject>),
    RString(String),
    Range(Box<RObject>, Box<RObject>),
    Data,
    Nil,
}

#[derive(Debug, Clone)]
pub struct RObject {
    pub tt: RType,
    pub value: RValue
}

impl RObject {
    pub fn nil() -> Self {
        RObject {
            tt: RType::Nil,
            value: RValue::Nil,
        }
    }

    pub fn integer(n: i64) -> Self {
        RObject {
            tt: RType::Integer,
            value: RValue::Integer(n),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RClass {
    pub sym_id: RSym,
    pub super_class: Option<Box<RClass>>,
    pub procs: HashMap<String, RProc>,
}

#[derive(Debug, Clone)]
pub struct RInstance {
    pub class: Box<RClass>,
    pub ivar: HashMap<String, Box<RObject>>,
    pub data: Vec<u8>,
    pub ref_count: usize,
}

#[derive(Debug, Clone)]
pub struct RProc {
    pub is_rb_func: bool,
    pub sym_id: RSym,
    pub next: Box<RProc>,
    pub irep: Box<IREP>,
    pub func: Box<*const c_void>, // TODO: can we cast this into fn pointer?
}

#[derive(Debug, Clone)]
pub struct RSym {
    pub name: String
}

impl From<&'static str> for RSym {
    fn from(value: &'static str) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}
