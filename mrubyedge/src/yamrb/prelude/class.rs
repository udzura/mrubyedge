use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{yamrb::{helpers::{mrb_define_cmethod, mrb_funcall}, value::*, vm::VM}, Error};

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
    let obj = Rc::new(RObject {
        tt: RType::Instance,
        value: RValue::Instance(RInstance{
            class: class.clone(),
            ivar: RefCell::new(HashMap::new()),
            data: Vec::new(),
            ref_count: 1,
        }),
    });

    mrb_funcall(vm, Some(obj.clone()), "initialize", args)?;

    Ok(obj)
}