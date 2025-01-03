use std::rc::Rc;

use crate::{yamrb::{helpers::mrb_define_cmethod, value::{RObject, RValue}, vm::VM}, Error};

pub(crate) fn initialize_range(vm: &mut VM) {
    let range_class = vm.define_standard_class("Range");
    
    mrb_define_cmethod(vm, range_class.clone(), "include?", Box::new(mrb_range_is_include));
}

pub fn mrb_range_is_include(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    match &this.value {
        RValue::Range(start, end, exclusive) => {
            let obj = args[0].clone();
            match (&start.value, &end.value, &obj.value) {
                (RValue::Integer(start), RValue::Integer(end), RValue::Integer(obj)) => {
                    if *exclusive {
                        Ok(Rc::new(RObject::boolean(*start <= *obj && *obj < *end)))
                    } else {
                        Ok(Rc::new(RObject::boolean(*start <= *obj && *obj <= *end)))
                    }
                }
                (RValue::Integer(start), RValue::Integer(end), RValue::Float(obj)) => {
                    let obj = *obj as i64;
                    if *exclusive {
                        Ok(Rc::new(RObject::boolean(*start <= obj && obj < *end)))
                    } else {
                        Ok(Rc::new(RObject::boolean(*start <= obj && obj <= *end)))
                    }
                }
                _ => {
                    return Ok(Rc::new(RObject::boolean(false)));
                }
            }
        }
        _ => {
            return Err(Error::RuntimeError("Range#include? must be called on a Range".to_string()));
        }
    }
}
