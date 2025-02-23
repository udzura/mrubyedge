use std::cell::{Cell, RefCell};
use std::env;
use std::rc::Rc;
use std::collections::HashMap;

use crate::rite::{insn, Irep, Rite};
use crate::Error;

use super::{op, optable::*};
use super::prelude::prelude;
use super::value::*;
use super::op::Op;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const ENGINE: &'static str = "mruby/edge";

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
    pub exception: Option<Rc<RException>>,

    pub flag_preemption: Cell<bool>,

    // common class
    pub object_class: Rc<RClass>,
    pub builtin_class_table: HashMap<&'static str, Rc<RClass>>,

    pub globals: HashMap<String, Rc<RObject>>,
    pub consts: HashMap<String, Rc<RObject>>,

    pub upper: Option<Rc<ENV>>,
    // TODO: using fixed array?
    pub cur_env: HashMap<usize, Rc<ENV>>,
    pub has_env_ref: HashMap<usize, bool>,

    pub fn_table: Vec<Rc<RFn>>,
}

impl VM {
    pub fn open(rite: &mut Rite) -> VM {
        let irep = rite_to_irep(rite);
        let vm = VM::new_by_raw_irep(irep);
        vm
    }

    pub fn empty() -> VM {
        let irep = IREP {
            __id: 0,
            nlocals: 0,
            nregs: 0,
            rlen: 0,
            code: vec![
                op::Op { code: insn::OpCode::STOP, operand: insn::Fetched::Z, pos: 18, len: 1 },
            ],
            syms: Vec::new(),
            pool: Vec::new(),
            reps: Vec::new(),
            catch_target_pos: Vec::new(),
        };
        Self::new_by_raw_irep(irep)
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
        let exception = None;
        let flag_preemption = Cell::new(false);
        let fn_table = Vec::new();
        let upper = None;
        let cur_env = HashMap::new();
        let has_env_ref = HashMap::new();

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
            exception,
            flag_preemption,
            object_class,
            builtin_class_table,
            globals,
            consts,
            upper,
            cur_env,
            has_env_ref,
            fn_table
        };

        prelude(&mut vm);
        
        vm
    }

    pub fn run(&mut self) -> Result<Rc<RObject>, Box<dyn std::error::Error>> {
        let class = self.object_class.clone();
        // Insert top_self
        let top_self = RObject{
            tt: RType::Instance,
            value: RValue::Instance(RInstance {
                class,
                ivar: RefCell::new(HashMap::new()),
                data: Vec::new(),
                ref_count: 1,
            }),
            object_id: 0.into(),
        }.to_refcount_assigned();
        if self.current_regs()[0].is_none() {
            self.current_regs()[0].replace(top_self.clone());
        }
        let mut rescued = false;

        loop {
            if ! rescued {
                if let Some(_e) = self.exception.clone() {
                    let operand = insn::Fetched::B(0);
                    if let Some(pos) = self.find_next_handler_pos() {
                        self.pc.set(pos);
                        rescued = true;
                        continue;
                    }

                    match op_return(self, &operand) {
                        Ok(_) => {},
                        Err(_) => {
                            // use assigned expection through
                            break;
                        }
                    }
                    if self.flag_preemption.get() {
                        break;
                    } else {
                        continue;
                    }
                }
            }
            rescued = false;

            let pc = self.pc.get();
            if self.current_irep.code.len() <= pc {
                // reached end of the IREP
                break;
            }
            let op = self.current_irep.code.get(pc).ok_or_else(
                || Error::internal("end of opcode reached")
            )?;
            let operand = op.operand;
            self.pc.set(pc + 1);

            if env::var("MRUBYEDGE_DEBUG").is_ok() {
                eprintln!("{:?}: {:?} (pos={} len={})", &op.code, &op.operand, op.pos, op.len);
            }
            match consume_expr(self, op.code, &operand, op.pos, op.len) {
                Ok(_) => {},
                Err(e) => {
                    let exception = RException::from_error(self, &e);
                    self.exception = Some(Rc::new(exception));
                    continue;
                }
            }

            if self.flag_preemption.get() {
                break;
            }
        }

        self.flag_preemption.set(false);

        if let Some(e) = self.exception.clone() {
            return Err(e.error_type.borrow().clone().into());
        }

        let retval = match self.current_regs()[0].take() {
            Some(v) => Ok(v),
            None => Ok(Rc::new(RObject::nil()))
        };
        self.current_regs()[0].replace(top_self.clone());

        retval
    }

    pub(crate) fn find_next_handler_pos(&mut self) -> Option<usize> {
        let ci = self.pc.get();
        for p in self.current_irep.catch_target_pos.iter() {
            if ci < *p {
                return Some(*p);
            }
        }
        None
    }

    pub(crate) fn current_regs(&mut self) -> &mut [Option<Rc<RObject>>] {
        &mut self.regs[self.current_regs_offset..]
    }

    pub(crate) fn get_current_regs_cloned(&mut self, i: usize) -> Result<Rc<RObject>, Error> {
        self.current_regs()[i].clone().ok_or_else(|| Error::internal(format!("register {} is not assigned", i)))   
    }

    pub(crate) fn take_current_regs(&mut self, i: usize) -> Result<Rc<RObject>, Error> {
        self.current_regs()[i].take().ok_or_else(|| Error::internal(format!("register {} is not assigned", i)))   
    }

    pub fn getself(&mut self) -> Result<Rc<RObject>, Error> {
        self.get_current_regs_cloned(0)
    }

    pub fn must_getself(&mut self) -> Rc<RObject> {
        self.current_regs()[0].clone().expect("self is not assigned")
    }

    pub(crate) fn register_fn(&mut self, f: RFn) -> usize {
        self.fn_table.push(Rc::new(f));
        return self.fn_table.len() - 1;
    }
    
    pub(crate) fn get_fn(&self, i: usize) -> Option<Rc<RFn>> {
        self.fn_table.get(i).cloned()
    }

    pub fn get_class_by_name(&self, name: &str) -> Rc<RClass> {
        self.builtin_class_table.get(name).cloned().expect(format!("Class {} not found", name).as_str())
    }

    pub(crate) fn define_class(&mut self, name: &str, superclass: Option<Rc<RClass>>) -> Rc<RClass> {
        let superclass = match superclass {
            Some(c) => c,
            None => self.object_class.clone(),
        };
        let class = Rc::new(
            RClass::new(name, Some(superclass)),
        );
        let object = RObject::class(class.clone()).to_refcount_assigned();
        self.consts.insert(name.to_string(), object);
        class
    }

    pub(crate) fn define_standard_class(&mut self, name: &'static str) -> Rc<RClass> {
        let class = self.define_class(name, None);
        self.builtin_class_table.insert(name, class.clone());
        class
    }

    pub(crate) fn define_standard_class_under(&mut self, name: &'static str, sklass: Rc<RClass>) -> Rc<RClass> {
        let class = self.define_class(name, Some(sklass));
        self.builtin_class_table.insert(name, class.clone());
        class
    }
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

