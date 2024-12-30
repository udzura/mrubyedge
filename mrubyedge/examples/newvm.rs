extern crate mrubyedge;
use std::rc::Rc;

use mrubyedge::yamrb::*;
use mrubyedge::rite::insn::OpCode;
use mrubyedge::rite::insn::Fetched;

fn main() {
    let irep = vm::IREP {
        nlocals: 0,
        nregs: 0,
        rlen: 0,
        iren: 4,
        plen: 0,
        code: vec![
            op::Op { code: OpCode::NOP, operand: Fetched::Z, pos: 0, len: 1 },
            op::Op { code: OpCode::NOP, operand: Fetched::Z, pos: 0, len: 1 },
            op::Op { code: OpCode::NOP, operand: Fetched::Z, pos: 0, len: 1 },
            op::Op { code: OpCode::RETURN, operand: Fetched::B(0), pos: 0, len: 1 },
        ],
        syms: Vec::new(),
        pool: Vec::new(),
        reps: Vec::new(),
    };
    let mut vm = vm::VM::new_by_irep(irep);
    vm.regs[0].replace(Rc::new(value::RObject::nil()));
    let ret = vm.run().unwrap();
    dbg!(ret);
}