use crate::yamrb::vm::VM;

pub(crate) fn initialize_integer(vm: &mut VM) {
    let _integer_class = vm.define_standard_class("Integer");
}
