use std::rc::Rc;

use crate::{yamrb::{helpers::{mrb_define_cmethod, mrb_funcall}, value::*, vm::VM}, Error};

pub(crate) fn initialize_object(vm: &mut VM) {
    let object_class = vm.object_class.clone();
    let klass: RObject = object_class.clone().into();
    vm.consts.insert("Object".to_string(), klass.to_refcount_assigned());
    vm.builtin_class_table.insert("Object", object_class.clone());

    #[cfg(feature = "wasi")]
    {
        mrb_define_cmethod(vm, object_class.clone(), "puts", Box::new(mrb_kernel_puts));
        mrb_define_cmethod(vm, object_class.clone(), "p", Box::new(mrb_kernel_p));
        mrb_define_cmethod(vm, object_class.clone(), "debug", Box::new(mrb_kernel_debug));
    }

    mrb_define_cmethod(vm, object_class.clone(), "initialize", Box::new(mrb_object_initialize));
    mrb_define_cmethod(vm, object_class.clone(), "==", Box::new(mrb_object_double_eq));
    mrb_define_cmethod(vm, object_class.clone(), "===", Box::new(mrb_object_triple_eq));
    mrb_define_cmethod(vm, object_class.clone(), "object_id", Box::new(mrb_object_object_id));
    mrb_define_cmethod(vm, object_class.clone(), "__id__", Box::new(mrb_object_object_id));
    mrb_define_cmethod(vm, object_class.clone(), "to_s", Box::new(mrb_object_to_s));
    mrb_define_cmethod(vm, object_class.clone(), "inspect", Box::new(mrb_object_to_s));

    // define global consts:
    vm.consts.insert("RUBY_VERSION".to_string(), Rc::new(RObject::string(crate::yamrb::vm::VERSION.to_string())));
    vm.consts.insert("MRUBY_VERSION".to_string(), Rc::new(RObject::string(crate::yamrb::vm::VERSION.to_string())));
    vm.consts.insert("MRUBY_EDGE_VERSION".to_string(), Rc::new(RObject::string(crate::yamrb::vm::VERSION.to_string())));
    vm.consts.insert("RUBY_ENGINE".to_string(), Rc::new(RObject::string(crate::yamrb::vm::ENGINE.to_string())));
}

#[cfg(feature = "wasi")]
pub fn mrb_kernel_puts(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let msg = args[0].clone();
    match &msg.value {
        RValue::String(s) => {
            println!("{}", String::from_utf8_lossy(&s.borrow()));
        }
        RValue::Integer(i) => {
            println!("{}", i);
        }
        _ => {
            let inspect = mrb_funcall(vm, Some(msg), "to_s", &[])?;
            let inspect: String = inspect.as_ref().try_into()?;
            println!("{}", inspect);
        }
    }
    Ok(Rc::new(RObject::nil()))
}

#[cfg(feature = "wasi")]
pub fn mrb_kernel_p(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let msg = args[0].clone();
    let inspect = mrb_funcall(vm, Some(msg), "inspect", &[])?;
    let inspect: String = inspect.as_ref().try_into()?;
    println!("{}", inspect);
    Ok(Rc::new(RObject::nil()))
}

#[cfg(feature = "wasi")]
pub fn mrb_kernel_debug(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    for (i, obj) in args.iter().enumerate() {
        dbg!(i, obj.clone());
    }
    Ok(Rc::new(RObject::nil()))
}

pub fn mrb_object_is_equal(_vm: &mut VM, lhs: Rc<RObject>, rhs: Rc<RObject>) -> Rc<RObject> {
    RObject::boolean(lhs.as_eq_value() == rhs.as_eq_value()).to_refcount_assigned()
}

pub fn mrb_object_is_not_equal(_vm: &mut VM, lhs: Rc<RObject>, rhs: Rc<RObject>) -> Rc<RObject> {
    RObject::boolean(lhs.as_eq_value() != rhs.as_eq_value()).to_refcount_assigned()
}

pub fn mrb_object_double_eq(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let lhs = vm.getself();
    let rhs = args[0].clone();
    Ok(mrb_object_is_equal(vm, lhs, rhs))
}

pub fn mrb_object_triple_eq(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let lhs = vm.getself();
    let rhs = args[0].clone();

    match (&lhs.value, &rhs.value) {
        (RValue::Integer(i1), RValue::Integer(i2)) => {
            Ok(Rc::new(RObject::boolean(*i1 == *i2)))
        }
        (RValue::Float(f1), RValue::Float(f2)) => {
            Ok(Rc::new(RObject::boolean(*f1 == *f2)))
        }
        (RValue::Symbol(sym1), RValue::Symbol(sym2)) => {
            Ok(Rc::new(RObject::boolean(sym1 == sym2)))
        }
        (RValue::String(s1), RValue::String(s2)) => {
            Ok(Rc::new(RObject::boolean(s1 == s2)))
        }
        (RValue::Class(c1), _) => {
            match &lhs.value {
                RValue::Class(c2) => {
                    Ok(Rc::new(RObject::boolean(c1.sym_id == c2.sym_id)))
                }
                _ => {
                    let c2 = lhs.get_class(vm);
                    Ok(Rc::new(RObject::boolean(c1.sym_id == c2.sym_id)))
                }
            }
        }
        (RValue::Range(_s, _e, _v), _) => {
            let arg = vec![rhs];
            mrb_funcall(vm, Some(lhs), "include?", &arg)
        }
        // TODO: Implement object id for generic instance
        _ => {
            Ok(Rc::new(RObject::boolean(false)))
        }
    }
}

