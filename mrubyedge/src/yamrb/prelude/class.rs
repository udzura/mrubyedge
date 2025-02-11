use std::rc::Rc;

use crate::{yamrb::{helpers::{mrb_define_cmethod, mrb_funcall}, value::*, vm::VM}, Error};

use super::shared_memory::mrb_shared_memory_new;

pub(crate) fn initialize_class(vm: &mut VM) {
    let class_class = vm.define_standard_class("Class");

    let mrb_class_new = 
        Box::new(mrb_class_new);
    mrb_define_cmethod(vm, class_class.clone(), "new", mrb_class_new);
}

fn mrb_class_new(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let class = vm.getself();
    let class = match &class.value {
        RValue::Class(c) => c.clone(),
        _ => {
            return Err(Error::RuntimeError("Class#new must be called from class".to_string()));
        }
    };
    // Classes with special initializers
    match class.sym_id.name.as_str() {
        "String" => {
            todo!("String.new");
        }
        "Array" => {
            todo!("Array.new");
        }
        "Hash" => {
            todo!("Hash.new");
        }
        "SharedMemory" => {
            let sm = mrb_shared_memory_new(vm, args)?;
            return Ok(sm);
        }
        _ => {}        
    }

    let obj = RObject::instance(class).to_refcount_assigned();

    mrb_funcall(vm, Some(obj.clone()), "initialize", args)?;

    Ok(obj)
}