use std::rc::Rc;

use crate::Error;

use super::{optable::push_callinfo, value::{RClass, RFn, RObject, RProc, RSym, RValue}, vm::VM};

fn call_block(vm: &mut VM, block: RProc, recv: Rc<RObject>, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    push_callinfo(vm, RSym::new("block".to_string()), args.len());

    let old_callinfo = vm.current_callinfo.take();

    // Since call_block does not move the registers offset,
    // keep the state before the call.
    let prev_self = vm.current_regs()[0].replace(recv);

    let mut prev_args = vec![];
    for (i, arg) in args.iter().enumerate() {
        let old = vm.current_regs()[i + 1].replace(arg.clone());
        prev_args.push(old);
    }

    vm.pc.set(0);
    vm.current_irep = block.irep.as_ref().unwrap().clone();
    let res = vm.run().unwrap();

    if let Some(prev) = prev_self {
        vm.current_regs()[0].replace(prev);
    } else {
        vm.current_regs()[0].take();
    }
    for (i, prev_arg) in prev_args.into_iter().enumerate() {
        if let Some(prev) = prev_arg {
            vm.current_regs()[i + 1].replace(prev);
        } else {
            vm.current_regs()[i + 1].take();
        }
    }

    if let Some(ci) = old_callinfo {
        if ci.prev.is_some() {
            vm.current_callinfo.replace(ci.prev.clone().unwrap());
        }
        vm.current_irep = ci.pc_irep.clone();
        vm.pc.set(ci.pc);
        vm.current_regs_offset = ci.current_regs_offset;
        vm.target_class = ci.target_class.clone();
    }

    Ok(res.clone())
}

pub fn mrb_call_block(vm: &mut VM, block: Rc<RObject>, recv: Option<Rc<RObject>>, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let block = match &block.value {
        RValue::Proc(p) => p.clone(),
        _ => panic!("Not a block"),
        
    };
    let recv = match recv {
        Some(r) => r,
        None => block.block_self.clone().unwrap(),
    };
    call_block(vm, block, recv, args)
}

pub fn mrb_funcall(vm: &mut VM, top_self: Option<Rc<RObject>>, name: &str, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let recv: Rc<RObject> = match top_self {
        Some(obj) => obj,
        None => vm.current_regs()[0].as_ref().unwrap().clone(),
    };
    let binding = recv.as_ref().get_class(vm);
    let method = binding.as_ref().find_method(name).expect("Method not found").clone();
    
    if method.is_rb_func {
        call_block(vm, method, recv.clone(), args)
    } else {
        vm.current_regs_offset += 2;
        vm.current_regs()[0].replace(recv.clone());

        let func = vm.fn_table[method.func.unwrap()].clone();
        let res = func(vm, &args);
        vm.current_regs_offset -= 2;

        if res.is_err() {
            vm.error_code = 255;
        }
        match res {
            Ok(o) => Ok(o.clone()),
            Err(e) => Err(e),
        }
    }
}

pub fn mrb_define_cmethod(vm: &mut VM, klass: Rc<RClass>, name: &str, cmethod: RFn) {
    let index = vm.register_fn(cmethod);
    let method = RProc {
        is_rb_func: false,
        sym_id: Some(RSym::new(name.to_string())),
        next: None,
        irep: None,
        func: Some(index),
        block_self: None,
    };
    let mut procs = klass.procs.borrow_mut();
    procs.insert(name.to_string(), method);
}

pub fn mrb_define_method(_vm: &mut VM, klass: Rc<RClass>, name: &str, method: RProc) {
    let mut procs = klass.procs.borrow_mut();
    procs.insert(name.to_string(), method);
}
