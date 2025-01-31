use std::rc::Rc;

use crate::Error;

use super::{optable::push_callinfo, value::{RClass, RFn, RObject, RProc, RSym}, vm::VM};

pub fn mrb_funcall(vm: &mut VM, top_self: Option<Rc<RObject>>, name: &str, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let recv = match top_self {
        Some(obj) => obj,
        None => vm.current_regs()[0].as_ref().unwrap().clone(),
    };
    let binding = recv.get_class(vm);
    let method = binding.find_method(name).expect("Method not found");
    
    if method.is_rb_func {
        push_callinfo(vm, RSym::new(name.to_string()), 0);

        // let old_irep = vm.current_irep.clone();
        let old_callinfo = vm.current_callinfo.take();

        vm.current_regs()[0].replace(recv.clone());
        for (i, arg) in args.iter().enumerate() {
            vm.current_regs()[i + 1].replace(arg.clone());
        }
    
        vm.pc.set(0);
        vm.current_irep = method.irep.as_ref().unwrap().clone();
        let res = vm.run().unwrap();

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
    };
    let mut procs = klass.procs.borrow_mut();
    procs.insert(name.to_string(), method);
}

pub fn mrb_define_method(_vm: &mut VM, klass: Rc<RClass>, name: &str, method: RProc) {
    let mut procs = klass.procs.borrow_mut();
    procs.insert(name.to_string(), method);
}
