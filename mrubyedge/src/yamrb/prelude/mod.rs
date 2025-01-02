pub mod object;
pub mod class;

use std::rc::Rc;

use super::{value::*, vm::VM};

pub(crate) fn define_standard_class(vm: &mut VM, name: &'static str) -> Rc<RClass> {
    let class = Rc::new(
        RClass::new(name, Some(vm.object_class.clone())),
    );
    let object = Rc::new(RObject {
        tt: RType::Class,
        value: RValue::Class(class.clone()),
    });
    vm.consts.insert(name.to_string(), object);
    vm.builtin_class_table.insert(name, class.clone());
    class
}