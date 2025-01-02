use std::cell::{Cell, RefCell};
use std::error::Error;
use std::rc::Rc;
use std::collections::HashMap;

use crate::rite::{insn, Irep, Rite};

use super::optable::*;
use super::prelude::object::initialize_object;
use super::value::*;
use super::op::Op;

const MAX_REGS_SIZE: usize = 256;

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

    pub fn_table: Vec<Rc<RFn>>,
}

fn interpret_insn(mut insns: &[u8]) -> Vec<Op> {
    let mut pos: usize = 0;
    let mut ops = Vec::new();
    while insns.len() > 0 {
        let op = insns[0];
        let opcode: insn::OpCode = op.try_into().unwrap();
        let fetched = insn::FETCH_TABLE[op as usize](&mut insns).unwrap();
        ops.push(Op::new(opcode, fetched, pos, 1 + fetched.len()));
        pos += 1 + fetched.len();
    }
    ops
}

fn irep_to_irep1(irep: &mut Irep) -> IREP {
    let mut irep1 = IREP {
        nlocals: irep.nlocals(),  
        nregs: irep.nregs(),
        rlen: irep.rlen(),
        iren: 0,
        plen: irep.plen,
        code: Vec::new(),
        syms: Vec::new(),
        pool: Vec::new(),
        reps: Vec::new(),
    };
    for sym in irep.syms.iter() {
        irep1.syms.push(RSym::new(sym.to_string_lossy().to_string()));
    }
    for str in irep.strvals.iter() {
        irep1.pool.push(RPool::Str(str.to_string_lossy().to_string()));
    }

    let code = interpret_insn(&mut irep.insn);
    irep1.code = code;
    irep1
}

// This will consume the Rite and return the IREP
fn rite_to_irep(rite: &mut Rite) -> IREP {
    let mut irep0 = irep_to_irep1(&mut rite.irep[0]);
    for rep in rite.irep[1..].iter_mut() {
        irep0.reps.push(Rc::new(irep_to_irep1(rep)));
    }
    irep0
}

impl VM {
    pub fn open(rite: &mut Rite) -> VM {
        let irep = rite_to_irep(rite);
        let vm = VM::new_by_raw_irep(irep);
        vm
    }

    pub fn new_by_raw_irep(irep: IREP) -> VM {
        let irep = Rc::new(irep);
        let globals = HashMap::new();
        let consts = HashMap::new();
        let builtin_class_table = HashMap::new();

        let object_class = Rc::new(
            RClass {
                sym_id: "Object".into(),
                super_class: None,
                procs: RefCell::new(HashMap::new()),
                consts: RefCell::new(HashMap::new()),
            }
        );

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
        let fn_table = Vec::new();

        let mut vm = VM {
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
            fn_table
        };

        initialize_object(&mut vm);
        
        vm
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

    pub(crate) fn register_fn(&mut self, f: RFn) -> usize {
        self.fn_table.push(Rc::new(f));
        return self.fn_table.len() - 1;
    }
    
    pub(crate) fn get_fn(&self, i: usize) -> Option<Rc<RFn>> {
        self.fn_table.get(i).cloned()
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