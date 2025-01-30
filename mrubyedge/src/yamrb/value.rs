use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::Error;

use super::vm::{IREP, VM};
use super::shared_memory::SharedMemory;

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
    SharedMemory,
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
    String(RefCell<Vec<u8>>),
    Range(Rc<RObject>, Rc<RObject>, bool),
    SharedMemory(Rc<RefCell<SharedMemory>>),
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
            value: RValue::String(RefCell::new(s.into_bytes())),
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
            RValue::String(s) => format!("__String__{}", String::from_utf8_lossy(&s.borrow())),
            RValue::Integer(i) => format!("__Integer__{}", *i),
            RValue::Symbol(s) => format!("__Symbol__{}", s.name),
            RValue::Bool(b) => format!("__Bool__{:?}", *b),
            _ => unimplemented!("Key should be one of String, Integer, Symbol, or Boolean for now"),
        }
    }

    pub fn get_class(&self, vm: &VM) -> Rc<RClass> {
        match &self.value {
            RValue::Class(_) => vm.get_class_by_name("Class"),
            RValue::Instance(i) => i.class.clone(),
            RValue::Bool(b) => {
                if *b {
                    vm.get_class_by_name("TrueClass")
                } else {
                    vm.get_class_by_name("FalseClass")
                }
            },
            RValue::Symbol(_) => vm.get_class_by_name("Symbol"),
            RValue::Integer(_) => vm.get_class_by_name("Integer"),
            RValue::Float(_) => vm.get_class_by_name("Float"),
            RValue::Proc(_) => vm.get_class_by_name("Proc"),
            RValue::Array(_) => vm.get_class_by_name("Array"),
            RValue::Hash(_) => vm.get_class_by_name("Hash"),
            RValue::String(_) => vm.get_class_by_name("String"),
            RValue::Range(_, _, _) => vm.get_class_by_name("Range"),
            RValue::SharedMemory(_) => vm.get_class_by_name("SharedMemory"),
            RValue::Data => todo!("return ...? class"),
            RValue::Nil => vm.get_class_by_name("NilClass"),
        }
    }
}

impl TryFrom<&RObject> for i32 {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match value.value {
            RValue::Integer(i) => Ok(i as i32),
            RValue::Bool(b) => {
                if b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            RValue::Float(f) => return Ok(f as i32),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl TryFrom<&RObject> for f32 {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match value.value {
            RValue::Integer(i) => Ok(i as f32),
            RValue::Bool(b) => {
                if b {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            RValue::Float(f) => return Ok(f as f32),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl TryFrom<&RObject> for bool {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match &value.value {
            RValue::Bool(b) => Ok(*b),
            RValue::Integer(i) => Ok(*i != 0),
            RValue::Nil => Ok(false),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl TryFrom<&RObject> for String {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match &value.value {
            RValue::String(s) => Ok(String::from_utf8_lossy(&s.borrow()).to_string()),
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

    // find_method will search method from self to superclass
    pub fn find_method(&self, name: &str) -> Option<RProc> {
        let procs = self.procs.borrow();
        match procs.get(name) {
            Some(p) => Some(p.clone()),
            None => {
                match &self.super_class {
                    Some(sc) => sc.find_method(name),
                    None => None,
                }
            }
        }
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
    pub next: Option<Rc<RProc>>,
    pub irep: Option<Rc<IREP>>,
    pub func: Option<usize>,}

pub type RFn = Box<dyn Fn(&mut VM, &[Rc<RObject>]) -> Result<Rc<RObject>, Error>>;

#[derive(Debug, Clone, PartialEq, Eq)]
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