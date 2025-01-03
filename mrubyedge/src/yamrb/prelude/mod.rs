use super::vm::VM;

pub mod object;
pub mod class;
pub mod integer;
pub mod string;
pub mod range;

pub fn prelude(vm: &mut VM) {
    object::initialize_object(vm);
    class::initialize_class(vm);
    integer::initialize_integer(vm);
    string::initialize_string(vm);
    range::initialize_range(vm);
}
