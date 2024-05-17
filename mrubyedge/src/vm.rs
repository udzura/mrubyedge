use std::collections::HashMap;
use std::rc::Rc;
use std::{any::Any, cell::RefCell};

// use crate::rite::binfmt::*;
use crate::rite::insn::{Fetched, OpCode};
pub use crate::value::*;
use crate::{klass, mrb_helper, rite::*};

#[derive(Debug)]
pub struct VM<'insn> {
    pub vm_id: u32,
    pub irep_arena: HashMap<usize, Rc<VMIrep<'insn>>>,
    pub top_irep: Rc<VMIrep<'insn>>,
    pub cur_irep: Rc<VMIrep<'insn>>,
    // pub insns: &'a [u8],
    pub pc: usize,
    pub class_arena: HashMap<usize, Rc<RefCell<RClass<'insn>>>>,
    pub const_arena: HashMap<String, Rc<RObject>>,
    pub target_class: Option<usize>,
    pub callinfo_vec: Vec<CallInfo<'insn>>,
    pub exception: Option<Box<RObject>>,
    pub regs: HashMap<usize, Rc<RObject>>,
}

impl<'insn> VM<'insn> {
    pub fn open(mut rite: Rite<'insn>) -> VM<'insn> {
        let mut irep_arena = HashMap::default();
        let top_irep_data = rite.irep.remove(0);
        let top_irep = VMIrep::from_raw_record(top_irep_data);
        let top_irep = Rc::new(top_irep);

        for (i, irep) in rite.irep.into_iter().enumerate() {
            let irep = VMIrep::from_raw_record(irep);
            let irep = Rc::new(irep);
            irep_arena.insert(i, irep);
        }
        let class_arena = HashMap::default();
        let const_arena = HashMap::default();

        let vm = VM {
            vm_id: 1,
            irep_arena,
            top_irep: top_irep.clone(),
            cur_irep: top_irep.clone(),
            pc: 0,
            class_arena,
            const_arena,
            target_class: None,
            callinfo_vec: Vec::new(),
            exception: None,
            regs: HashMap::new(),
        };
        vm
    }

    // TODO: move to klass.rs?
    pub fn prelude(&mut self) -> Result<(), Error> {
        let object_class = klass::new_builtin_object_class();
        let object_class = Rc::new(RefCell::new(object_class));
        let objclass_sym = object_class.as_ref().borrow().sym_id as usize;
        self.target_class = Some(objclass_sym);

        self.class_arena
            .insert(klass::KLASS_SYM_ID_OBJECT as usize, object_class);
        self.const_arena.insert(
            "Object".to_string(),
            Rc::new(RObject::Class {
                class_index: objclass_sym,
            }),
        );

        #[cfg(feature = "wasi")]
        {
            let random_class = klass::new_builtin_random_class();
            let random_class = Rc::new(RefCell::new(random_class));
            let random_sym = random_class.as_ref().borrow().sym_id as usize;
            self.class_arena
                .insert(klass::KLASS_SYM_ID_RANDOM as usize, random_class);
            self.const_arena.insert(
                "Random".to_string(),
                Rc::new(RObject::Class {
                    class_index: random_sym,
                }),
            );

            let time_class = klass::new_builtin_time_class();
            let time_class = Rc::new(RefCell::new(time_class));
            let time_sym = time_class.as_ref().borrow().sym_id as usize;
            self.class_arena
                .insert(klass::KLASS_SYM_ID_TIME as usize, time_class);
            self.const_arena.insert(
                "Time".to_string(),
                Rc::new(RObject::Class {
                    class_index: time_sym,
                }),
            );
        }

        let top_self = RObject::RInstance {
            class_index: objclass_sym,
            data: Rc::new(RefCell::new(Box::new(()) as Box<dyn Any>)),
        };
        self.regs.insert(0, Rc::new(top_self));

        Ok(())
    }

    pub fn eval_insn(&mut self) -> Result<(), Error> {
        let cur_irep = self.cur_irep.clone();
        let len = cur_irep.as_ref().inst_head.len();
        while self.pc < len {
            let mut insns = &cur_irep.as_ref().inst_head[self.pc..];
            let op = insns[0];
            let opcode: OpCode = op.try_into()?;
            let before_len = insns.len();
            let fetched = insn::FETCH_TABLE[op as usize](&mut insns)?;
            let insn_len = before_len - insns.len();
            // println!("insn: {:?} {:?}", opcode, fetched);
            eval_insn1(self, cur_irep.as_ref(), &opcode, &fetched, insn_len)?;
        }

        Ok(())
    }
}

pub fn eval_insn1(
    vm: &mut VM,
    irep: &VMIrep,
    opcode: &OpCode,
    fetched: &Fetched,
    ilen: usize,
) -> Result<(), Error> {
    vm.pc += ilen;

    match opcode {
        OpCode::NOP => {
            fetched.as_z()?;
        }

        OpCode::MOVE => {
            let (a, b) = fetched.as_bb()?;
            let dst = a as usize;
            let src = b as usize;
            if let Some(val) = vm.regs.get(&src) {
                vm.regs.insert(dst, val.clone());
            }
        }

        OpCode::LOADL => {
            let (a, b) = fetched.as_bb()?;
            let dst = a as usize;
            let src = b as usize;
            if let Some(val) = vm.cur_irep.as_ref().pool.get(src) {
                vm.regs.insert(dst, Rc::new(val.into()));
            }
        }

        OpCode::LOADI => {
            let (a, b) = fetched.as_bb()?;
            let dst = a as usize;
            let val = b;
            let val = Rc::new(RObject::RInteger(val as i64));

            vm.regs.insert(dst, val);
        }

        OpCode::LOADINEG => {
            let (a, b) = fetched.as_bb()?;
            let dst = a as usize;
            let val = b;
            let val = Rc::new(RObject::RInteger(val as i64 * -1));

            vm.regs.insert(dst, val);
        }

        OpCode::JMP => {
            let a = fetched.as_s()?;
            let off = a as usize;
            vm.pc += off;
        }

        OpCode::JMPIF => {
            let (a, b) = fetched.as_bs()?;
            let cond = a as usize;
            let off = b as usize;
            let cond = vm.regs.get(&cond).unwrap().clone();

            if let RObject::RBool(cond) = cond.as_ref() {
                if *cond {
                    vm.pc += off;
                }
                return Ok(());
            }
        }

        OpCode::JMPNOT => {
            let (a, b) = fetched.as_bs()?;
            let cond = a as usize;
            let off = b as usize;
            let cond = vm.regs.get(&cond).unwrap().clone();

            if let RObject::RBool(cond) = cond.as_ref() {
                if !(*cond) {
                    vm.pc += off;
                }
                return Ok(());
            }
        }

        OpCode::ADDI => {
            let (a, b) = fetched.as_bb()?;
            let dst = a as usize;
            let val = b as i64;
            let target = vm.regs.remove(&dst).unwrap();

            if let RObject::RInteger(orig) = target.as_ref() {
                let ans = *orig + val;
                vm.regs.insert(dst, Rc::new(RObject::RInteger(ans)));
                return Ok(());
            }
        }

        OpCode::SUBI => {
            let (a, b) = fetched.as_bb()?;
            let dst = a as usize;
            let val = b as i64;
            let target = vm.regs.remove(&dst).unwrap();

            if let RObject::RInteger(orig) = target.as_ref() {
                let ans = *orig - val;
                vm.regs.insert(dst, Rc::new(RObject::RInteger(ans)));
                return Ok(());
            }
        }

        OpCode::ADD => {
            let a = fetched.as_b()?;
            let dst = a as usize;
            let dst2 = dst + 1;
            let target = vm.regs.remove(&dst).unwrap();
            let target2 = vm.regs.get(&dst2).unwrap().clone();

            if let RObject::RInteger(t1) = target.as_ref() {
                if let RObject::RInteger(t2) = target2.as_ref() {
                    let ans = *t1 + *t2;
                    vm.regs.insert(dst, Rc::new(RObject::RInteger(ans)));
                    return Ok(());
                }
            }
        }

        OpCode::SUB => {
            let a = fetched.as_b()?;
            let dst = a as usize;
            let dst2 = dst + 1;
            let target = vm.regs.remove(&dst).unwrap();
            let target2 = vm.regs.get(&dst2).unwrap().clone();

            if let RObject::RInteger(t1) = target.as_ref() {
                if let RObject::RInteger(t2) = target2.as_ref() {
                    let ans = *t1 - *t2;
                    vm.regs.insert(dst, Rc::new(RObject::RInteger(ans)));
                    return Ok(());
                }
            }
        }

        OpCode::LOADI_0
        | OpCode::LOADI_1
        | OpCode::LOADI_2
        | OpCode::LOADI_3
        | OpCode::LOADI_4
        | OpCode::LOADI_5
        | OpCode::LOADI_6
        | OpCode::LOADI_7 => {
            let a = fetched.as_b()?;
            let dst = a as usize;
            let val = (*opcode as i64) - (OpCode::LOADI_0 as i64);
            let val = Rc::new(RObject::RInteger(val));

            vm.regs.insert(dst, val);
        }

        OpCode::LT => {
            let a = fetched.as_b()?;
            let lhs = a as usize;
            let rhs = lhs + 1;

            let lhs = vm.regs.get(&lhs).unwrap().clone();
            let rhs = vm.regs.get(&rhs).unwrap().clone();

            if let RObject::RInteger(lhs) = lhs.as_ref() {
                if let RObject::RInteger(rhs) = rhs.as_ref() {
                    let cond = *lhs < *rhs;
                    vm.regs.insert(a as usize, Rc::new(RObject::RBool(cond)));
                    return Ok(());
                }
            }

            unreachable!("type error")
        }

        OpCode::GT => {
            let a = fetched.as_b()?;
            let lhs = a as usize;
            let rhs = lhs + 1;

            let lhs = vm.regs.get(&lhs).unwrap().clone();
            let rhs = vm.regs.get(&rhs).unwrap().clone();

            if let RObject::RInteger(lhs) = lhs.as_ref() {
                if let RObject::RInteger(rhs) = rhs.as_ref() {
                    let cond = *lhs > *rhs;
                    vm.regs.insert(a as usize, Rc::new(RObject::RBool(cond)));
                    return Ok(());
                }
            }

            unreachable!("type error")
        }

        OpCode::GETCONST => {
            let (a, b) = fetched.as_bb()?;
            let cur_irep = vm.cur_irep.as_ref();
            let const_sym = &cur_irep.syms[b as usize];
            let const_sym = const_sym.value.to_str().unwrap().to_owned();
            let obj = vm.const_arena.get(&const_sym).unwrap();
            vm.regs.insert(a as usize, obj.clone());
        }

        OpCode::STRING => {
            let (a, b) = fetched.as_bb()?;
            let strval = &irep.pool[b as usize];
            let RPool::StaticStr(s) = strval else {
                unreachable!("not string const")
            };
            let regval = RObject::RString(s.to_str().unwrap().to_string());
            vm.regs.insert(a as usize, Rc::new(regval));
        }

        OpCode::SSEND => {
            let (a, b, c) = fetched.as_bbb()?;
            let cur_irep = vm.cur_irep.as_ref();

            let reg_begin = a;
            let arg_count = c;
            let cur_self = RObject::RInstance {
                class_index: vm.target_class.unwrap(),
                data: Rc::new(RefCell::new(Box::new(()) as Box<dyn Any>)),
            };

            let sym = cur_irep.syms[b as usize]
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
            vm.regs.insert(reg_begin as usize, ret);
        }

        OpCode::SEND => {
            let (a, b, c) = fetched.as_bbb()?;
            let cur_irep = vm.cur_irep.as_ref();

            let reg_begin = a;
            let arg_count = c;
            let pos = a as usize;
            let get_self = vm.regs.get(&pos).unwrap().clone();

            let sym = cur_irep.syms[b as usize]
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

            let ret = call_func(vm, &get_self, sym, &args)?;
            vm.regs.insert(reg_begin as usize, ret);
        }

        OpCode::TCLASS => {
            let a = fetched.as_b()?;
            let reg_set = a as usize;
            let tclass = RObject::Class {
                class_index: vm.target_class.unwrap(),
            };
            vm.regs.insert(reg_set, Rc::new(tclass));
        }

        OpCode::METHOD => {
            let (a, b) = fetched.as_bb()?;
            let reg_set = a as usize;
            let irep_index = b as usize;
            let proc = RObject::RProc { irep_index };
            vm.regs.insert(reg_set, Rc::new(proc));
        }

        OpCode::RETURN => {
            let a = fetched.as_b()?;
            let reg_ret = a as usize;
            if let Some(val) = vm.regs.remove(&reg_ret) {
                vm.regs.insert(0, val);
            }
        }

        OpCode::DEF => {
            let (a, b) = fetched.as_bb()?;
            let reg_tclass = a as usize;
            let reg_proc = reg_tclass + 1;
            let sym_id = b as usize;
            let sym_name = &vm.cur_irep.syms[sym_id].value.to_str().unwrap();
            if let Some(proc) = vm.regs.remove(&reg_proc) {
                let tclass = vm.regs.remove(&reg_tclass).unwrap();
                if let RObject::Class { class_index } = tclass.as_ref() {
                    if let RObject::RProc { irep_index } = proc.as_ref() {
                        let rclass = vm.class_arena.get(class_index).unwrap().clone();
                        let mut rclass = rclass.borrow_mut();
                        let irep = vm.irep_arena.get(irep_index).unwrap();
                        let body = Method::RubyMethod(irep.clone());

                        rclass.methods.insert(
                            sym_name.to_string(),
                            RMethod {
                                sym_id: 10001,
                                body,
                            },
                        );
                    }
                } else {
                    unreachable!("tclass");
                }
            } else {
                unreachable!("proc reg");
            }
        }

        OpCode::ENTER => {
            // TBA
            return Ok(());
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

fn call_func<'insn>(
    vm: &mut VM<'insn>,
    recv: &RObject,
    sym: String,
    args: &[usize],
) -> Result<Rc<RObject>, Error> {
    let mut fn_args = Vec::<Rc<RObject>>::default();
    for i in args.iter() {
        if let Some(value) = vm.regs.get(i) {
            fn_args.push(value.clone());
        } else {
            eprintln!("reg not found");
            return Err(Error::General);
        }
    }

    mrb_helper::mrb_funcall(vm, recv, sym, &fn_args)
}

pub fn push_callinfo(vm: &mut VM) {
    let mut old_regs = HashMap::new();
    for (k, obj) in vm.regs.iter() {
        old_regs.insert(*k, obj.clone());
    }
    vm.regs.clear();

    let callinfo = CallInfo {
        cur_pc: vm.pc,
        cur_regs: old_regs,
        cur_irep: vm.cur_irep.clone(),
        target_class: vm.target_class,
    };
    vm.callinfo_vec.push(callinfo);
    vm.pc = 0;
}

pub fn pop_callinfo(vm: &mut VM) {
    let callinfo = vm.callinfo_vec.pop().unwrap();
    for (k, obj) in callinfo.cur_regs.iter() {
        vm.regs.insert(*k, obj.clone());
    }

    vm.cur_irep = callinfo.cur_irep.clone();
    vm.target_class = callinfo.target_class;
    vm.pc = callinfo.cur_pc;
}
