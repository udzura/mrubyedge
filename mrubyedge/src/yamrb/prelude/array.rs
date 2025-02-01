use std::rc::Rc;

use crate::{yamrb::{helpers::mrb_define_cmethod, value::{RObject, RValue}, vm::VM}, Error};

pub(crate) fn initialize_array(vm: &mut VM) {
    let array_class = vm.define_standard_class("Array");

    mrb_define_cmethod(vm, array_class.clone(), "push", Box::new(mrb_array_push_self));
    mrb_define_cmethod(vm, array_class.clone(), "[]", Box::new(mrb_array_get_index_self));
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