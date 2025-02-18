use std::rc::Rc;

use crate::{yamrb::{helpers::mrb_define_cmethod, value::*, vm::VM}, Error};

pub(crate) fn initialize_exception(vm: &mut VM) {
    let exp_class: Rc<RClass> = vm.define_standard_class("Exception");
    // fill in ruby's standard exceptions:
    let std_exp_class: Rc<RClass> = vm.define_standard_class_under("StandardError", exp_class.clone());
    let _ = vm.define_standard_class_under("RuntimeError", std_exp_class.clone());
    let _ = vm.define_standard_class_under("NoMemoryError", exp_class.clone());
    let _ = vm.define_standard_class_under("ScriptError", exp_class.clone());
    let _ = vm.define_standard_class_under("LoadError", exp_class.clone());
    let _ = vm.define_standard_class_under("NotImplementedError", std_exp_class.clone());
    let _ = vm.define_standard_class_under("SyntaxError", exp_class.clone());
    let _ = vm.define_standard_class_under("SecurityError", std_exp_class.clone());
    let _ = vm.define_standard_class_under("SignalException", exp_class.clone());
    let _ = vm.define_standard_class_under("Interrupt", exp_class.clone());
    let _ = vm.define_standard_class_under("SystemExit", exp_class.clone());
    let _ = vm.define_standard_class_under("SystemStackError", exp_class.clone());
    let _ = vm.define_standard_class_under("SystemCallError", std_exp_class.clone());
    let _ = vm.define_standard_class_under("NoMethodEArror", std_exp_class.clone());

    mrb_define_cmethod(vm, exp_class, "message", Box::new(mrb_exception_message));
}

pub fn mrb_exception_message(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let exp = vm.getself();
    match &exp.value {
        RValue::Exception(e) => {
            let message = e.as_ref().message.clone();
            Ok(RObject::string(message).to_refcount_assigned())
        },
        _ => {
            Err(Error::RuntimeError("Exception#message must be called on an Exception".to_string()))
        }
    }
}