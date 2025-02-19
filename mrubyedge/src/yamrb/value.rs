use std::cell::Cell;
use std::collections::HashSet;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::Error;

use super::vm::{ENV, IREP, VM};
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
    Exception,
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
    Array(RefCell<Vec<Rc<RObject>>>),
    Hash(RefCell<HashMap<ValueHasher, (Rc<RObject>, Rc<RObject>)>>),
    String(RefCell<Vec<u8>>),
    Range(Rc<RObject>, Rc<RObject>, bool),
    SharedMemory(Rc<RefCell<SharedMemory>>),
    Data,
    Exception(Rc<RException>),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueHasher {
    Bool(bool),
    Integer(i64),
    Float(Vec<u8>),
    Symbol(String),
    String(Vec<u8>),
    Class(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueEquality {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Symbol(String),
    String(Vec<u8>),
    Class(String),
    Range(Box<ValueEquality>, Box<ValueEquality>, bool),
    Array(Vec<ValueEquality>),
    KeyValue(ValueEqualityForKeyValue),
    ObjectID(u64),
    Nil,
}

#[derive(Debug, Clone)]
pub struct ValueEqualityForKeyValue(HashSet<ValueHasher>, HashMap<ValueHasher, ValueEquality>);

impl PartialEq for ValueEqualityForKeyValue {
    fn eq(&self, other: &Self) -> bool {
        if self.0 != other.0 {
            return false;
        }
        for key in self.0.iter() {
            if self.1.get(key) != other.1.get(key) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug, Clone)]
pub struct RObject {
    pub tt: RType,
    pub value: RValue,
    pub object_id: Cell<u64>,
}

impl RObject {
    pub fn nil() -> Self {
        RObject {
            tt: RType::Nil,
            value: RValue::Nil,
            object_id: 4.into(),
        }
    }

    pub fn boolean(b: bool) -> Self {
        RObject {
            tt: RType::Bool,
            value: RValue::Bool(b),
            object_id: (if b { 20 } else { 0 }).into(),
        }
    }

    pub fn symbol(sym: RSym) -> Self {
        RObject {
            tt: RType::Symbol,
            value: RValue::Symbol(sym),
            object_id: 2.into(), // TODO: calc the same id for the same symbol
        }
    }

    pub fn integer(n: i64) -> Self {
        let object_id = if n >= (i32::MAX as i64) {
            u64::MAX
        } else if n <= (i32::MIN as i64) {
            u64::MAX
        } else {
            n as u64 * 2 + 1
        };

        RObject {
            tt: RType::Integer,
            value: RValue::Integer(n),
            object_id: object_id.into(),
        }
    }

    pub fn float(f: f64) -> Self {
        RObject {
            tt: RType::Float,
            value: RValue::Float(f),
            object_id: (f.to_bits() as u64).into(),
        }
    }

    pub fn string(s: String) -> Self {
        RObject {
            tt: RType::String,
            value: RValue::String(RefCell::new(s.into_bytes())),
            object_id: (u64::MAX).into(),
        }
    }

    pub fn string_from_vec(v: Vec<u8>) -> Self {
        RObject {
            tt: RType::String,
            value: RValue::String(RefCell::new(v)),
            object_id: (u64::MAX).into(),
        }
    }

    pub fn array(v: Vec<Rc<RObject>>) -> Self {
        RObject {
            tt: RType::Array,
            value: RValue::Array(RefCell::new(v)),
            object_id: (u64::MAX).into(),
        }
    }

    pub fn hash(h: HashMap<ValueHasher, (Rc<RObject>, Rc<RObject>)>) -> Self {
        RObject {
            tt: RType::Hash,
            value: RValue::Hash(RefCell::new(h)),
            object_id: (u64::MAX).into(),
        }
    }

    pub fn range(start: Rc<RObject>, end: Rc<RObject>, exclusive: bool) -> Self {
        RObject {
            tt: RType::Range,
            value: RValue::Range(start, end, exclusive),
            object_id: (u64::MAX).into(),
        }
    }

    pub fn class(c: Rc<RClass>) -> Self {
        RObject {
            tt: RType::Class,
            value: RValue::Class(c),
            object_id: (u64::MAX).into(),
        }
    }

    pub fn instance(c: Rc<RClass>) -> Self {
        RObject {
            tt: RType::Instance,
            value: RValue::Instance(RInstance {
                class: c,
                ivar: RefCell::new(HashMap::new()),
                data: Vec::new(),
                ref_count: 1,
            }),
            object_id: (u64::MAX).into(),
        }
    }

    pub fn exception(e: Rc<RException>) -> Self {
        RObject {
            tt: RType::Exception,
            value: RValue::Exception(e),
            object_id: (u64::MAX).into(),
        }
    }

    pub fn to_refcount_assigned(self) -> Rc<Self> {
        let rc = Rc::new(self);
        let id = Rc::as_ptr(&rc) as u64;
        if rc.object_id.get() == u64::MAX {
            rc.object_id.set(id);
        }
        rc
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
    pub fn as_hash_key(&self) -> Result<ValueHasher, Error> {
        match &self.value {
            RValue::Bool(b) => Ok(ValueHasher::Bool(*b)),
            RValue::Integer(i) => Ok(ValueHasher::Integer(*i)),
            RValue::Float(f) => Ok(ValueHasher::Float(f.to_be_bytes().to_vec())),
            RValue::Symbol(s) => Ok(ValueHasher::Symbol(s.name.clone())),
            RValue::String(s) => Ok(ValueHasher::String(s.borrow().clone())),
            RValue::Class(c) => Ok(ValueHasher::Class(c.sym_id.name.clone())),
            _ => {
                Err(Error::TypeMismatch)
            }
        }
    }

    pub fn as_eq_value(&self) -> ValueEquality {
        match &self.value {
            RValue::Bool(b) => ValueEquality::Bool(*b),
            RValue::Integer(i) => ValueEquality::Integer(*i),
            RValue::Float(f) => ValueEquality::Float(*f),
            RValue::Symbol(s) => ValueEquality::Symbol(s.name.clone()),
            RValue::String(s) => ValueEquality::String(s.borrow().clone()),
            RValue::Class(c) => ValueEquality::Class(c.sym_id.name.clone()),
            RValue::Range(s, e, ex) => {
                ValueEquality::Range(
                    Box::new(s.as_eq_value()),
                    Box::new(e.as_eq_value()),
                    *ex
                )
            },
            RValue::Array(a) => {
                let arr = a.borrow().iter().map(|v| v.as_eq_value()).collect();
                ValueEquality::Array(arr)
            },
            RValue::Hash(ha) => {
                let keys: HashSet<_> = ha.borrow().keys().map(|k| k.clone()).collect();
                ValueEquality::KeyValue(ValueEqualityForKeyValue(
                    keys,
                    ha.borrow().iter().map(|(k, (_, v))| (k.clone(), v.as_ref().as_eq_value())).collect(),
                ))
            },
            RValue::Nil => ValueEquality::Nil,
            _ => {
                ValueEquality::ObjectID(self.object_id.get())
            }
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
            RValue::Exception(e) => e.class.clone(),
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

impl TryFrom<&RObject> for u32 {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match value.value {
            RValue::Integer(i) => Ok(i as u32),
            RValue::Bool(b) => {
                if b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            RValue::Float(f) => return Ok(f as u32),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl TryFrom<&RObject> for i64 {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match value.value {
            RValue::Integer(i) => Ok(i),
            RValue::Bool(b) => {
                if b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            RValue::Float(f) => return Ok(f as i64),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl TryFrom<&RObject> for u64 {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match value.value {
            RValue::Integer(i) => Ok(i as u64),
            RValue::Bool(b) => {
                if b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            RValue::Float(f) => return Ok(f as u64),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl TryFrom<&RObject> for usize {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match value.value {
            RValue::Integer(i) => Ok(i as usize),
            RValue::Bool(b) => {
                if b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            RValue::Float(f) => return Ok(f as usize),
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

impl TryFrom<&RObject> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match &value.value {
            RValue::String(s) => Ok(s.borrow().clone()),
            _ => Err(Error::TypeMismatch),
        }
    }
}

impl TryFrom<&RObject> for () {
    type Error = Error;

    fn try_from(_: &RObject) -> Result<Self, Self::Error> {
        Ok(())
    }
}

impl TryFrom<&RObject> for *mut u8 {
    type Error = Error;

    fn try_from(value: &RObject) -> Result<Self, Self::Error> {
        match &value.value {
            RValue::SharedMemory(sm) => Ok(sm.borrow_mut().as_mut_ptr()),
            _ => Err(Error::TypeMismatch),
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
        RObject::class(value)
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
    pub func: Option<usize>,
    pub environ: Option<Rc<ENV>>,
    pub block_self: Option<Rc<RObject>>,
}

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

#[derive(Debug)]
pub struct RException {
    pub class: Rc<RClass>,
    pub error_type: RefCell<Error>,
    pub message: String,
    pub backtrace: Vec<String>, // TODO
}

impl RClass {
    pub fn from_error(vm: &mut VM, e: &Error) -> Rc<Self> {
        match e {
            Error::General => {
                return vm.get_class_by_name("Exception");
            }
            Error::InvalidOpCode => {
                return vm.get_class_by_name("LoadError");
            }
            Error::RuntimeError(_) => {
                return vm.get_class_by_name("RuntimeError");
            }
            Error::TypeMismatch => {
                return vm.get_class_by_name("LoadError");
            }
            Error::NoMethodError(_) => {
                return vm.get_class_by_name("NoMethodError");
            }
        }
    }
}

impl RException {
    pub fn from_error(vm: &mut VM, e: &Error) -> Self {
        RException {
            class: RClass::from_error(vm, e),
            error_type: RefCell::new(e.clone()),
            message: e.message(),
            backtrace: Vec::new(),
        }
    }
}