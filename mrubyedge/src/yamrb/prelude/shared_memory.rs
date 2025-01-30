use std::rc::Rc;

use crate::yamrb::shared_memory::SharedMemory;
use crate::yamrb::vm::VM;
use crate::{yamrb::{helpers::mrb_define_cmethod, value::{RObject, RValue, RType}, vm::VM}, Error};

pub(crate) fn initialize_string(vm: &mut VM) {
    let shared_memory_class = vm.define_standard_class("SharedMemory");

    let mrb_shared_memory_new = 
        Box::new(mrb_shared_memory_new);
    mrb_define_cmethod(vm, class_class.clone(), "new", mrb_class_new);
}

fn mrb_shared_memory_new(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let size: u32 = args[0].clone().try_into().expect("arg[0] must be integer");
    let obj = Rc::new(RObject {
        tt: RType::SharedMemory,
        value: RValue::SharedMemory(Rc::new(RefCell::new(
            SharedMemory::new(size as usize),
        ))),
    });
    Ok(Rc::new(obj))
}