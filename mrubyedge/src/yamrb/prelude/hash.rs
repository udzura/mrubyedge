use std::{collections::HashMap, rc::Rc};

use crate::{yamrb::{helpers::{mrb_call_block, mrb_define_cmethod}, value::{RObject, RValue}, vm::VM}, Error};

pub(crate) fn initialize_hash(vm: &mut VM) {
    let array_class = vm.define_standard_class("Hash");

    mrb_define_cmethod(vm, array_class.clone(), "[]", Box::new(mrb_hash_get_index_self));
    mrb_define_cmethod(vm, array_class.clone(), "[]=", Box::new(mrb_hash_set_index_self));
    mrb_define_cmethod(vm, array_class.clone(), "each", Box::new(mrb_hash_each));
}

fn mrb_hash_get_index_self(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    mrb_hash_get_index(this, args)
}

pub fn mrb_hash_get_index(this: Rc<RObject>, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let hash = match &this.value {
        RValue::Hash(a) => a.clone(),
        _ => {
            return Err(Error::RuntimeError("Hash#[] must called on a hash".to_string()));
        }
    };
    let hash = hash.borrow();
    let key  = args[0].as_ref().as_hash_key()?;
    match hash.get(&key).clone() {
        Some((_, value)) => Ok(value.clone()),
        None => Ok(Rc::new(RObject::nil())),
    }
}

fn mrb_hash_set_index_self(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    mrb_hash_set_index(this, args)
}

pub fn mrb_hash_set_index(this: Rc<RObject>, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let hash = match &this.value {
        RValue::Hash(a) => a.clone(),
        _ => {
            return Err(Error::RuntimeError("Hash#[] must called on a hash".to_string()));
        }
    };
    let mut hash = hash.borrow_mut();
    let key = args[0].clone();
    let hashed  = key.as_hash_key()?;
    let value = &args[1];
    hash.insert(hashed, (key, value.clone()));
    dbg!(&hash);
    Ok(value.clone())
}

fn mrb_hash_each(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself();
    let block = &args[0];
    match &this.value {
        RValue::Hash(hash) => {
            let hash = hash.borrow();
            for (_, (key, value)) in hash.iter() {
                let args = vec![key.clone(), value.clone()];
                mrb_call_block(vm, block.clone(), None, &args)?;
            }
        }
        _ => {
            return Err(Error::RuntimeError("Hash#each must be called on a hash".to_string()));
        }
    };
    Ok(this.clone())
}

#[test]
fn test_mrb_hash_set_and_index() {
    use crate::yamrb::*;
    let mut vm = VM::empty();
    prelude::prelude(&mut vm);

    let hash = Rc::new(RObject::hash(HashMap::new()));
    let key = Rc::new(RObject::string("key".to_string()));
    let value = Rc::new(RObject::integer(42));
    let args = vec![key.clone(), value.clone()];

    mrb_hash_set_index(hash.clone(), &args).expect("set index failed");

    let get_args = vec![key.clone()];
    let value = mrb_hash_get_index(hash.clone(), &get_args).expect("getting index failed");
    dbg!(&value);
    let value: i64 = value.as_ref().try_into().expect("value is not integer");
    assert_eq!(value, 42);
}
