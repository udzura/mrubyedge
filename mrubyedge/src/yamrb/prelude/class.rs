use std::rc::Rc;

use crate::{yamrb::{helpers::{mrb_define_cmethod, mrb_funcall}, value::*, vm::VM}, Error};

use super::shared_memory::mrb_shared_memory_new;

pub(crate) fn initialize_class(vm: &mut VM) {
    let class_class = vm.define_standard_class("Class");

    mrb_define_cmethod(vm, class_class.clone(), "new", Box::new(mrb_class_new));
    mrb_define_cmethod(vm, class_class.clone(), "attr_reader", Box::new(mrb_class_attr_reader));
    mrb_define_cmethod(vm, class_class.clone(), "attr_writer", Box::new(mrb_class_attr_writer));
    mrb_define_cmethod(vm, class_class.clone(), "attr_accessor", Box::new(mrb_class_attr_acceccor));
    mrb_define_cmethod(vm, class_class.clone(), "attr", Box::new(mrb_class_attr_acceccor));
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

fn mrb_class_attr_reader(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let class_ = vm.getself();
    let class = match &class_.value {
        RValue::Class(c) => c.clone(),
        _ => {
            return Err(Error::RuntimeError("Class#attr_reader must be called from class".to_string()));
        }
    };
    for arg in args.iter() {
        match arg.value {
            RValue::Symbol(ref sym) => {
                let sym_id: &'static str = sym.name.clone().leak();
                let method = move |vm: &mut VM, _args: &[Rc<RObject>]| {
                    let this = vm.getself();
                    let key = format!("@{}", sym_id);
                    let value = match &this.value {
                        RValue::Instance(i) => {
                            i.ivar.borrow().get(&key).unwrap().clone()
                        },
                        _ => {
                            return Err(Error::RuntimeError("attr_reader defined method must be called from instance".to_string()));
                        }
                    };
                    Ok(value)
                };
                mrb_define_cmethod(vm, class.clone(), &sym_id, Box::new(method));
            }
            _ => {
                return Err(Error::RuntimeError("Class#attr_reader must be called with symbols".to_string()));
            }
        }
    }
    Ok(Rc::new(RObject::nil()))
}

fn mrb_class_attr_writer(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let class_ = vm.getself();
    let class = match &class_.value {
        RValue::Class(c) => c.clone(),
        _ => {
            return Err(Error::RuntimeError("Class#attr_reader must be called from class".to_string()));
        }
    };
    for arg in args.iter() {
        match arg.value {
            RValue::Symbol(ref sym) => {
                let sym_id: &'static str = sym.name.clone().leak();
                let method = move |vm: &mut VM, args: &[Rc<RObject>]| {
                    let this = vm.getself();
                    let key = format!("@{}", sym_id);
                    let value = args[0].clone();
                    match &this.value {
                        RValue::Instance(i) => {
                            i.ivar.borrow_mut().insert(key, value.clone());
                        },
                        _ => {
                            return Err(Error::RuntimeError("attr_reader defined method must be called from instance".to_string()));
                        }
                    };
                    Ok(value)
                };
                let sym_id = format!("{}=", sym_id);
                mrb_define_cmethod(vm, class.clone(), &sym_id, Box::new(method));
            }
            _ => {
                return Err(Error::RuntimeError("Class#attr_reader must be called with symbols".to_string()));
            }
        }
    }
    Ok(Rc::new(RObject::nil()))
}

fn mrb_class_attr_acceccor(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    mrb_class_attr_reader(vm, args)?;
    mrb_class_attr_writer(vm, args)
}

#[test]
fn test_class_attr_accessor() {
    use crate::yamrb::helpers::*;

    let mut vm = VM::empty();
    let class = vm.define_class("Test", None);
    let args = vec![RObject::symbol("foo".into()).to_refcount_assigned()];
    vm.current_regs()[0].replace(RObject::class(class.clone()).to_refcount_assigned());
    mrb_class_attr_acceccor(&mut vm, &args).expect("mrb_class_attr_acceccor failed");

    let instance = RObject::instance(class).to_refcount_assigned();

    let args = vec![RObject::integer(557188).to_refcount_assigned()];
    mrb_funcall(&mut vm, Some(instance.clone()), "foo=", &args).expect("call obj.foo = failed");

    let ret = mrb_funcall(&mut vm, Some(instance.clone()), "foo", &[]).expect("call obj.foo failed");
    let ret: i64 = ret.as_ref().try_into().expect("obj.foo must be integer");
    assert_eq!(ret, 557188);
}