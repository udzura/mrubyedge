// use std::rc::Rc;

use std::rc::Rc;

use crate::rite::insn::{Fetched, OpCode};

use super::{value::{RObject, RValue}, vm::VM};

pub(crate) fn consume_expr(vm: &mut VM, code: OpCode, operand: &Fetched) {
    use crate::rite::insn::OpCode::*;
    match code {
        NOP => {
            op_nop(vm, &operand);
        }
        MOVE => {
            op_move(vm, &operand);
        }
        LOADI => {
            op_loadi(vm, &operand);
        }
        LOADI__1 => {
            op_loadi_n(vm, -1, &operand);
        }
        LOADI_1 => {
            op_loadi_n(vm, 1, &operand);
        }
        LOADI_2 => {
            op_loadi_n(vm, 2, &operand);
        }
        LOADI_3 => {
            op_loadi_n(vm, 3, &operand);
        }
        LOADI_4 => {
            op_loadi_n(vm, 4, &operand);
        }
        LOADI_5 => {
            op_loadi_n(vm, 5, &operand);
        }
        LOADI_6 => {
            op_loadi_n(vm, 6, &operand);
        }
        LOADI_7 => {
            op_loadi_n(vm, 7, &operand);
        }
        ADD => {
            op_add(vm, &operand);
        }
        RETURN => {
            op_return(vm, &operand);
        }
        STOP => {
            op_stop(vm, &operand);
        }
        _ => { unimplemented!("Not supported yet")}
    }
}

pub(crate) fn op_nop(_vm: &mut VM, _operand: &Fetched) {
    // NOOP
    dbg!("nop");
}

pub(crate) fn op_loadi_n(vm: &mut VM, n: i32, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let val = RObject::integer(n as i64);
    vm.regs[a].replace(Rc::new(val));
}

pub(crate) fn op_loadi(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = RObject::integer(b as i64);
    vm.regs[a as usize].replace(Rc::new(val));
}

pub(crate) fn op_move(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = vm.regs[b as usize].take();
    vm.regs[a as usize].replace(val.unwrap());
}

pub(crate) fn op_return(vm: &mut VM, operand: &Fetched) {
    // TODO: handle callinfo stack...
    let a = operand.as_b().unwrap() as usize;
    let retval = vm.regs[a].take();
    vm.regs[0].replace(retval.unwrap());
}

pub(crate) fn op_add(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.regs[a].take().unwrap();
    let val2 = vm.regs[b].take().unwrap();
    let result = match (&val1.value, &val2.value) {
        (RValue::Integer(n1), RValue::Integer(n2)) => {
            RObject::integer(n1 + n2)
        }
        _ => {
            unreachable!("add supports only integer")
        }
    };
    vm.regs[a].replace(Rc::new(result));
}

pub(crate) fn op_stop(vm: &mut VM, _operand: &Fetched) {
    vm.flag_preemption.set(true);
}