pub fn mrb_object_object_id(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    // Abstract method; do nothing
    let x = vm.getself().as_ref().object_id.get();
    // ref: https://stackoverflow.com/questions/74491204/how-do-i-represent-an-i64-in-the-u64-domain
    let to_i64 = ((x as i64) ^ (1 << 63)) & (1 << 63) | (x & (u64::MAX >> 1)) as i64;
    Ok(Rc::new(RObject::integer(to_i64)))
}

pub fn mrb_object_to_s(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let obj = vm.getself();
    Ok(Rc::new(RObject::string(format!("{:?}", obj))))
}

pub fn mrb_object_initialize(_vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    // Abstract method; do nothing
    Ok(Rc::new(RObject::nil()))
}

#[test]
fn test_mrb_object_is_equal() {
    let mut vm = VM::empty();

    let lhs = RObject::integer(1).to_refcount_assigned();
    let rhs = RObject::integer(1).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::integer(1).to_refcount_assigned();
    let rhs = RObject::integer(3).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);

    let lhs = RObject::string("mruby/edge is Ruby".into()).to_refcount_assigned();
    let rhs = RObject::string("mruby/edge is Ruby".into()).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::string("mruby/edge is Ruby".into()).to_refcount_assigned();
    let rhs = RObject::string("mruby/edge is not Ruby".into()).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);

    let lhs = RObject::symbol("some".into()).to_refcount_assigned();
    let rhs = RObject::symbol("some".into()).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::symbol("some".into()).to_refcount_assigned();
    let rhs = RObject::symbol("other".into()).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);

    let lhs = RObject::boolean(true).to_refcount_assigned();
    let rhs = RObject::boolean(true).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::boolean(false).to_refcount_assigned();
    let rhs = RObject::boolean(false).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::boolean(true).to_refcount_assigned();
    let rhs = RObject::boolean(false).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);

    let lhs = RObject::float(0.1).to_refcount_assigned();
    let rhs = RObject::float(0.1).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::float(0.2).to_refcount_assigned();
    let rhs = RObject::float(0.1).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);

    let lhs = RObject::nil().to_refcount_assigned();
    let rhs = RObject::nil().to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::integer(100).to_refcount_assigned();
    let rhs = RObject::nil().to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);

    let lhs = RObject::integer(100).to_refcount_assigned();
    let rhs = RObject::nil().to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);
}

#[test]
fn test_mrb_object_is_equal_range() {
    let mut vm = VM::empty();

    let s = RObject::integer(1).to_refcount_assigned();
    let e = RObject::integer(10).to_refcount_assigned();
    let lhs = RObject::range(s.clone(), e.clone(), true).to_refcount_assigned();
    let rhs = RObject::range(s.clone(), e.clone(), true).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let s = RObject::integer(1).to_refcount_assigned();
    let e = RObject::integer(10).to_refcount_assigned();
    let lhs = RObject::range(s.clone(), e.clone(), true).to_refcount_assigned();
    let rhs = RObject::range(s.clone(), e.clone(), false).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);

    let s = RObject::integer(1).to_refcount_assigned();
    let e = RObject::integer(10).to_refcount_assigned();
    let e2 = RObject::integer(11).to_refcount_assigned();
    let lhs = RObject::range(s.clone(), e.clone(), true).to_refcount_assigned();
    let rhs = RObject::range(s.clone(), e2.clone(), true).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);

    let s = RObject::string("a".into()).to_refcount_assigned();
    let e = RObject::string("z".into()).to_refcount_assigned();
    let lhs = RObject::range(s.clone(), e.clone(), true).to_refcount_assigned();
    let rhs = RObject::range(s.clone(), e.clone(), true).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let s = RObject::string("a".into()).to_refcount_assigned();
    let e = RObject::string("z".into()).to_refcount_assigned();
    let e2 = RObject::string("A".into()).to_refcount_assigned();
    let lhs = RObject::range(s.clone(), e.clone(), true).to_refcount_assigned();
    let rhs = RObject::range(s.clone(), e2.clone(), true).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);
}

