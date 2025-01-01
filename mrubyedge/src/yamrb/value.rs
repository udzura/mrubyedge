use std::{cell::RefCell, collections::HashMap, ffi::c_void, rc::Rc};

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
    String(String),
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

    pub fn boolean(b: bool) -> Self {
        RObject {
            tt: RType::Bool,
            value: RValue::Bool(b),
        }
    }

    pub fn symbol(sym: RSym) -> Self {
        RObject {
            tt: RType::Symbol,
            value: RValue::Symbol(sym),
        }
    }

    pub fn integer(n: i64) -> Self {
        RObject {
            tt: RType::Integer,
            value: RValue::Integer(n),
        }
    }

    pub fn float(f: f64) -> Self {
        RObject {
            tt: RType::Float,
            value: RValue::Float(f),
        }
    }

    pub fn string(s: String) -> Self {
        RObject {
            tt: RType::String,
            value: RValue::String(s),
        }
    }

    pub fn is_falsy(&self) -> bool {
        match self.tt {
            RType::Nil => true,
            RType::Bool => match self.value {
                RValue::Bool(b) => !b,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        !self.is_falsy()
    }

    pub fn is_nil(&self) -> bool {
        match self.tt {
            RType::Nil => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RClass {
    pub sym_id: RSym,
    pub super_class: Option<Box<RClass>>,
    pub procs: RefCell<HashMap<String, RProc>>,
    pub consts: RefCell<HashMap<String, Rc<RObject>>>,
}

impl RClass {
    pub fn getmcnst(&self, name: &str) -> Option<Rc<RObject>> {
        let consts = self.consts.borrow();
        consts.get(name).map(|v| v.clone())
    }
}

#[derive(Debug, Clone)]
pub struct RInstance {
    pub class: Rc<RClass>,
    pub ivar: RefCell<HashMap<String, Rc<RObject>>>,
    pub data: Vec<u8>,
    pub ref_count: usize,
}

#[derive(Debug, Clone)]
pub struct RProc {
    pub is_rb_func: bool,
    pub sym_id: RefCell<RSym>,
    pub next: Option<Box<RProc>>,
    pub irep: Option<Rc<IREP>>,
    pub func: Option<Box<*const c_void>>, // TODO: can we cast this into fn pointer?
}

#[derive(Debug, Clone)]
pub struct RSym {
    pub name: String
}

impl RSym {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

impl From<&'static str> for RSym {
    fn from(value: &'static str) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}
