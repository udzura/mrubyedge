extern crate mrubyedge;
use std::rc::Rc;

use mrubyedge::yamrb::*;
use mrubyedge::rite::insn::OpCode;
use mrubyedge::rite::insn::Fetched;

//
// This is a simple example of a new VM from realworld mrbc result.
//
fn main() {
    // irep1:
    // irep 0x6000020e4050 nregs=8 nlocals=3 pools=0 syms=1 reps=0 ilen=66
    // local variable names:
    //   R1:n
    // file: examples/fib.rb
    //     1 000 ENTER		1:0:0:0:0:0:0 (0x40000)
    //     2 004 MOVE		R3	R1		; R1:n
    //     2 007 LOADI_1	R4	(1)	
    //     2 009 LT		R3	R4
    //     2 011 JMPNOT	R3	022	
    //     3 015 LOADI_0	R3	(0)	
    //     3 017 RETURN	R3		
    //     3 019 JMP		064
    //     4 022 MOVE		R3	R1		; R1:n
    //     4 025 LOADI_3	R4	(3)	
    //     4 027 LT		R3	R4
    //     4 029 JMPNOT	R3	040	
    //     5 033 LOADI_1	R3	(1)	
    //     5 035 RETURN	R3		
    //     5 037 JMP		064
    //     7 040 MOVE		R4	R1		; R1:n
    //     7 043 SUBI		R4	1	
    //     7 046 SSEND		R3	:fib	n=1
    //     7 050 MOVE		R5	R1		; R1:n
    //     7 053 SUBI		R5	2	
    //     7 056 SSEND		R4	:fib	n=1
    //     7 060 ADD		R3	R4
    //     7 062 RETURN	R3		
    //     7 064 RETURN	R3		
    //
    let irep1 = vm::IREP {
        nlocals: 3,
        nregs: 8,
        rlen: 0,
        iren: 24,
        plen: 0,
        code: vec![
            op::Op { code: OpCode::ENTER, operand: Fetched::W(0x40000), pos: 0, len: 4 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(3, 1), pos: 4, len: 3 },
            op::Op { code: OpCode::LOADI_1, operand: Fetched::B(4), pos: 7, len: 2 },
            op::Op { code: OpCode::LT, operand: Fetched::B(3), pos: 9, len: 2 },
            op::Op { code: OpCode::JMPNOT, operand: Fetched::BS(3, 22), pos: 11, len: 4 },
            op::Op { code: OpCode::LOADI_0, operand: Fetched::B(3), pos: 15, len: 2 },
            op::Op { code: OpCode::RETURN, operand: Fetched::B(3), pos: 17, len: 2 },
            op::Op { code: OpCode::JMP, operand: Fetched::S(64), pos: 19, len: 3 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(3, 1), pos: 22, len: 3 },
            op::Op { code: OpCode::LOADI_3, operand: Fetched::B(4), pos: 25, len: 2 },
            op::Op { code: OpCode::LT, operand: Fetched::B(3), pos: 27, len: 2 },
            op::Op { code: OpCode::JMPNOT, operand: Fetched::BS(3, 40), pos: 29, len: 4 },
            op::Op { code: OpCode::LOADI_1, operand: Fetched::B(3), pos: 33, len: 2 },
            op::Op { code: OpCode::RETURN, operand: Fetched::B(3), pos: 35, len: 2 },
            op::Op { code: OpCode::JMP, operand: Fetched::S(64), pos: 37, len: 3},
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(4, 1), pos: 40, len: 3 },
            op::Op { code: OpCode::SUBI, operand: Fetched::BB(4, 1), pos: 43, len: 3 },
            op::Op { code: OpCode::SSEND, operand: Fetched::BBB(3, 0, 1), pos: 46, len: 4 },
            op::Op { code: OpCode::MOVE, operand: Fetched::BB(5, 1), pos: 50, len: 3 },
            op::Op { code: OpCode::SUBI, operand: Fetched::BB(5, 2), pos: 53, len: 3 },
            op::Op { code: OpCode::SSEND, operand: Fetched::BBB(4, 0, 1), pos: 56, len: 4 },
            op::Op { code: OpCode::ADD, operand: Fetched::B(3), pos: 60, len: 2 },
            op::Op { code: OpCode::RETURN, operand: Fetched::B(3), pos: 62, len: 2 },
            op::Op { code: OpCode::RETURN, operand: Fetched::B(3), pos: 64, len: 2 },
        ],
        syms: vec![value::RSym::new("fib".to_string())],
        pool: Vec::new(),
        reps: Vec::new(),
    };

    // irep0:
    // irep 0x6000020e4000 nregs=3 nlocals=1 pools=0 syms=1 reps=1 ilen=11
    // file: examples/fib.rb
    //     1 000 TCLASS	R1		
    //     1 002 METHOD	R2	I[0]
    //     1 005 DEF		R1	:fib
    //    11 008 LOADI		R2	10	
    //    11 011 SSEND		R1	:fib	n=1
    //    11 015 RETURN	R1		
    //    11 017 STOP
    //
    let irep0 = vm::IREP {
        nlocals: 1,
        nregs: 3,
        rlen: 0,
        iren: 5,
        plen: 0,
        code: vec![
            op::Op { code: OpCode::TCLASS, operand: Fetched::B(1), pos: 0, len: 2 },
            op::Op { code: OpCode::METHOD, operand: Fetched::BB(2, 0), pos: 2, len: 3 },
            op::Op { code: OpCode::DEF, operand: Fetched::BB(1, 0), pos: 5, len: 3 },
            op::Op { code: OpCode::LOADI, operand: Fetched::BB(2, 25), pos: 8, len: 3 },
            op::Op { code: OpCode::SSEND, operand: Fetched::BBB(1, 0, 1), pos: 11, len: 4 },
            op::Op { code: OpCode::RETURN, operand: Fetched::B(1), pos: 15, len: 2 },
            op::Op { code: OpCode::STOP, operand: Fetched::Z, pos: 17, len: 1 },
        ],
        syms: vec![value::RSym::new("fib".to_string())],
        pool: Vec::new(),
        reps: vec![Rc::new(irep1)],
    };
    let mut vm = vm::VM::new_by_raw_irep(irep0);
    vm.regs[0].replace(Rc::new(value::RObject::nil()));
    let ret = vm.run().unwrap();
    dbg!(ret);
}