use std::{cell::RefCell, collections::HashMap, ffi::c_void, fmt::Debug, rc::Rc};

use crate::Error;

use super::vm::{IREP, VM};

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
    Array(Vec<Rc<RObject>>),
    Hash(HashMap<String, Rc<RObject>>),
    String(RefCell<String>),
    Range(Rc<RObject>, Rc<RObject>, bool),
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
            value: RValue::String(RefCell::new(s)),
        }
    }

    pub fn array(v: Vec<Rc<RObject>>) -> Self {
        RObject {
            tt: RType::Array,
            value: RValue::Array(v),
        }
    }

    pub fn hash(h: HashMap<String, Rc<RObject>>) -> Self {
        RObject {
            tt: RType::Hash,
            value: RValue::Hash(h),
        }
    }

    pub fn class(c: Rc<RClass>) -> Self {
        RObject {
            tt: RType::Class,
            value: RValue::Class(c),
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

    // TODO: implment Object#hash
    pub fn as_hash_key(&self) -> String {
        match &self.value {
            RValue::String(s) => format!("__String__{}", s.borrow().clone()),
            RValue::Integer(i) => format!("__Integer__{}", *i),
            RValue::Symbol(s) => format!("__Symbol__{}", s.name),
            RValue::Bool(b) => format!("__Bool__{:?}", *b),
            _ => unimplemented!("Key should be one of String, Integer, Symbol, or Boolean for now"),
        }
    }

    pub fn get_class(&self) -> Rc<RClass> {
        match &self.value {
            RValue::Class(_) => todo!("return Class class"),
            RValue::Instance(i) => i.class.clone(),
            RValue::Bool(_b) => todo!("return True or False class"),
            RValue::Symbol(_) => todo!("return Symbol class"),
            RValue::Integer(_) => todo!("return Integer class"),
            RValue::Float(_) => todo!("return Float class"),
            RValue::Proc(_) => todo!("return Proc class"),
            RValue::Array(_) => todo!("return Array class"),
            RValue::Hash(_) => todo!("return Hash class"),
            RValue::String(_) => todo!("return String class"),
            RValue::Range(_, _, _) => todo!("return Range class"),
            RValue::Data => todo!("return ...? class"),
            RValue::Nil => todo!("return NilClass"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RClass {
    pub sym_id: RSym,
    pub super_class: Option<Rc<RClass>>,
    pub procs: RefCell<HashMap<String, RProc>>,
    pub consts: RefCell<HashMap<String, Rc<RObject>>>,
}

impl RClass {
    pub fn new(name: &str, super_class: Option<Rc<RClass>>) ->Self {
        let name = name.to_string();
        RClass {
            sym_id: RSym::new(name),
            super_class,
            procs: RefCell::new(HashMap::new()),
            consts: RefCell::new(HashMap::new()),
        }
    }

    pub fn getmcnst(&self, name: &str) -> Option<Rc<RObject>> {
        let consts   = self.consts.borrow();
        consts.get(name).map(|v| v.clone())
    }
}

impl From<Rc<RClass>> for RObject {
    fn from(value: Rc<RClass>) -> Self {
        RObject {
            tt: RType::Class,
            value: RValue::Class(value),
        }
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
    pub sym_id: Option<RSym>,
    pub next: Option<Box<RProc>>,
    pub irep: Option<Rc<IREP>>,
    pub func: Option<usize>,}

pub type RFn = Box<dyn Fn(&mut VM, &[Rc<RObject>]) -> Result<usize, Error>>;

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

#[derive(Debug, Clone)]
pub enum RPool {
    Str(String),
    Data(Vec<u8>),
}

impl RPool {
    pub fn as_str(&self) -> &str {
        match self {
            RPool::Str(s) => s,
            _ => unreachable!("RPool is not a string...?"),
        }
    }
}