fn load_irep_1(reps: &mut [Irep], pos: usize) -> (IREP, usize) {
    let irep = &mut reps[pos];
    let mut irep1 = IREP {
        __id: pos,
        nlocals: irep.nlocals(),  
        nregs: irep.nregs(),
        rlen: irep.rlen(),
        code: Vec::new(),
        syms: Vec::new(),
        pool: Vec::new(),
        reps: Vec::new(),
        catch_target_pos: Vec::new(),
    };
    for sym in irep.syms.iter() {
        irep1.syms.push(RSym::new(sym.to_string_lossy().to_string()));
    }
    for str in irep.strvals.iter() {
        irep1.pool.push(RPool::Str(str.to_string_lossy().to_string()));
    }
    let code = interpret_insn(&mut irep.insn);
    for ch in irep.catch_handlers.iter() {
        let pos = ch.target;
        let (i, _) = code.iter().enumerate().find(|(_, op)| op.pos == pos).expect("catch handler mismatch");
        irep1.catch_target_pos.push(i);
    }
    irep1.catch_target_pos.sort();

    irep1.code = code;
    (irep1, pos + 1)
}

fn load_irep_0(reps: &mut [Irep], pos: usize) -> (IREP, usize) {
    let (mut irep0, newpos) = load_irep_1(reps, pos);
    let mut pos = newpos;
    for _ in 0..irep0.rlen {
        let (rep, newpos) = load_irep_0(reps, pos);
        pos = newpos;
        irep0.reps.push(Rc::new(rep));
    }
    (irep0, pos)
}

// This will consume the Rite object and return the IREP
fn rite_to_irep(rite: &mut Rite) -> IREP {
    let (irep0, _) = load_irep_0(&mut rite.irep, 0);
    irep0
}

#[derive(Debug, Clone)]
pub struct IREP {
    pub __id: usize,

    pub nlocals: usize,
    pub nregs: usize, // NOTE: is u8 better?
    pub rlen: usize,
    pub code: Vec<Op>,
    pub syms: Vec<RSym>,
    pub pool: Vec<RPool>,
    pub reps: Vec<Rc<IREP>>,
    pub catch_target_pos: Vec<usize>,
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

#[derive(Debug, Clone)]
pub struct ENV {
    pub upper: Option<Rc<ENV>>,
    pub captured: RefCell<Option<Vec<Option<Rc<RObject>>>>>,
    pub current_regs_offset: usize,
    pub is_expired: Cell<bool>,
}

impl ENV {
    pub fn has_captured(&self) -> bool {
        self.captured.borrow().is_some()
    }

    pub fn capture(&self, regs: &[Option<Rc<RObject>>]) {
        let mut captured = self.captured.borrow_mut();
        captured.replace(regs.iter().map(|r| r.clone()).collect());
    }

    pub fn capture_no_clone(&self, regs: Vec<Option<Rc<RObject>>>) {
        let mut captured = self.captured.borrow_mut();
        captured.replace(regs);
    }

    pub fn expire(&self) {
        self.is_expired.set(true);
    }

    pub fn expired(&self) -> bool {
        self.is_expired.get()
    }
}