use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::rc::Rc;

// use crate::rite::binfmt::*;
use crate::rite::insn::{Fetched, OpCode};
use crate::{klass, rite::*};

#[derive(Debug)]
pub struct VM<'insn> {
    pub vm_id: u32,
    pub irep_arena: HashMap<usize, Rc<VMIrep<'insn>>>,
    pub top_irep: Rc<VMIrep<'insn>>,
    pub cur_irep: Rc<VMIrep<'insn>>,
    // pub insns: &'a [u8],
    pub pc: usize,
    pub class_arena: HashMap<usize, Rc<RefCell<RClass<'insn>>>>,
    pub target_class: Option<usize>,
    pub callinfo_vec: Option<Vec<CallInfo>>,
    pub exception: Option<Box<RObject>>,
    pub regs: HashMap<usize, Rc<RObject>>,
}

impl<'insn> VM<'insn> {
    pub fn open(mut rite: Rite<'insn>) -> VM<'insn> {
        let mut irep_arena = HashMap::default();
        let top_irep = VMIrep::from_raw_record(rite.irep.pop().unwrap());
        let top_irep = Rc::new(top_irep);

        for (i, irep) in rite.irep.into_iter().enumerate() {
            let irep = VMIrep::from_raw_record(irep);
            let irep = Rc::new(irep);
            irep_arena.insert(i, irep);
        }
        let class_arena = HashMap::default();

        let vm = VM {
            vm_id: 1,
            irep_arena,
            top_irep: top_irep.clone(),
            cur_irep: top_irep.clone(),
            pc: 0,
            class_arena,
            target_class: None,
            callinfo_vec: None,
            exception: None,
            regs: HashMap::new(),
        };
        vm
    }

    // TODO: move to klass.rs?
    pub fn prelude(&mut self) -> Result<(), Error> {
        let object_class = klass::new_builtin_object_class();
        let object_class = Rc::new(RefCell::new(object_class));
        self.target_class = Some(object_class.as_ref().borrow().sym_id as usize);

        self.class_arena
            .insert(klass::KLASS_SYM_ID_OBJECT as usize, object_class);

        Ok(())
    }

    pub fn eval_insn(&mut self) -> Result<(), Error> {
        let cur_irep = self.cur_irep.clone();
        let mut insns = cur_irep.as_ref().inst_head;
        while insns.len() > 0 {
            let op = insns[0];
            let opcode: OpCode = op.try_into()?;
            let fetched = insn::FETCH_TABLE[op as usize](&mut insns)?;
            // println!("insn: {:?} {:?}", opcode, fetched);
            eval_insn1(self, cur_irep.as_ref(), &opcode, &fetched)?;

            self.pc = self.cur_irep.ilen - insns.len();
        }

        Ok(())
    }
}

pub fn eval_insn1(
    vm: &mut VM,
    irep: &VMIrep,
    opcode: &OpCode,
    fetched: &Fetched,
) -> Result<(), Error> {
    match opcode {
        OpCode::STRING => {
            if let Fetched::BB(a, b) = fetched {
                let strval = &irep.pool[*b as usize];
                let RPool::StaticStr(s) = strval;
                let regval = RObject::RString(s.to_str().unwrap().to_string());
                vm.regs.insert(*a as usize, Rc::new(regval));
            } else {
                unreachable!("not BB insn")
            }
        }

        OpCode::SSEND => {
            if let Fetched::BBB(a, b, c) = fetched {
                let cur_irep = vm.cur_irep.as_ref();

                let reg_begin = *a;
                let arg_count = *c;
                let cur_self = RObject::RInstance {
                    class_index: vm.target_class.unwrap(),
                };

                let sym = cur_irep.syms[*b as usize]
                    .value
                    .to_str()
                    .unwrap()
                    .to_string();
                let mut args = Vec::new();
                if arg_count > 0 {
                    for i in (reg_begin + 1)..=(reg_begin + arg_count) {
                        args.push(i as usize)
                    }
                }

                let ret = call_func(vm, &cur_self, sym, &args)?;
                vm.regs.insert(reg_begin as usize, Rc::new(ret));
            } else {
                unreachable!("not BBB insn")
            }
        }

        OpCode::RETURN => {
            if let Fetched::B(a) = fetched {
                let reg_ret = *a as usize;
                if let Some(val) = vm.regs.remove(&reg_ret) {
                    vm.regs.insert(0, val);
                }
            } else {
                unreachable!("not B insn")
            }
        }

        OpCode::STOP => {
            return Ok(());
        }

        op => {
            unimplemented!("unsupported opcpde {:?}", op)
        }
    }
    Ok(())
}

pub fn call_func<'insn>(
    vm: &mut VM<'insn>,
    recv: &RObject,
    sym: String,
    args: &[usize],
) -> Result<RObject, Error> {
    match recv {
        RObject::RInstance { class_index } => {
            let klass = vm.class_arena.get(class_index).unwrap().clone();
            let klass = klass.as_ref().borrow();
            match klass.methods.get(&sym) {
                Some(method) => {
                    if let Method::CMethod(func) = method.body {
                        let mut fn_args = Vec::<Rc<RObject>>::default();
                        for i in args.iter() {
                            if let Some(value) = vm.regs.get(i) {
                                fn_args.push(value.clone());
                            } else {
                                eprintln!("reg not found");
                                return Err(Error::General);
                            }
                        }
                        let ret = func(vm, &fn_args);
                        return Ok(ret);
                    } else {
                        todo!("eval internal irep");
                    }
                }
                None => {
                    eprint!("todo: method_missing");
                    return Err(Error::General);
                }
            }
        }
        _ => {
            todo!("some day")
        }
    }
    // Ok(RObject::Nil)
}

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

#[derive(Debug, Clone)]
pub enum RPool {
    StaticStr(CString),
}

#[derive(Debug)]
pub struct RSym {
    pub value: CString,
}

#[derive(Debug)]
pub enum RObject {
    Class { class_index: usize },
    RInstance { class_index: usize },
    RString(String),
    True,
    False,
    Nil,
    // ...
}

#[derive(Debug)]
pub struct RClass<'insn> {
    pub sym_id: u32,
    // pub num_builtin_method: usize,
    pub super_klass: Rc<Option<Box<RClass<'insn>>>>,
    pub methods: HashMap<String, RMethod<'insn>>,
}

#[derive(Debug)]
pub struct RMethod<'insn> {
    pub sym_id: u32,
    pub body: Method<'insn>,
}

type RFn = for<'insn> fn(&mut VM<'insn>, &[Rc<RObject>]) -> RObject;

#[derive(Debug)]
pub enum Method<'insn> {
    RubyMethod(Box<VMIrep<'insn>>),
    CMethod(RFn),
}

#[derive(Debug)]
pub struct CallInfo {
    // https://github.com/mrubyc/mrubyc/blob/5fab2b85dce8fc0780293235df6c0daa5fd57dce/src/vm.h#L111-L126
}
