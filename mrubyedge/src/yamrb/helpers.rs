use std::rc::Rc;

use crate::Error;

use super::{optable::push_callinfo, value::{RClass, RFn, RObject, RProc, RSym}, vm::VM};

pub fn mrb_funcall(vm: &mut VM, top_self: Option<Rc<RObject>>, name: String, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let top_self = match top_self {
        Some(obj) => obj,
        None => vm.current_regs()[0].as_ref().unwrap().clone(),
    };
    let binding = top_self.get_class(vm);
    let binding = binding.procs.borrow();
    let method = binding.get(&name).unwrap();
    if method.is_rb_func {
        vm.current_regs()[0].replace(top_self.clone());
        for (i, arg) in args.iter().enumerate() {
            vm.current_regs()[i + 1].replace(arg.clone());
        }
        push_callinfo(vm, RSym::new(name), args.len());
    
        vm.pc.set(0);
        vm.current_irep = method.irep.as_ref().unwrap().clone();
        let res = vm.run().unwrap();

        Ok(res.clone())
    } else {
        let func = vm.fn_table[method.func.unwrap()].clone();
        let res = func(vm, &args);
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
