use std::cell::RefCell;
use std::rc::Rc;

use crate::yamrb::shared_memory::SharedMemory;
use crate::yamrb::vm::VM;
use crate::{yamrb::{helpers::mrb_define_cmethod, value::{RObject, RValue, RType}}, Error};

pub(crate) fn initialize_shared_memory(vm: &mut VM) {
    let shared_memory_class = vm.define_standard_class("SharedMemory");

    mrb_define_cmethod(vm, shared_memory_class.clone(), "to_s", Box::new(mrb_shared_memory_to_string));
    mrb_define_cmethod(vm, shared_memory_class.clone(), "offset_in_memory", Box::new(mrb_shared_memory_offset_in_memory));
    mrb_define_cmethod(vm, shared_memory_class.clone(), "to_i", Box::new(mrb_shared_memory_offset_in_memory));
    mrb_define_cmethod(vm, shared_memory_class.clone(), "[]", Box::new(mrb_shared_memory_index_range));
    mrb_define_cmethod(vm, shared_memory_class.clone(), "[]=", Box::new(mrb_shared_memory_set_index_range));
    mrb_define_cmethod(vm, shared_memory_class.clone(), "read_by_size", Box::new(mrb_shared_memory_read_by_size));
}

pub fn mrb_shared_memory_new(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let size: u64 = args[0].as_ref().try_into().expect("arg[0] must be integer");
    let obj = RObject {
        tt: RType::SharedMemory,
        value: RValue::SharedMemory(Rc::new(RefCell::new(
            SharedMemory::new(size as usize),
        ))),
        object_id: u64::MAX.into()
    };
    Ok(obj.to_refcount_assigned())
}

fn mrb_shared_memory_offset_in_memory(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    let sm = match &this.value {
        RValue::SharedMemory(s) => s,
        _ => {
            return Err(Error::RuntimeError("SharedMemory#to_s must be called on a SharedMemory".to_string()));
        }
    };
    let offset = sm.borrow().offset_in_memory();
    Ok(Rc::new(RObject::integer(offset as i64)))
}

fn mrb_shared_memory_set_index_range(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
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
            return Err(Error::RuntimeError("Range should be passed on SharedMemory#[]=".to_string()));
        }
    };
    let sm = match &this.value {
        RValue::SharedMemory(s) => s,
        _ => {
            return Err(Error::RuntimeError("SharedMemory#to_s must be called on a SharedMemory".to_string()));
        }
    };
    let data: Vec<u8> = args[1].as_ref().try_into()?;
    if data.len() != (end - start + 1) as usize {
        return Err(Error::RuntimeError("Data length must be equal to range length".to_string()));
    }
    let mut sm = sm.borrow_mut();
    sm.write(start as usize, &data);
    Ok(this.clone())
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
    Ok(RObject::string_from_vec(range).to_refcount_assigned())
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
            return Err(Error::RuntimeError("this value's not a SharedMemory".to_string()));
        }
    };
    let range = sm.borrow().memory.as_ref()[(start as usize)..=(end as usize)].to_vec();
    Ok(RObject::string_from_vec(range).to_refcount_assigned())
}

// SharedMemory#read_by_size(size: Integer, offset: Integer) -> Integer
fn mrb_shared_memory_read_by_size(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    let size: usize = args[0].as_ref().try_into()?;
    let offset: usize = args[1].as_ref().try_into()?;

    let sm = match &this.value {
        RValue::SharedMemory(s) => s,
        _ => {
            return Err(Error::RuntimeError("SharedMemory#to_s must be called on a SharedMemory".to_string()));
        }
    };
    match size {
        1 => {
            let value = sm.borrow().memory.as_ref()[offset];
            Ok(Rc::new(RObject::integer(value as i64)))
        }
        2 => {
            let value = u16::from_le_bytes([
                sm.borrow().memory.as_ref()[offset],
                sm.borrow().memory.as_ref()[offset + 1],
            ]);
            Ok(Rc::new(RObject::integer(value as i64)))
        }
        4 => {
            let sm_borrowed = sm.borrow();
            let memory = sm_borrowed.memory.as_ref();
            let value = u32::from_le_bytes([
                memory[offset],
                memory[offset + 1],
                memory[offset + 2],
                memory[offset + 3],
            ]);
            Ok(Rc::new(RObject::integer(value as i64)))
        }
        8 => {
            let sm_borrowed = sm.borrow();
            let memory = sm_borrowed.memory.as_ref();
            let value = u64::from_le_bytes([
                memory[offset],
                memory[offset + 1],
                memory[offset + 2],
                memory[offset + 3],
                memory[offset + 4],
                memory[offset + 5],
                memory[offset + 6],
                memory[offset + 7],
            ]);
            Ok(Rc::new(RObject::integer(value as i64)))
        }
        _ => {
            Err(Error::RuntimeError("Invalid size passed".to_string()))
        }
    }
}

#[test]
fn test_mrb_shared_memory_new() {
    let mut vm = VM::empty();
    initialize_shared_memory(&mut vm);

    let args = vec![Rc::new(RObject::integer(10))];
    let sm = mrb_shared_memory_new(&mut vm, &args).expect("failed to create SharedMemory");
    match &sm.value {
        RValue::SharedMemory(s) => {
            assert_eq!(s.borrow().memory.as_ref().len(), 10);
        }
        _ => {
            panic!("not a SharedMemory");
        }
    }
}

#[test]
fn test_mrb_shared_memory_read_by_size() {
    let mut vm = VM::empty();
    initialize_shared_memory(&mut vm);

    let args = vec![Rc::new(RObject::integer(10))];
    let sm = mrb_shared_memory_new(&mut vm, &args).expect("failed to create SharedMemory");
    vm.current_regs()[0].replace(sm);

    let args = vec![Rc::new(RObject::integer(1)), Rc::new(RObject::integer(0))];
    let result = mrb_shared_memory_read_by_size(&mut vm, &args).expect("failed to read");
    let result: i64 = result.as_ref().try_into().expect("not an integer");
    assert_eq!(result, 0);

    let sm = vm.current_regs()[0].as_ref().cloned().unwrap();
    match &sm.value {
        RValue::SharedMemory(s) => {
            let data = vec![1, 2, 3, 4, 5, 6, 7];
            s.borrow_mut().write(0, &data);
        }
        _ => {
            panic!("not a SharedMemory");
        }
    }

    let args = vec![Rc::new(RObject::integer(1)), Rc::new(RObject::integer(0))];
    let result = mrb_shared_memory_read_by_size(&mut vm, &args).expect("failed to read");
    let result: i64 = result.as_ref().try_into().expect("not an integer");
    assert_eq!(result, 1);

    let args = vec![Rc::new(RObject::integer(2)), Rc::new(RObject::integer(1))];
    let result = mrb_shared_memory_read_by_size(&mut vm, &args).expect("failed to read");
    let result: i64 = result.as_ref().try_into().expect("not an integer");
    assert_eq!(result, 770);

    let args = vec![Rc::new(RObject::integer(4)), Rc::new(RObject::integer(3))];
    let result = mrb_shared_memory_read_by_size(&mut vm, &args).expect("failed to read");
    let result: i64 = result.as_ref().try_into().expect("not an integer");
    assert_eq!(result, 117835012);
}