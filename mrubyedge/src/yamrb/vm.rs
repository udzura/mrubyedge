use std::cell::{Cell, RefCell};
use std::error::Error;
use std::rc::Rc;
use std::collections::HashMap;

use super::optable::*;
use super::value::*;
use super::op::Op;

const MAX_REGS_SIZE: usize = 256;

#[derive(Debug, Clone)]
pub struct VM {
    pub irep: Rc<IREP>,
    
    pub id: usize,
    pub bytecode: Vec<u8>,
    pub current_irep: Rc<IREP>,
    pub pc: Cell<usize>,
    pub regs: [Option<Rc<RObject>>; MAX_REGS_SIZE],
    pub current_regs_offset: usize,
    pub current_callinfo: Option<Rc<CALLINFO>>,
    pub target_class: Rc<RClass>,
    pub error_code: u32,

    pub flag_preemption: Cell<bool>,

    // common class
    pub object_class: Rc<RClass>,
    pub builtin_class_table: HashMap<&'static str, Rc<RClass>>,

    pub globals: HashMap<String, Rc<RObject>>,
    pub consts: HashMap<String, Rc<RObject>>,
}

impl VM {
    pub fn new_by_irep(irep: IREP) -> VM {
        let irep = Rc::new(irep);
        let globals = HashMap::new();
        let mut consts = HashMap::new();
        let mut builtin_class_table = HashMap::new();

        let object_class = Rc::new(
            RClass {
                sym_id: "Object".into(),
                super_class: None,
                procs: RefCell::new(HashMap::new()),
                consts: RefCell::new(HashMap::new()),
            }
        );
        consts.insert("Object".to_string(), Rc::new(RObject {
            tt: RType::Class,
            value: RValue::Class(object_class.clone()),
        }));
        builtin_class_table.insert("Object", object_class.clone());

        let id = 1; // TODO generator
        let bytecode = Vec::new();
        let current_irep = irep.clone();
        let pc = Cell::new(0);
        let regs: [Option<Rc<RObject>>; MAX_REGS_SIZE] = [const { None }; MAX_REGS_SIZE];
        let current_regs_offset = 0;
        let current_callinfo = None;
        let target_class = object_class.clone();
        let error_code = 0;
        let flag_preemption = Cell::new(false);

        VM {
            id,
            bytecode,
            irep,
            current_irep,
            pc,
            regs,
            current_regs_offset,
            current_callinfo,
            target_class,
            error_code,
            flag_preemption,
            object_class,
            builtin_class_table,
            globals,
            consts,
        }
    }

    pub fn run(&mut self) -> Result<Rc<RObject>, Box<dyn Error>>{
        let class = self.object_class.clone();
        // Insert top_self
        self.current_regs()[0].replace(Rc::new(RObject{
            tt: RType::Instance,
            value: RValue::Instance(RInstance {
                class,
                ivar: RefCell::new(HashMap::new()),
                data: Vec::new(),
                ref_count: 1,
            })
        }));
        loop {
            let pc = self.pc.get();
            let op = self.current_irep.as_ref().code.get(pc).unwrap();
            let operand = op.operand;
            self.pc.set(pc + 1);

            consume_expr(self, op.code, &operand);

            if self.flag_preemption.get() {
                break;
            }
        }

        self.flag_preemption.set(false);

        match self.regs[0].take() {
            Some(v) => Ok(v),
            None => Ok(Rc::new(RObject::nil()))
        }
    }

    pub(crate) fn current_regs(&mut self) -> &mut [Option<Rc<RObject>>] {
        &mut self.regs[self.current_regs_offset..]
    }
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
    pub pool: Vec<RPool>,
    pub reps: Vec<Rc<IREP>>
}

#[derive(Debug, Clone)]
pub struct CALLINFO {
    pub prev: Option<Rc<CALLINFO>>,
    pub method_id: RSym,
    pub pc_irep: Rc<IREP>,
    pub pc: usize,
    pub current_regs_offset: usize,
    pub target_class: Rc<RClass>,
    pub n_args: usize,
}