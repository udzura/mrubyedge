extern crate mrubyedge;
use std::rc::Rc;

use mrubyedge::yamrb::*;
use mrubyedge::rite::insn::OpCode;
use mrubyedge::rite::insn::Fetched;

//
// This is a simple example of a new VM from realworld mrbc result.
// file: examples/add.rb
//     1 000 LOADI_1	R1	(1)		; R1:x
//     2 002 LOADI_2	R2	(2)		; R2:y
//     3 004 MOVE		R4	R1		; R1:x
//     3 007 MOVE		R5	R2		; R2:y
//     3 010 ADD		R4	R5
//     3 012 SSEND		R3	:puts	n=1 ; not yet supported, so use OP_MOVE instead
//     3 016 RETURN	R3		
//     3 018 STOP
//
fn main() {
    let irep = vm::IREP {
        nlocals: 0,
        nregs: 0,
        rlen: 0,
        iren: 8,
        plen: 0,
        code: vec![
            op::Op { code: OpCode::LOADI_1, operand: Fetched::B(1), pos: 0, len: 2 },
            op::Op { code: OpCode::LOADI_2, operand: Fetched::B(2), pos: 2, len: 2 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(4, 1), pos: 4, len: 3 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(5, 2), pos: 7, len: 3 },
            op::Op { code: OpCode::ADD, operand: Fetched::B(4), pos: 10, len: 2 },
            // op::Op { code: OpCode::SSEND, operand: Fetched::BBB(3, 0, 1), pos: 12, len: 4 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(3, 4), pos: 12, len: 3 },
            op::Op { code: OpCode::RETURN, operand: Fetched::B(3), pos: 16, len: 2 },
            op::Op { code: OpCode::STOP, operand: Fetched::Z, pos: 18, len: 1 },
        ],
        syms: Vec::new(),
        pool: Vec::new(),
        reps: Vec::new(),
    };
    let mut vm = vm::VM::new_by_raw_irep(irep);
    vm.regs[0].replace(Rc::new(value::RObject::nil()));
    let ret = vm.run().unwrap();
    dbg!(ret);
}