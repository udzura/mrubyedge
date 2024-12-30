extern crate mrubyedge;
use std::rc::Rc;

use mrubyedge::yamrb::*;
use mrubyedge::rite::insn::OpCode;
use mrubyedge::rite::insn::Fetched;

//
// This is a simple example of a new VM from realworld mrbc result.
//
fn main() {
    // irep 0x600000f20050 nregs=7 nlocals=4 pools=0 syms=0 reps=0 ilen=14
    // local variable names:
    //   R1:a
    //   R2:b
    // file: examples/def2.rb
    //     1 000 ENTER		2:0:0:0:0:0:0 (0x80000)
    //     2 004 MOVE		R4	R1		; R1:a
    //     2 007 MOVE		R5	R2		; R2:b
    //     2 010 ADD		R4	R5
    //     2 012 RETURN	R4		
    //
    let irep1 = vm::IREP {
        nlocals: 0,
        nregs: 0,
        rlen: 0,
        iren: 5,
        plen: 0,
        code: vec![
            op::Op { code: OpCode::ENTER, operand: Fetched::W(0x80000), pos: 0, len: 4 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(4, 1), pos: 4, len: 3 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(5, 2), pos: 7, len: 3 },
            op::Op { code: OpCode::ADD, operand: Fetched::B(4), pos: 10, len: 2 },
            op::Op { code: OpCode::RETURN, operand: Fetched::B(4), pos: 12, len: 2 },
        ],
        syms: Vec::new(),
        pool: Vec::new(),
        reps: Vec::new(),
    };

    // irep 0x600000f20000 nregs=7 nlocals=3 pools=0 syms=1 reps=1 ilen=27
    // local variable names:
    //   R1:c
    //   R2:d
    // file: examples/def2.rb
    //     1 000 TCLASS	R3		
    //     1 002 METHOD	R4	I[0]
    //     1 005 DEF		R3	:do_add
    //     5 008 LOADI		R1	100		; R1:c
    //     6 011 LOADI		R2	200		; R2:d
    //     7 014 MOVE		R4	R1		; R1:c
    //     7 017 MOVE		R5	R2		; R2:d
    //     7 020 SSEND		R3	:do_add	n=2
    //     7 024 RETURN	R3		
    //     7 026 STOP
    //
    let irep0 = vm::IREP {
        nlocals: 0,
        nregs: 0,
        rlen: 0,
        iren: 8,
        plen: 0,
        code: vec![
            op::Op { code: OpCode::TCLASS, operand: Fetched::B(3), pos: 0, len: 2 },
            op::Op { code: OpCode::METHOD, operand: Fetched::BB(4, 0), pos: 2, len: 3 },
            op::Op { code: OpCode::DEF, operand: Fetched::BB(3, 0), pos: 5, len: 3 },
            op::Op { code: OpCode::LOADI, operand: Fetched::BB(1, 100), pos: 8, len: 3 },
            op::Op { code: OpCode::LOADI, operand: Fetched::BB(2, 200), pos: 11, len: 3 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(4, 1), pos: 14, len: 3 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(5, 2), pos: 17, len: 3 },
            op::Op { code: OpCode::SSEND, operand: Fetched::BBB(3, 0, 2), pos: 20, len: 4 },
            op::Op { code: OpCode::RETURN, operand: Fetched::B(3), pos: 24, len: 2 },
            op::Op { code: OpCode::STOP, operand: Fetched::Z, pos: 26, len: 1 },
        ],
        syms: Vec::new(),
        pool: Vec::new(),
        reps: vec![irep1],
    };
    let mut vm = vm::VM::new_by_irep(irep0);
    vm.regs[0].replace(Rc::new(value::RObject::nil()));
    let ret = vm.run().unwrap();
    dbg!(ret);
}