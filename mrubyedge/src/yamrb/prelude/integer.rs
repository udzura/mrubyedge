use std::rc::Rc;

use crate::yamrb::helpers::mrb_define_cmethod;
use crate::Error;

use crate::yamrb::{helpers::mrb_call_block, value::RObject, vm::VM};

pub(crate) fn initialize_integer(vm: &mut VM) {
    let integer_class = vm.define_standard_class("Integer");

    mrb_define_cmethod(vm, integer_class.clone(), "%", Box::new(mrb_integer_mod));
    mrb_define_cmethod(vm, integer_class.clone(), "times", Box::new(mrb_integer_times));
}

fn mrb_integer_times(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this: i64 = vm.getself()?.as_ref().try_into()?;
    for i in 0..this {
        let block = args[0].clone();
        let args = vec![Rc::new(RObject::integer(i))];
        mrb_call_block(vm, block, None, &args)?;
    }
    vm.getself()
}

fn mrb_integer_mod(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let lhs: i64 = vm.getself()?.as_ref().try_into()?;
    let rhs: i64 = args[0].as_ref().try_into()?;

    Ok(Rc::new(RObject::integer(lhs % rhs)))
}
