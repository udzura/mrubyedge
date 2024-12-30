// use std::rc::Rc;

use crate::rite::insn::Fetched;

use super::vm::VM;

pub(crate) fn op_nop(_vm: &mut VM, _operand: &Fetched) {
    // NOOP
    dbg!("nop");
}

pub(crate) fn op_return(vm: &mut VM, operand: &Fetched) {
    // TODO: handle callinfo stack...
    let a = operand.as_b().unwrap() as usize;
    let retval = vm.regs[a].take();
    vm.regs[0].replace(retval.unwrap());

    vm.flag_preemption.set(true);
}