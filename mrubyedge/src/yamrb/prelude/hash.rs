use std::rc::Rc;

use crate::{yamrb::{helpers::{mrb_call_block, mrb_define_cmethod}, value::{RObject, RValue}, vm::VM}, Error};

pub(crate) fn initialize_hash(vm: &mut VM) {
    let hash_class = vm.define_standard_class("Hash");

    mrb_define_cmethod(vm, hash_class.clone(), "[]", Box::new(mrb_hash_get_index_self));
    mrb_define_cmethod(vm, hash_class.clone(), "[]=", Box::new(mrb_hash_set_index_self));
    mrb_define_cmethod(vm, hash_class.clone(), "each", Box::new(mrb_hash_each));
    mrb_define_cmethod(vm, hash_class.clone(), "size", Box::new(mrb_hash_size));
    mrb_define_cmethod(vm, hash_class.clone(), "length", Box::new(mrb_hash_size));
}

fn mrb_hash_get_index_self(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself()?;
    mrb_hash_get_index(this, args[0].clone())
}

pub fn mrb_hash_get_index(this: Rc<RObject>, key: Rc<RObject>) -> Result<Rc<RObject>, Error> {
    let hash = match &this.value {
        RValue::Hash(a) => a.clone(),
        _ => {
            return Err(Error::RuntimeError("Hash#[] must called on a hash".to_string()));
        }
    };
    let hash = hash.borrow();
    let key  = key.as_ref().as_hash_key()?;
    match hash.get(&key).clone() {
        Some((_, value)) => Ok(value.clone()),
        None => Ok(Rc::new(RObject::nil())),
    }
}

fn mrb_hash_set_index_self(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself()?;
    let key = args[0].clone();
    let value = args[1].clone();
    mrb_hash_set_index(this, key, value)
}

pub fn mrb_hash_set_index(this: Rc<RObject>, key: Rc<RObject>, value: Rc<RObject>) -> Result<Rc<RObject>, Error> {
    let hash = match &this.value {
        RValue::Hash(a) => a,
        _ => {
            return Err(Error::RuntimeError("Hash#[] must called on a hash".to_string()));
        }
    };
    let mut hash = hash.borrow_mut();
    let hashed  = key.as_hash_key()?;
    hash.insert(hashed, (key.clone(), value.clone()));
    Ok(value.clone())
}

fn mrb_hash_each(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself()?;
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
fn test_hashing() {
    let vec1 = RObject::string("key".to_string());
    let vec2 = RObject::string("key".to_string()).clone();
    assert_eq!(vec1.as_hash_key(), vec2.as_hash_key());
}

#[test]
fn test_mrb_hash_set_and_index() {
    use std::collections::HashMap;
    use crate::yamrb::*;
    let mut vm = VM::empty();
    prelude::prelude(&mut vm);

    let hash = Rc::new(RObject::hash(HashMap::new()));
    let keys = vec![
        Rc::new(RObject::string("key".to_string())),
        Rc::new(RObject::integer(1234)),
        Rc::new(RObject::symbol("key2".into())),
    ];
    let values = vec![
        Rc::new(RObject::integer(1)),
        Rc::new(RObject::integer(2)),
        Rc::new(RObject::integer(42)),
    ];

    for (i, key) in keys.iter().enumerate() {
        let value = &values[i];
        mrb_hash_set_index(hash.clone(), key.clone(), value.clone()).expect("set index failed");
    }

    for (i, key) in keys.iter().enumerate() {
        let value = mrb_hash_get_index(hash.clone(), key.clone()).expect("getting index failed");
        let value: i64 = value.as_ref().try_into().expect("value is not integer");
        let expected: i64 = values[i].as_ref().try_into().expect("expected is not integer");
        assert_eq!(value, expected);
    }
}

#[test]
fn test_mrb_hash_set_and_index_not_found() {
    use std::collections::HashMap;
    use crate::yamrb::*;
    let mut vm = VM::empty();
    prelude::prelude(&mut vm);

    let hash = Rc::new(RObject::hash(HashMap::new()));
    let key = Rc::new(RObject::string("key".to_string()));
    let value = Rc::new(RObject::integer(42));

    mrb_hash_set_index(hash.clone(), key.clone(), value.clone()).expect("set index failed");

    let key = Rc::new(RObject::string("key2".to_string()));
    let value = mrb_hash_get_index(hash.clone(), key.clone()).expect("getting index failed");
    let value = value.as_ref();
    assert!(value.is_nil());
}

fn mrb_hash_size(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself()?;
    let hash = match &this.value {
        RValue::Hash(a) => a,
        _ => {
            return Err(Error::RuntimeError("Hash#size must be called on a hash".to_string()));
        }
    };
    let hash = hash.borrow();
    Ok(Rc::new(RObject::integer(hash.len() as i64)))
}

#[test]
fn test_mrb_hash_size() {
    use std::collections::HashMap;
    let mut vm = VM::empty();

    let hash = Rc::new(RObject::hash(HashMap::new()));
    let key = Rc::new(RObject::string("key".to_string()));
    let value = Rc::new(RObject::integer(42));
    vm.current_regs()[0].replace(hash.clone());

    let size = mrb_hash_size(&mut vm, &[]).expect("getting size failed");
    let size: i64 = size.as_ref().try_into().expect("size is not integer");
    assert_eq!(size, 0);

    mrb_hash_set_index(hash.clone(), key.clone(), value.clone()).expect("set index failed");

    let size = mrb_hash_size(&mut vm, &[]).expect("getting size failed");
    let size: i64 = size.as_ref().try_into().expect("size is not integer");
    assert_eq!(size, 1);
}