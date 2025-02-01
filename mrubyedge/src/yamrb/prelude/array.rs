use std::rc::Rc;

use crate::{yamrb::{helpers::mrb_define_cmethod, value::{RObject, RValue}, vm::VM}, Error};

pub(crate) fn initialize_array(vm: &mut VM) {
    let array_class = vm.define_standard_class("Array");

    mrb_define_cmethod(vm, array_class.clone(), "push", Box::new(mrb_array_push_self));
}

fn mrb_array_push_self(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    mrb_array_push(this, args)
}

pub fn mrb_array_push(this: Rc<RObject>, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let mut array = match &this.value {
        RValue::Array(a) => a.clone(),
        _ => {
            return Err(Error::RuntimeError("Array#push must be called on an Array".to_string()));
        }
    };
    let array = array.get_mut();
    for arg in args {
        array.push(arg.clone());
    }
    Ok(this)
}
