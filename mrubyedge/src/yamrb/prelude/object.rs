use std::rc::Rc;

use crate::{yamrb::{helpers::{mrb_define_cmethod, mrb_funcall}, value::*, vm::VM}, Error};

pub(crate) fn initialize_object(vm: &mut VM) {
    let object_class = vm.object_class.clone();
    vm.consts.insert("Object".to_string(), Rc::new(RObject {
        tt: RType::Class,
        value: RValue::Class(object_class.clone()),
    }));
    vm.builtin_class_table.insert("Object", object_class.clone());

    #[cfg(feature = "wasi")]
    {
        mrb_define_cmethod(vm, object_class.clone(), "puts", Box::new(mrb_kernel_puts));
        mrb_define_cmethod(vm, object_class.clone(), "debug", Box::new(mrb_kernel_debug));
    }

    mrb_define_cmethod(vm, object_class.clone(), "initialize", Box::new(mrb_object_initialize));
    mrb_define_cmethod(vm, object_class.clone(), "===", Box::new(mrb_object_triple_eq));
}

#[cfg(feature = "wasi")]
pub fn mrb_kernel_puts(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let msg = args[0].clone();
    match &msg.value {
        RValue::String(s) => {
            println!("{}", s.borrow());
        }
        RValue::Integer(i) => {
            println!("{}", i);
        }
        _ => {
            dbg!(&msg);
            return Err(Error::RuntimeError("puts only accept string".to_string()));
        }
    }
    Ok(Rc::new(RObject::nil()))
}

#[cfg(feature = "wasi")]
pub fn mrb_kernel_debug(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    for (i, obj) in args.iter().enumerate() {
        dbg!(i, obj.clone());
    }
    Ok(Rc::new(RObject::nil()))
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

pub fn mrb_object_initialize(_vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    // Abstract method; do nothing
    Ok(Rc::new(RObject::nil()))
}