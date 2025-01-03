use std::rc::Rc;

use crate::{yamrb::{helpers::mrb_define_cmethod, value::*, vm::VM}, Error};

pub(crate) fn initialize_object(vm: &mut VM) {
    let object_class = vm.object_class.clone();
    vm.consts.insert("Object".to_string(), Rc::new(RObject {
        tt: RType::Class,
        value: RValue::Class(object_class.clone()),
    }));
    vm.builtin_class_table.insert("Object", object_class.clone());

    #[cfg(feature = "wasi")]
    {
        let mrb_kernel_puts = 
            Box::new(mrb_kernel_puts);
        mrb_define_cmethod(vm, object_class.clone(), "puts", mrb_kernel_puts);
        let mrb_kernel_debug = 
            Box::new(mrb_kernel_debug);
        mrb_define_cmethod(vm, object_class.clone(), "debug", mrb_kernel_debug);
    }

    let mrb_object_initialize = 
    Box::new(mrb_object_initialize);
    mrb_define_cmethod(vm, object_class.clone(), "initialize", mrb_object_initialize);
}

#[cfg(feature = "wasi")]
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

#[cfg(feature = "wasi")]
pub fn mrb_kernel_debug(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    for (i, obj) in args.iter().enumerate() {
        dbg!(i, obj.clone());
    }
    Ok(Rc::new(RObject::nil()))
}

pub fn mrb_object_initialize(_vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    // Abstract method; do nothing
    Ok(Rc::new(RObject::nil()))
}