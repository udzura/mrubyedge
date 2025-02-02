use std::rc::Rc;

use crate::{yamrb::{helpers::{mrb_call_block, mrb_define_cmethod}, value::{RObject, RValue}, vm::VM}, Error};

pub(crate) fn initialize_range(vm: &mut VM) {
    let range_class = vm.define_standard_class("Range");
    
    mrb_define_cmethod(vm, range_class.clone(), "include?", Box::new(mrb_range_is_include));
    mrb_define_cmethod(vm, range_class.clone(), "each", Box::new(mrb_range_each));
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

pub fn mrb_range_each(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    let block = &args[0];
    match &this.value {
        RValue::Range(start, end, exclusive) => {
            match (&start.value, &end.value) {
                (RValue::Integer(start), RValue::Integer(end)) => {
                    let start = *start;
                    let mut end = *end;
                    if *exclusive {
                        end = end - 1;
                    }
                    for i in start..=end {
                        let args = vec![Rc::new(RObject::integer(i))];
                        mrb_call_block(vm, block.clone(), None, &args)?;
                    }
                }
                _ => {
                    return Err(Error::RuntimeError("Range#each must be called on a integer Range with block (for now)".to_string()));
                }
            }
        }
        _ => {
            return Err(Error::RuntimeError("Range#each must be called on a Range".to_string()));
        }
    }
    Ok(this.clone())
}