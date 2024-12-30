use crate::rite::insn::Fetched;

use super::vm::VM;

pub(crate) fn op_nop(vm: &VM, operand: &Fetched) {
    // NOOP
    dbg!("nop");
}

pub(crate) fn op_return(vm: &VM, operand: &Fetched) {
    // NOOP
    vm.flag_preemption.set(true);
}