#[test]
fn test_mrb_object_is_equal_array() {
    let mut vm = VM::empty();

    let lhs = RObject::array(vec![]).to_refcount_assigned();
    let rhs = RObject::array(vec![]).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = vec![
        RObject::integer(1).to_refcount_assigned(),
        RObject::integer(2).to_refcount_assigned(),
        RObject::integer(3).to_refcount_assigned(),
    ];
    let rhs = vec![
        RObject::integer(1).to_refcount_assigned(),
        RObject::integer(2).to_refcount_assigned(),
        RObject::integer(3).to_refcount_assigned(),
    ];
    let lhs = RObject::array(lhs).to_refcount_assigned();
    let rhs = RObject::array(rhs).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = vec![
        RObject::integer(1).to_refcount_assigned(),
        RObject::integer(2).to_refcount_assigned(),
        RObject::integer(3).to_refcount_assigned(),
    ];
    let rhs = vec![
        RObject::integer(1).to_refcount_assigned(),
        RObject::integer(2).to_refcount_assigned(),
        RObject::integer(4).to_refcount_assigned(),
    ];
    let lhs = RObject::array(lhs).to_refcount_assigned();
    let rhs = RObject::array(rhs).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);
}

#[test]
fn test_mrb_object_is_equal_hash() {
    use std::collections::HashMap;
    use crate::yamrb::prelude::hash::*;

    let mut vm = VM::empty();

    let lhs = RObject::hash(HashMap::new()).to_refcount_assigned();
    let rhs = RObject::hash(HashMap::new()).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::hash(HashMap::new()).to_refcount_assigned();
    mrb_hash_set_index(lhs.clone(), RObject::symbol("key1".into()).to_refcount_assigned(), RObject::integer(1).to_refcount_assigned()).expect("set index failed");
    mrb_hash_set_index(lhs.clone(), RObject::symbol("key2".into()).to_refcount_assigned(), RObject::integer(2).to_refcount_assigned()).expect("set index failed");

    let rhs = RObject::hash(HashMap::new()).to_refcount_assigned();
    mrb_hash_set_index(rhs.clone(), RObject::symbol("key2".into()).to_refcount_assigned(), RObject::integer(2).to_refcount_assigned()).expect("set index failed");
    mrb_hash_set_index(rhs.clone(), RObject::symbol("key1".into()).to_refcount_assigned(), RObject::integer(1).to_refcount_assigned()).expect("set index failed");

    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::hash(HashMap::new()).to_refcount_assigned();
    mrb_hash_set_index(lhs.clone(), RObject::symbol("key1".into()).to_refcount_assigned(), RObject::integer(1).to_refcount_assigned()).expect("set index failed");
    mrb_hash_set_index(lhs.clone(), RObject::symbol("key2".into()).to_refcount_assigned(), RObject::integer(2).to_refcount_assigned()).expect("set index failed");

    let rhs = RObject::hash(HashMap::new()).to_refcount_assigned();
    mrb_hash_set_index(rhs.clone(), RObject::symbol("key2".into()).to_refcount_assigned(), RObject::integer(2).to_refcount_assigned()).expect("set index failed");
    mrb_hash_set_index(rhs.clone(), RObject::symbol("key1".into()).to_refcount_assigned(), RObject::integer(3).to_refcount_assigned()).expect("set index failed");

    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);

    let lhs = RObject::hash(HashMap::new()).to_refcount_assigned();
    mrb_hash_set_index(lhs.clone(), RObject::symbol("key1".into()).to_refcount_assigned(), RObject::integer(1).to_refcount_assigned()).expect("set index failed");
    mrb_hash_set_index(lhs.clone(), RObject::symbol("key2".into()).to_refcount_assigned(), RObject::integer(2).to_refcount_assigned()).expect("set index failed");

    let rhs = RObject::hash(HashMap::new()).to_refcount_assigned();
    mrb_hash_set_index(rhs.clone(), RObject::symbol("key2".into()).to_refcount_assigned(), RObject::integer(2).to_refcount_assigned()).expect("set index failed");
    mrb_hash_set_index(rhs.clone(), RObject::symbol("key1-b".into()).to_refcount_assigned(), RObject::integer(1).to_refcount_assigned()).expect("set index failed");

    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);
}

#[test]
fn test_mrb_object_is_equal_klass() {
    let mut vm = VM::empty();

    let lhs: RObject = vm.get_class_by_name("String").into();
    let rhs: RObject = RObject::string("String".into()).get_class(&mut vm).into();
    let lhs = lhs.to_refcount_assigned();
    let rhs = rhs.to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs: RObject = RObject::integer(5471).get_class(&mut vm).into();
    let rhs: RObject = RObject::string("String".into()).get_class(&mut vm).into();
    let lhs = lhs.to_refcount_assigned();
    let rhs = rhs.to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);
}

#[test]
fn test_mrb_object_is_equal_instance() {
    let mut vm = VM::empty();

    let lhs = RObject::instance(vm.object_class.clone()).to_refcount_assigned();
    let rhs = lhs.clone();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(ret);

    let lhs = RObject::instance(vm.object_class.clone()).to_refcount_assigned();
    let rhs = RObject::instance(vm.object_class.clone()).to_refcount_assigned();
    let ret: bool = mrb_object_is_equal(&mut vm, lhs, rhs).as_ref().try_into().expect("must return bool");
    assert!(!ret);
}