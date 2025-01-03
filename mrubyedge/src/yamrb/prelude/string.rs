use crate::yamrb::vm::VM;

pub(crate) fn initialize_string(vm: &mut VM) {
    let _string_class = vm.define_standard_class("String");
}
