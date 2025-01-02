use std::rc::Rc;

use crate::{yamrb::{value::*, vm::VM}, Error};

pub(crate) fn initialize_object(vm: &mut VM) {
    let object_class = vm.object_class.clone();
    vm.consts.insert("Object".to_string(), Rc::new(RObject {
        tt: RType::Class,
        value: RValue::Class(object_class.clone()),
    }));
    vm.builtin_class_table.insert("Object", object_class.clone());

    let mrb_kernel_puts = 
        Box::new(mrb_kernel_puts);
    let index = vm.register_fn(mrb_kernel_puts);
    object_class.procs.borrow_mut().insert("puts".to_string(), RProc {
        func: Some(index),
        is_rb_func: false,
        sym_id: Some("puts".into()),
        next: None,
        irep: None,
    });
}

pub fn mrb_kernel_puts(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let msg = args[0].clone();
    match &msg.value {
        RValue::String(s) => {
            println!("{}", s.borrow());
        }
        RValue::Integer(i) => {
            println!("{}", i);
        }
        _ => {
            dbg!(&msg);
            return Err(Error::RuntimeError("puts only accept string".to_string()));
        }
    }
    Ok(Rc::new(RObject::nil()))
}