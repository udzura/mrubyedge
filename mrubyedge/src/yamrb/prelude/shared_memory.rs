use std::cell::RefCell;
use std::rc::Rc;

use crate::yamrb::shared_memory::SharedMemory;
use crate::yamrb::vm::VM;
use crate::{yamrb::{helpers::mrb_define_cmethod, value::{RObject, RValue, RType}}, Error};

pub(crate) fn initialize_shared_memory(vm: &mut VM) {
    let shared_memory_class = vm.define_standard_class("SharedMemory");

    let mrb_shared_memory_new = 
        Box::new(mrb_shared_memory_new);
    mrb_define_cmethod(vm, shared_memory_class.clone(), "new", mrb_shared_memory_new);
    let mrb_shared_memory_to_string = 
        Box::new(mrb_shared_memory_to_string);
    mrb_define_cmethod(vm, shared_memory_class.clone(), "to_s", mrb_shared_memory_to_string);
    let mrb_shared_memory_index_range = 
        Box::new(mrb_shared_memory_index_range);
    mrb_define_cmethod(vm, shared_memory_class.clone(), "[]", mrb_shared_memory_index_range);
}

fn mrb_shared_memory_new(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let size: u64 = args[0].as_ref().try_into().expect("arg[0] must be integer");
    let obj = Rc::new(RObject {
        tt: RType::SharedMemory,
        value: RValue::SharedMemory(Rc::new(RefCell::new(
            SharedMemory::new(size as usize),
        ))),
    });
    Ok(obj)
}

fn mrb_shared_memory_to_string(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    let sm = match &this.value {
        RValue::SharedMemory(s) => s,
        _ => {
            return Err(Error::RuntimeError("SharedMemory#to_s must be called on a SharedMemory".to_string()));
        }
    };
    let range = sm.borrow().memory.as_ref().to_vec();
    Ok(Rc::new(RObject::string_from_vec(range)))
}

fn mrb_shared_memory_index_range(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    let (start, end) = match &args[0].as_ref().value {
        RValue::Range(start, end, exclusive) => {
            let start: u64 = start.as_ref().try_into()?;
            let end: u64 = end.as_ref().try_into()?;
            if *exclusive {
                (start, end-1)
            } else {
                (start, end)
            }
        }
        _ => {
            return Err(Error::RuntimeError("Range should be passed on SharedMemory#[]".to_string()));
        }
    };
    let sm = match &this.value {
        RValue::SharedMemory(s) => s,
        _ => {
            return Err(Error::RuntimeError("SharedMemory#to_s must be called on a SharedMemory".to_string()));
        }
    };
    let range = sm.borrow().memory.as_ref()[(start as usize)..=(end as usize)].to_vec();
    Ok(Rc::new(RObject::string_from_vec(range)))
}