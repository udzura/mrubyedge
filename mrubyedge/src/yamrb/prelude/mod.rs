use super::vm::VM;

pub mod object;
pub mod class;
pub mod integer;
pub mod string;
pub mod array;
pub mod hash;
pub mod range;
pub mod shared_memory;

pub fn prelude(vm: &mut VM) {
    object::initialize_object(vm);
    class::initialize_class(vm);
    integer::initialize_integer(vm);
    string::initialize_string(vm);
    array::initialize_array(vm);
    hash::initialize_hash(vm);
    range::initialize_range(vm);
    shared_memory::initialize_shared_memory(vm);
}
