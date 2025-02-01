use std::rc::Rc;

use crate::{yamrb::{helpers::mrb_define_cmethod, value::{RObject, RValue}, vm::VM}, Error};

pub(crate) fn initialize_array(vm: &mut VM) {
    let array_class = vm.define_standard_class("Array");

    mrb_define_cmethod(vm, array_class.clone(), "push", Box::new(mrb_array_push_self));
    mrb_define_cmethod(vm, array_class.clone(), "[]", Box::new(mrb_array_get_index_self));
    mrb_define_cmethod(vm, array_class.clone(), "[]=", Box::new(mrb_array_set_index_self));
    mrb_define_cmethod(vm, array_class.clone(), "pack", Box::new(mrb_array_pack));
}

fn mrb_array_push_self(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    mrb_array_push(this, args)
}

pub fn mrb_array_push(this: Rc<RObject>, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    match &this.value {
        RValue::Array(a) => {
            let mut array = a.borrow_mut();
            for arg in args {
                array.push(arg.clone());
            }
            Ok(this.clone())
        },
        _ => {
            Err(Error::RuntimeError("Array#push must be called on an Array".to_string()))
        }
    }
}

fn mrb_array_get_index_self(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    mrb_array_get_index(this, args)
}

pub fn mrb_array_get_index(this: Rc<RObject>, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let array = match &this.value {
        RValue::Array(a) => a.clone(),
        _ => {
            return Err(Error::RuntimeError("Array#push must be called on an Array".to_string()));
        }
    };
    let index: u32 = args[0].as_ref().try_into()?;
    let value = array.borrow()[index as usize].clone();
    Ok(value)
}

fn mrb_array_set_index_self(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    mrb_array_set_index(this, args)
}

pub fn mrb_array_set_index(this: Rc<RObject>, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let index: usize = args[0].as_ref().try_into()?;
    let value = &args[1];
    match &this.value {
        RValue::Array(a) => {
            let mut a = a.borrow_mut();
            a.insert(index, value.clone());
        }
        _ => {
            return Err(Error::RuntimeError("Array#push must be called on an Array".to_string()));
        }
    };
    Ok(value.clone())
}

fn mrb_array_pack(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    let format: Vec<u8> = args[0].as_ref().try_into()?;
    let mut buf = vec![];
    match &this.value {
        RValue::Array(a) => {
            let a = a.borrow();
            let mut index: usize = 0;
            for c in format.iter() {
                match c {
                    b'Q' => {
                        let value: u64 = a[index].as_ref().try_into()?;
                        buf.extend_from_slice(&value.to_le_bytes());
                        index += 1;
                    }
                    b'q' => {
                        let value: i64 = a[index].as_ref().try_into()?;
                        buf.extend_from_slice(&value.to_le_bytes());
                        index += 1;
                    }
                    b'L' | b'I' => {
                        let value: u32 = a[index].as_ref().try_into()?;
                        buf.extend_from_slice(&value.to_le_bytes());
                        index += 1;
                    }
                    b'l' | b'i' => {
                        let value: i32 = a[index].as_ref().try_into()?;
                        buf.extend_from_slice(&value.to_le_bytes());
                        index += 1;
                    }
                    b'S' => {
                        let value: u32 = a[index].as_ref().try_into()?;
                        let value = value as u16;
                        buf.extend_from_slice(&value.to_le_bytes());
                        index += 1;
                    }
                    b's' => {
                        let value: i32 = a[index].as_ref().try_into()?;
                        let value = value as i16;
                        buf.extend_from_slice(&value.to_le_bytes());
                        index += 1;
                    }
                    b'C' => {
                        let value: u32 = a[index].as_ref().try_into()?;
                        let value = value as u8;
                        buf.extend_from_slice(&value.to_le_bytes());
                        index += 1;
                    }
                    b'c' => {
                        let value: i32 = a[index].as_ref().try_into()?;
                        let value = value as i8;
                        buf.extend_from_slice(&value.to_le_bytes());
                        index += 1;
                    }
                    b' ' => {
                        // skip
                        continue;
                    }
                    _ => {
                        return Err(Error::RuntimeError("Unsupported format".to_string()));
                    }
                }
            }
        }
        _ => {
            return Err(Error::RuntimeError("Array#pack must be called on an Array".to_string()));
        }
    };
    let value = Rc::new(RObject::string_from_vec(buf));
    Ok(value)
}

#[test]
fn test_mrb_array_push_and_index() {
    use crate::yamrb::*;
    let mut vm = VM::empty();
    prelude::prelude(&mut vm);

    let array = Rc::new(RObject::array(vec![]));
    let args = vec![
        Rc::new(RObject::integer(1)),
        Rc::new(RObject::integer(2)),
        Rc::new(RObject::integer(3)),
    ];
    mrb_array_push(array.clone(), &args).expect("push failed");

    let answers = vec![
        1,
        2,
        3,
    ];

    for (i, expected) in answers.iter().enumerate() {
        let args = vec![Rc::new(RObject::integer(i as i64))];
        let value = mrb_array_get_index(array.clone(), &args).expect("getting index failed");
        let value: i64 = value.as_ref().try_into().expect("value is not integer");
        assert_eq!(value, *expected);
    }
}

#[test]
fn test_mrb_array_set_and_index() {
    use crate::yamrb::*;
    let mut vm = VM::empty();
    prelude::prelude(&mut vm);

    let array = Rc::new(RObject::array(vec![]));
    let args = vec![
        Rc::new(RObject::nil()),
        Rc::new(RObject::nil()),
        Rc::new(RObject::integer(0)),
    ];
    mrb_array_push(array.clone(), &args).expect("push failed");

    let upd_index = Rc::new(RObject::integer(2));
    let newval = Rc::new(RObject::integer(42));
    let args = vec![
        upd_index,
        newval,
    ];

    mrb_array_set_index(array.clone(), &args).expect("set index failed");

    let value = mrb_array_get_index(array.clone(), &args).expect("getting index failed");
    let value: i64 = value.as_ref().try_into().expect("value is not integer");
    assert_eq!(value, 42);
}

#[test]
fn test_mrb_array_pack() {
    use crate::yamrb::*;
    let mut vm = VM::empty();
    prelude::prelude(&mut vm);

    let array = Rc::new(RObject::array(vec![
        Rc::new(RObject::integer(1)),
        Rc::new(RObject::integer(2)),
        Rc::new(RObject::integer(3)),
        Rc::new(RObject::integer(4)),
    ]));
    vm.current_regs()[0].replace(array);
    let format = Rc::new(RObject::string(
        "c s l q".to_string(),
    ));
    let args = vec![format];
    let value = mrb_array_pack(&mut vm, &args).expect("pack failed");

    let expected: Vec<u8> = vec![
        0x01,
        0x02, 0x00,
        0x03, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let value: Vec<u8> = value.as_ref().try_into().expect("value is not string");
    for (i, v) in value.iter().enumerate() {
        assert_eq!(*v, expected[i]);
    }
}