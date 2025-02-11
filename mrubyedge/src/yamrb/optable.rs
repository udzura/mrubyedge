use std::cell::Cell;
use std::cell::RefCell;

use std::collections::HashMap;
use std::rc::Rc;

use crate::rite::insn::{Fetched, OpCode};

use super::prelude::object::mrb_object_is_equal;
use super::{helpers::mrb_funcall, value::*, vm::*};

// OpCodes of mruby 3.2.0 from mruby/op.h:
// OPCODE(NOP,        Z)        /* no operation */
// OPCODE(MOVE,       BB)       /* R[a] = R[b] */
// OPCODE(LOADL,      BB)       /* R[a] = Pool[b] */
// OPCODE(LOADI,      BB)       /* R[a] = mrb_int(b) */
// OPCODE(LOADINEG,   BB)       /* R[a] = mrb_int(-b) */
// OPCODE(LOADI__1,   B)        /* R[a] = mrb_int(-1) */
// OPCODE(LOADI_0,    B)        /* R[a] = mrb_int(0) */
// OPCODE(LOADI_1,    B)        /* R[a] = mrb_int(1) */
// OPCODE(LOADI_2,    B)        /* R[a] = mrb_int(2) */
// OPCODE(LOADI_3,    B)        /* R[a] = mrb_int(3) */
// OPCODE(LOADI_4,    B)        /* R[a] = mrb_int(4) */
// OPCODE(LOADI_5,    B)        /* R[a] = mrb_int(5) */
// OPCODE(LOADI_6,    B)        /* R[a] = mrb_int(6) */
// OPCODE(LOADI_7,    B)        /* R[a] = mrb_int(7) */
// OPCODE(LOADI16,    BS)       /* R[a] = mrb_int(b) */
// OPCODE(LOADI32,    BSS)      /* R[a] = mrb_int((b<<16)+c) */
// OPCODE(LOADSYM,    BB)       /* R[a] = Syms[b] */
// OPCODE(LOADNIL,    B)        /* R[a] = nil */
// OPCODE(LOADSELF,   B)        /* R[a] = self */
// OPCODE(LOADT,      B)        /* R[a] = true */
// OPCODE(LOADF,      B)        /* R[a] = false */
// OPCODE(GETGV,      BB)       /* R[a] = getglobal(Syms[b]) */
// OPCODE(SETGV,      BB)       /* setglobal(Syms[b], R[a]) */
// OPCODE(GETSV,      BB)       /* R[a] = Special[Syms[b]] */
// OPCODE(SETSV,      BB)       /* Special[Syms[b]] = R[a] */
// OPCODE(GETIV,      BB)       /* R[a] = ivget(Syms[b]) */
// OPCODE(SETIV,      BB)       /* ivset(Syms[b],R[a]) */
// OPCODE(GETCV,      BB)       /* R[a] = cvget(Syms[b]) */
// OPCODE(SETCV,      BB)       /* cvset(Syms[b],R[a]) */
// OPCODE(GETCONST,   BB)       /* R[a] = constget(Syms[b]) */
// OPCODE(SETCONST,   BB)       /* constset(Syms[b],R[a]) */
// OPCODE(GETMCNST,   BB)       /* R[a] = R[a]::Syms[b] */
// OPCODE(SETMCNST,   BB)       /* R[a+1]::Syms[b] = R[a] */
// OPCODE(GETUPVAR,   BBB)      /* R[a] = uvget(b,c) */
// OPCODE(SETUPVAR,   BBB)      /* uvset(b,c,R[a]) */
// OPCODE(GETIDX,     B)        /* R[a] = R[a][R[a+1]] */
// OPCODE(SETIDX,     B)        /* R[a][R[a+1]] = R[a+2] */
// OPCODE(JMP,        S)        /* pc+=a */
// OPCODE(JMPIF,      BS)       /* if R[a] pc+=b */
// OPCODE(JMPNOT,     BS)       /* if !R[a] pc+=b */
// OPCODE(JMPNIL,     BS)       /* if R[a]==nil pc+=b */
// OPCODE(JMPUW,      S)        /* unwind_and_jump_to(a) */
// OPCODE(EXCEPT,     B)        /* R[a] = exc */
// OPCODE(RESCUE,     BB)       /* R[b] = R[a].isa?(R[b]) */
// OPCODE(RAISEIF,    B)        /* raise(R[a]) if R[a] */
// OPCODE(SSEND,      BBB)      /* R[a] = self.send(Syms[b],R[a+1]..,R[a+n+1]:R[a+n+2]..) (c=n|k<<4) */
// OPCODE(SSENDB,     BBB)      /* R[a] = self.send(Syms[b],R[a+1]..,R[a+n+1]:R[a+n+2]..,&R[a+n+2k+1]) */
// OPCODE(SEND,       BBB)      /* R[a] = R[a].send(Syms[b],R[a+1]..,R[a+n+1]:R[a+n+2]..) (c=n|k<<4) */
// OPCODE(SENDB,      BBB)      /* R[a] = R[a].send(Syms[b],R[a+1]..,R[a+n+1]:R[a+n+2]..,&R[a+n+2k+1]) */
// OPCODE(CALL,       Z)        /* self.call(*, **, &) (But overlay the current call frame; tailcall) */
// OPCODE(SUPER,      BB)       /* R[a] = super(R[a+1],... ,R[a+b+1]) */
// OPCODE(ARGARY,     BS)       /* R[a] = argument array (16=m5:r1:m5:d1:lv4) */
// OPCODE(ENTER,      W)        /* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
// OPCODE(KEY_P,      BB)       /* R[a] = kdict.key?(Syms[b]) */
// OPCODE(KEYEND,     Z)        /* raise unless kdict.empty? */
// OPCODE(KARG,       BB)       /* R[a] = kdict[Syms[b]]; kdict.delete(Syms[b]) */
// OPCODE(RETURN,     B)        /* return R[a] (normal) */
// OPCODE(RETURN_BLK, B)        /* return R[a] (in-block return) */
// OPCODE(BREAK,      B)        /* break R[a] */
// OPCODE(BLKPUSH,    BS)       /* R[a] = block (16=m5:r1:m5:d1:lv4) */
// OPCODE(ADD,        B)        /* R[a] = R[a]+R[a+1] */
// OPCODE(ADDI,       BB)       /* R[a] = R[a]+mrb_int(b) */
// OPCODE(SUB,        B)        /* R[a] = R[a]-R[a+1] */
// OPCODE(SUBI,       BB)       /* R[a] = R[a]-mrb_int(b) */
// OPCODE(MUL,        B)        /* R[a] = R[a]*R[a+1] */
// OPCODE(DIV,        B)        /* R[a] = R[a]/R[a+1] */
// OPCODE(EQ,         B)        /* R[a] = R[a]==R[a+1] */
// OPCODE(LT,         B)        /* R[a] = R[a]<R[a+1] */
// OPCODE(LE,         B)        /* R[a] = R[a]<=R[a+1] */
// OPCODE(GT,         B)        /* R[a] = R[a]>R[a+1] */
// OPCODE(GE,         B)        /* R[a] = R[a]>=R[a+1] */
// OPCODE(ARRAY,      BB)       /* R[a] = ary_new(R[a],R[a+1]..R[a+b]) */
// OPCODE(ARRAY2,     BBB)      /* R[a] = ary_new(R[b],R[b+1]..R[b+c]) */
// OPCODE(ARYCAT,     B)        /* ary_cat(R[a],R[a+1]) */
// OPCODE(ARYPUSH,    BB)       /* ary_push(R[a],R[a+1]..R[a+b]) */
// OPCODE(ARYSPLAT,   B)        /* R[a] = ary_splat(R[a]) */
// OPCODE(AREF,       BBB)      /* R[a] = R[b][c] */
// OPCODE(ASET,       BBB)      /* R[b][c] = R[a] */
// OPCODE(APOST,      BBB)      /* *R[a],R[a+1]..R[a+c] = R[a][b..] */
// OPCODE(INTERN,     B)        /* R[a] = intern(R[a]) */
// OPCODE(SYMBOL,     BB)       /* R[a] = intern(Pool[b]) */
// OPCODE(STRING,     BB)       /* R[a] = str_dup(Pool[b]) */
// OPCODE(STRCAT,     B)        /* str_cat(R[a],R[a+1]) */
// OPCODE(HASH,       BB)       /* R[a] = hash_new(R[a],R[a+1]..R[a+b*2-1]) */
// OPCODE(HASHADD,    BB)       /* hash_push(R[a],R[a+1]..R[a+b*2]) */
// OPCODE(HASHCAT,    B)        /* R[a] = hash_cat(R[a],R[a+1]) */
// OPCODE(LAMBDA,     BB)       /* R[a] = lambda(Irep[b],L_LAMBDA) */
// OPCODE(BLOCK,      BB)       /* R[a] = lambda(Irep[b],L_BLOCK) */
// OPCODE(METHOD,     BB)       /* R[a] = lambda(Irep[b],L_METHOD) */
// OPCODE(RANGE_INC,  B)        /* R[a] = range_new(R[a],R[a+1],FALSE) */
// OPCODE(RANGE_EXC,  B)        /* R[a] = range_new(R[a],R[a+1],TRUE) */
// OPCODE(OCLASS,     B)        /* R[a] = ::Object */
// OPCODE(CLASS,      BB)       /* R[a] = newclass(R[a],Syms[b],R[a+1]) */
// OPCODE(MODULE,     BB)       /* R[a] = newmodule(R[a],Syms[b]) */
// OPCODE(EXEC,       BB)       /* R[a] = blockexec(R[a],Irep[b]) */
// OPCODE(DEF,        BB)       /* R[a].newmethod(Syms[b],R[a+1]); R[a] = Syms[b] */
// OPCODE(ALIAS,      BB)       /* alias_method(target_class,Syms[a],Syms[b]) */
// OPCODE(UNDEF,      B)        /* undef_method(target_class,Syms[a]) */
// OPCODE(SCLASS,     B)        /* R[a] = R[a].singleton_class */
// OPCODE(TCLASS,     B)        /* R[a] = target_class */
// OPCODE(DEBUG,      BBB)      /* print a,b,c */
// OPCODE(ERR,        B)        /* raise(LocalJumpError, Pool[a]) */
// OPCODE(EXT1,       Z)        /* make 1st operand (a) 16bit */
// OPCODE(EXT2,       Z)        /* make 2nd operand (b) 16bit */
// OPCODE(EXT3,       Z)        /* make 1st and 2nd operands 16bit */
// OPCODE(STOP,       Z)        /* stop VM */
// functions that represent each opcode are defined in this file.
// to understand the meaning of each operand mark, see enum Fetched in rite/insn.rs:
// pub enum Fetched {
//     Z,
//     B(u8),
//     BB(u8, u8),
//     BBB(u8, u8, u8),
//     BS(u8, u16),
//     BSS(u8, u16, u16),
//     S(u16),
//     W(u32), // u24 in real layout
// }
//

const ENTER_M1_MASK: u32 = 0b11111 << 18;
const ENTER_O_MASK: u32 = 0b11111 << 13;
const ENTER_R_MASK: u32 = 0b1 << 12;
const ENTER_M2_MASK: u32 = 0b11111 << 7;
const ENTER_K_MASK: u32 = 0b11111 << 2;
const ENTER_D_MASK: u32 = 0b1 << 1;
const ENTER_B_MASK: u32 = 0b1 << 0;

pub(crate) fn consume_expr(vm: &mut VM, code: OpCode, operand: &Fetched, pos: usize, len: usize) {
    use crate::rite::insn::OpCode::*;
    match code {
        NOP => {
            op_nop(vm, &operand);
        }
        MOVE => {
            op_move(vm, &operand);
        }
        LOADL => {
            op_loadl(vm, &operand);
        }
        LOADI => {
            op_loadi(vm, &operand);
        }
        LOADINEG => {
            op_loadineg(vm, &operand);
        }
        LOADI__1 => {
            op_loadi_n(vm, -1, &operand);
        }
        LOADI_0 => {
            op_loadi_n(vm, 0, &operand);
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
        LOADI16 => {
            op_loadi16(vm, &operand);
        }
        LOADI32 => {
            op_loadi32(vm, &operand);
        }
        LOADSYM => {
            op_loadsym(vm, &operand);
        }
        LOADNIL => {
            op_loadnil(vm, &operand);
        }
        LOADSELF => {
            op_loadself(vm, &operand);
        }
        LOADT => {
            op_loadt(vm, &operand);
        }
        LOADF => {
            op_loadf(vm, &operand);
        }
        GETGV => {
            op_getgv(vm, &operand);
        }
        SETGV => {
            op_setgv(vm, &operand);
        }
        // GETSV => {
        //     // op_getsv(vm, &operand);
        // }
        // SETSV => {
        //     // op_setsv(vm, &operand);
        // }
        GETIV => {
            op_getiv(vm, &operand);
        }
        SETIV => {
            op_setiv(vm, &operand);
        }
        // GETCV => {
        //     // op_getcv(vm, &operand);
        // }
        // SETCV => {
        //     // op_setcv(vm, &operand);
        // }
        GETCONST => {
            op_getconst(vm, &operand);
        }
        SETCONST => {
            op_setconst(vm, &operand);
        }
        GETMCNST => {
            op_getmcnst(vm, &operand);
        }
        // SETMCNST => {
        //     // op_setmcnst(vm, &operand);
        // }
        GETUPVAR => {
            op_getupvar(vm, &operand);
        }
        SETUPVAR => {
            op_setupvar(vm, &operand);
        }
        GETIDX => {
            op_getidx(vm, &operand);
        }
        SETIDX => {
            op_setidx(vm, &operand);
        }
        JMP => {
            op_jmp(vm, &operand, pos + len);
        }
        JMPIF => {
            op_jmpif(vm, &operand, pos + len);
        }
        JMPNOT => {
            op_jmpnot(vm, &operand, pos + len);
        }
        JMPNIL => {
            op_jmpnil(vm, &operand, pos + len);
        }
        // JMPUW => {
        //     // op_jmpuw(vm, &operand);
        // }
        // EXCEPT => {
        //     // op_except(vm, &operand);
        // }
        // RESCUE => {
        //     // op_rescue(vm, &operand);
        // }
        // RAISEIF => {
        //     // op_raiseif(vm, &operand);
        // }
        SSEND => {
            op_ssend(vm, &operand);
        }
        SSENDB => {
            op_ssendb(vm, &operand);
        }
        SEND => {
            op_send(vm, &operand);
        }
        SENDB => {
            op_sendb(vm, &operand);
        }
        CALL => {
            op_call(vm, &operand);
        }
        SUPER => {
            op_super(vm, &operand);
        }
        // ARGARY => {
        //     // op_argary(vm, &operand);
        // }
        ENTER => {
            op_enter(vm, &operand);
        }
        // KEY_P => {
        //     // op_key_p(vm, &operand);
        // }
        // KEYEND => {
        //     // op_keyend(vm, &operand);
        // }
        // KARG => {
        //     // op_karg(vm, &operand);
        // }
        RETURN => {
            op_return(vm, &operand);
        }
        // RETURN_BLK => {
        //     // op_return_blk(vm, &operand);
        // }
        // BREAK => {
        //     // op_break(vm, &operand);
        // }
        // BLKPUSH => {
        //     // op_blkpush(vm, &operand);
        // }
        ADD => {
            op_add(vm, &operand);
        }
        ADDI => {
            op_addi(vm, &operand);
        }
        SUB => {
            op_sub(vm, &operand);
        }
        SUBI => {
            op_subi(vm, &operand);
        }
        MUL => {
            op_mul(vm, &operand);
        }
        DIV => {
            op_div(vm, &operand);
        }
        EQ => {
            op_eq(vm, &operand);
        }
        LT => {
            op_lt(vm, &operand);
        }
        LE => {
            op_le(vm, &operand);
        }
        GT => {
            op_gt(vm, &operand);
        }
        GE => {
            op_ge(vm, &operand);
        }
        ARRAY => {
            op_array(vm, &operand);
        }
        ARRAY2 => {
            op_array2(vm, &operand);
        }
        // ARYCAT => {
        //     // op_arycat(vm, &operand);
        // }
        // ARYPUSH => {
        //     // op_arypush(vm, &operand);
        // }
        // ARYSPLAT => {
        //     // op_arysplat(vm, &operand);
        // }
        // AREF => {
        //     // op_aref(vm, &operand);
        // }
        // ASET => {
        //     // op_aset(vm, &operand);
        // }
        // APOST => {
        //     // op_apost(vm, &operand);
        // }
        // INTERN => {
        //     // op_intern(vm, &operand);
        // }
        SYMBOL => {
            op_symbol(vm, &operand);
        }
        STRING => {
            op_string(vm, &operand);
        }
        STRCAT => {
            op_strcat(vm, &operand);
        }
        HASH => {
            op_hash(vm, &operand);
        }
        // HASHADD => {
        //     // op_hashadd(vm, &operand);
        // }
        // HASHCAT => {
        //     // op_hashcat(vm, &operand);
        // }
        LAMBDA => {
            op_lambda(vm, &operand);
        }
        BLOCK => {
            op_block(vm, &operand);
        }
        METHOD => {
            op_method(vm, &operand);
        }
        RANGE_INC => {
            op_range_inc(vm, &operand);
        }
        RANGE_EXC => {
            op_range_exc(vm, &operand);
        }
        OCLASS => {
            op_oclass(vm, &operand);
        }
        CLASS => {
            op_class(vm, &operand);
        }
        // MODULE => {
        //     // op_module(vm, &operand);
        // }
        EXEC => {
            op_exec(vm, &operand);
        }
        DEF => {
            op_def(vm, &operand);
        }
        // ALIAS => {
        //     // op_alias(vm, &operand);
        // }
        // UNDEF => {
        //     // op_undef(vm, &operand);
        // }
        // SCLASS => {
        //     // op_sclass(vm, &operand);
        // }
        TCLASS => {
            op_tclass(vm, &operand);
        }
        // DEBUG => {
        //     // op_debug(vm, &operand);
        // }
        // ERR => {
        //     // op_err(vm, &operand);
        // }
        // EXT1 => {
        //     // op_ext1(vm, &operand);
        // }
        // EXT2 => {
        //     // op_ext2(vm, &operand);
        // }
        // EXT3 => {
        //     // op_ext3(vm, &operand);
        // }
        STOP => {
            op_stop(vm, &operand);
        }
        _ => { unimplemented!("{:?}: Not supported yet", code)}
    }
}

pub(crate) fn push_callinfo(vm: &mut VM, method_id: RSym, n_args: usize) {
    let callinfo = CALLINFO {
        prev: vm.current_callinfo.clone(),
        method_id,
        pc_irep: vm.current_irep.clone(),
        pc: vm.pc.get(),
        current_regs_offset: vm.current_regs_offset,
        n_args,
        target_class: vm.target_class.clone(),
    };
    vm.current_callinfo = Some(Rc::new(callinfo));
}

fn calcurate_pc(irep: &IREP, pc: usize, original_pc: usize) -> usize {
    let mut next_pc = pc;
    loop {
        let op = irep.code.get(next_pc).expect("cannot fetch op anymore");
        // dbg!((&op, original_pc));
        if op.pos == original_pc {
            break;
        }
        next_pc += 1;
    }
    next_pc
}

pub(crate) fn op_nop(_vm: &mut VM, _operand: &Fetched) {
    // NOOP
    // eprintln!("[debug] nop");
}

pub(crate) fn op_loadi_n(vm: &mut VM, n: i32, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let val = RObject::integer(n as i64);
    vm.current_regs()[a].replace(Rc::new(val));
}

pub(crate) fn op_loadl(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = vm.current_irep.pool[b as usize].clone();
    // TODO: support other rpool types?
    let val = Rc::new(RObject::string(val.as_str().to_string()));
    vm.current_regs()[a as usize].replace(val);
}

pub(crate) fn op_loadi16(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bs().unwrap();
    let val = RObject::integer(b as i64);
    vm.current_regs()[a as usize].replace(Rc::new(val));
}

pub(crate) fn op_loadi32(vm: &mut VM, operand: &Fetched) {
    let (a, b, c) = operand.as_bss().unwrap();
    let val = RObject::integer((b as i64) << 16 | c as i64);
    vm.current_regs()[a as usize].replace(Rc::new(val));
}

pub(crate) fn op_loadi(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = RObject::integer(b as i64);
    vm.current_regs()[a as usize].replace(Rc::new(val));
}

pub(crate) fn op_loadineg(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = RObject::integer(-(b as i64));
    vm.current_regs()[a as usize].replace(Rc::new(val));
}

pub(crate) fn op_loadsym(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = vm.current_irep.syms[b as usize].clone();
    vm.current_regs()[a as usize].replace(Rc::new(RObject::symbol(val)));
}

pub(crate) fn op_loadnil(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let val = RObject::nil();
    vm.current_regs()[a].replace(Rc::new(val));
}

pub(crate) fn op_loadself(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let val = vm.current_regs()[0].as_ref().cloned().unwrap();
    vm.current_regs()[a].replace(val);
}

pub(crate) fn op_loadt(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let val = RObject::boolean(true);
    vm.current_regs()[a].replace(Rc::new(val));
}

pub(crate) fn op_loadf(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let val = RObject::boolean(false);
    vm.current_regs()[a].replace(Rc::new(val));
}

pub(crate) fn op_getgv(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = vm.current_irep.syms[b as usize].clone();
    let val = vm.globals.get(&val.name).unwrap().clone();
    vm.current_regs()[a as usize].replace(val);
}

pub(crate) fn op_setgv(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = vm.current_regs()[a as usize].as_ref().cloned().unwrap();
    let sym = vm.current_irep.syms[b as usize].clone();
    vm.globals.insert(sym.name.clone(), val);
}

pub(crate) fn op_getiv(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let this = vm.current_regs()[0].as_ref().cloned().unwrap();
    let ivar = match &this.value {
        RValue::Instance(ins) => ins.ivar.borrow().get(
            &vm.current_irep.syms[b as usize].name,
        ).unwrap().clone(),
        _ => unreachable!("getiv must be called on instance")
    };
    vm.current_regs()[a as usize].replace(ivar);
}

pub(crate) fn op_setiv(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let this = vm.current_regs()[0].as_ref().cloned().unwrap();
    let val = vm.current_regs()[a as usize].as_ref().cloned().unwrap();
    match &this.value {
        RValue::Instance(ins) => {
            let mut ivar = ins.ivar.borrow_mut();
            ivar.insert(
                vm.current_irep.syms[b as usize].name.clone(),
                val,
            )
        },
        _ => unreachable!("setiv must be called on instance")
    };
}

pub(crate) fn op_getconst(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let name = &vm.current_irep.syms[b as usize].name;
    let cval = vm.consts.get(name).cloned();
    match cval {
        Some(val) => {
            vm.current_regs()[a as usize].replace(val);
        },
        None => {
            panic!("constant not found: {:?}", name);
        }
    }
}

pub(crate) fn op_setconst(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let name = vm.current_irep.syms[b as usize].name.clone();
    let val = vm.current_regs()[a as usize].as_ref().cloned().unwrap();
    vm.consts.insert(name, val);
}

pub(crate) fn op_getmcnst(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let recv = vm.current_regs()[a as usize].take().unwrap();
    let name = vm.current_irep.syms[b as usize].name.clone();
    let cval = match &recv.value {
        RValue::Class(klass) => {klass.getmcnst(&name).clone()},
        _ => unreachable!("getmcnst must be called on class or module")
    };
    match cval {
        Some(val) => {
            vm.current_regs()[a as usize].replace(val);
        },
        None => {
            panic!("constant not found: {:?}", name);
        }
    }
}

pub(crate) fn op_getupvar(vm: &mut VM, operand: &Fetched) {
    let (a, b, c) = operand.as_bbb().unwrap();
    let n = c as usize;
    let mut environ = vm.upper.as_ref().expect("op_getupvar expects upper env");
    for _ in 0..n {
        environ = environ.upper.as_ref().expect("op_getupvar failed to find upvar");
    }
    let environ = environ.clone();
    let up_regs = &vm.regs[environ.current_regs_offset..];
    if !environ.expired() {
        if let Some(val) = up_regs[b as usize].as_ref().cloned() {
            vm.current_regs()[a as usize].replace(val);
        } else {
            panic!("register {} is empty", b);
        }
    } else {
        let captured = environ.captured.borrow();
        let val = &captured.as_ref().unwrap().clone()[b as usize];
        let val = val.clone();
        vm.current_regs()[a as usize].replace(val.unwrap());
    }
}

pub(crate) fn op_setupvar(vm: &mut VM, operand: &Fetched) {
    let (a, b, c) = operand.as_bbb().unwrap();
    let n = c as usize;
    let mut environ = vm.upper.as_ref().expect("op_getupvar expects upper env");
    for _ in 0..n {
        environ = environ.upper.as_ref().expect("op_getupvar failed to find upvar");
    }
    let environ = environ.clone();
    let current_regs_offset = environ.current_regs_offset;

    let val = vm.current_regs()[a as usize].as_ref().cloned().unwrap();
    // let up_regs = &vm.regs[environ.current_regs_offset..];
    if !environ.expired() {
        let up_regs= &mut vm.regs[current_regs_offset..];
        let target = &mut up_regs[b as usize];
        target.replace(val);
    } else {
        let mut captured = environ.captured.borrow_mut();
        let captured = captured.as_mut().unwrap();
        let target = &mut captured[b as usize];
        target.replace(val);
    }
}

pub(crate) fn op_getidx(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let recv = vm.current_regs()[a].as_ref().cloned().unwrap();
    let idx = vm.current_regs()[a + 1].as_ref().cloned().unwrap();
    let args = vec![idx];
    // TODO: direct call of array_index for performance
    let val = mrb_funcall(vm, Some(recv), "[]", &args).expect("calling index failed");
    vm.current_regs()[a].replace(val);
}

pub(crate) fn op_setidx(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let recv = vm.current_regs()[a].as_ref().cloned().unwrap();
    let idx = vm.current_regs()[a + 1].as_ref().cloned().unwrap();
    let val = vm.current_regs()[a + 2].as_ref().cloned().unwrap();
    let args = vec![idx, val];
    mrb_funcall(vm, Some(recv), "[]=", &args).expect("calling index failed");
}

pub(crate) fn op_jmp(vm: &mut VM, operand: &Fetched, end_pos: usize) {
    let a = operand.as_s().unwrap();
    let next_pc = calcurate_pc(&vm.current_irep, vm.pc.get(), end_pos + a as usize);
    vm.pc.set(next_pc);
}

pub(crate) fn op_jmpif(vm: &mut VM, operand: &Fetched, end_pos: usize) {
    let (a, b) = operand.as_bs().unwrap();
    let val = vm.current_regs()[a as usize].as_ref().cloned().unwrap();
    if val.is_truthy() {
        let next_pc = calcurate_pc(&vm.current_irep, vm.pc.get(), end_pos + b as usize);
        vm.pc.set(next_pc);
    }
}

pub(crate) fn op_jmpnot(vm: &mut VM, operand: &Fetched, end_pos: usize) {
    let (a, b) = operand.as_bs().unwrap();
    let val = vm.current_regs()[a as usize].as_ref().cloned().unwrap();
    if val.is_falsy() {
        let next_pc = calcurate_pc(&vm.current_irep, vm.pc.get(), end_pos + b as usize);
        vm.pc.set(next_pc);
    }
}

pub(crate) fn op_jmpnil(vm: &mut VM, operand: &Fetched, end_pos: usize) {
    let (a, b) = operand.as_bs().unwrap();
    let val = vm.current_regs()[a as usize].as_ref().cloned().unwrap();
    if val.is_nil() {
        let next_pc = calcurate_pc(&vm.current_irep, vm.pc.get(), end_pos + b as usize);
        vm.pc.set(next_pc);
    }
}

pub(crate) fn op_move(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = vm.current_regs()[b as usize].clone().
        expect(&format!("register {} is empty", b));
    vm.current_regs()[a as usize].replace(val);
}

pub(crate) fn op_ssend(vm: &mut VM, operand: &Fetched) {
    let (a, b, c) = operand.as_bbb().unwrap();
    do_op_send(vm, 0, None, a, b, c);
}

pub(crate) fn op_ssendb(vm: &mut VM, operand: &Fetched) {
    let (a, b, c) = operand.as_bbb().unwrap();
    do_op_send(vm, 0, Some(a as usize + c as usize + 1), a, b, c);
}

pub(crate) fn op_send(vm: &mut VM, operand: &Fetched) {
    let (a, b, c) = operand.as_bbb().unwrap();
    do_op_send(vm, a as usize, None, a, b, c);
}

pub(crate) fn op_sendb(vm: &mut VM, operand: &Fetched) {
    let (a, b, c) = operand.as_bbb().unwrap();
    do_op_send(vm, a as usize, Some(a as usize + c as usize + 1), a, b, c);
}

pub(crate) fn do_op_send(vm: &mut VM, recv_index: usize, blk_index: Option<usize>, a: u8, b: u8, c: u8) {
    let block_index = (a + c + 1) as usize;

    let recv = vm.current_regs()[recv_index].as_ref().cloned().unwrap();
    let mut args = (0..c).
        map(|i| vm.current_regs()[(a + i + 1) as usize].as_ref().cloned().unwrap()).collect::<Vec<_>>();
    if let Some(blk_index) = blk_index {
        args.push(vm.current_regs()[blk_index].as_ref().cloned().unwrap());
    } else {
        args.push(Rc::new(RObject::nil()));
    }

    let method_id = vm.current_irep.syms[b as usize].clone();
    let klass = recv.get_class(vm);
    let method = klass.find_method(&method_id.name).expect(
        &format!("method {} not found. TODO: medthod_missing", &method_id.name),
    );
    if !method.is_rb_func {
        let func = vm.get_fn(method.func.unwrap()).unwrap();
        vm.current_regs_offset += a as usize;

        let res = func(vm, &args);

        vm.current_regs_offset -= a as usize;
        for i in (a as usize + 1)..block_index {
            vm.current_regs()[i].take();
        }
 
        if res.is_err() {
            eprintln!("[TODO] error handling: {:?}", &res);
            vm.error_code = 255;
            vm.current_regs()[a as usize].replace(Rc::new(RObject::nil()));
        } else {
            vm.current_regs()[a as usize].replace(res.unwrap());
        }

        return
    }

    vm.current_regs()[a as usize].replace(recv.clone());
    push_callinfo(vm, method_id, c as usize);

    vm.pc.set(0);
    vm.current_irep = method.irep.unwrap();
    vm.current_regs_offset += a as usize;
}

pub(crate) fn op_call(vm: &mut VM, _operand: &Fetched) {
    push_callinfo(vm, "<tailcall>".into(), 0);

    vm.pc.set(0);
    let proc = vm.current_regs()[0].as_ref().cloned().unwrap();
    match &proc.value {
        RValue::Proc(proc) => {
            vm.current_irep = proc.irep.clone().unwrap();
        },
        _ => unreachable!("call must be called on proc")
    }
}

pub(crate) fn op_super(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    // mrbc_sym sym_id = vm->callinfo_tail->mid;
    let sym_id = vm.current_callinfo.as_ref().unwrap().method_id.name.clone();
    let recv = vm.current_regs()[0].as_ref().cloned().unwrap();
    let args = (0..b).
        map(|i| vm.current_regs()[(a + i + 1) as usize].as_ref().cloned().unwrap()).collect::<Vec<_>>();

    let klass = match &recv.value {
        RValue::Instance(ins) => ins.class.as_ref(),
        _ => unreachable!("super must be called on instance")
    };
    let superclass = klass.super_class.as_ref().expect("superclass not found");
    let sc_procs = superclass.procs.borrow();
    let method = sc_procs.get(&sym_id).unwrap();
    if !method.is_rb_func {
        let func = vm.get_fn(method.func.unwrap()).unwrap();
        let res = func(vm, &args);
        if res.is_err() {
            vm.error_code = 255;
        }
        for i in (a as usize + 1)..(a as usize + b as usize + 1) {
            vm.current_regs()[i].take();
        }
        return
    }
    
    vm.current_regs()[a as usize].replace(recv.clone());
    push_callinfo(vm, method.sym_id.clone().unwrap(), b as usize);

    vm.pc.set(0);
    vm.current_irep = method.irep.as_ref().unwrap().clone();
    vm.current_regs_offset += a as usize;
}

#[allow(dead_code)]
struct EnterArgInfo {
    m1: u32,
    o: u32,
    r: u32,
    m2: u32,
    k: u32,
    d: u32,
    b: u32,
}

impl From<u32> for EnterArgInfo {
    fn from(val: u32) -> Self {
        EnterArgInfo {
            m1: (val & ENTER_M1_MASK) >> 18,
            o: (val & ENTER_O_MASK) >> 13,
            r: (val & ENTER_R_MASK) >> 12,
            m2: (val & ENTER_M2_MASK) >> 7,
            k: (val & ENTER_K_MASK) >> 2,
            d: (val & ENTER_D_MASK) >> 1,
            b: (val & ENTER_B_MASK) >> 0,
        }
    }
}

pub(crate) fn op_enter(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_w().unwrap();
    let arg_info = EnterArgInfo::from(a);
    // TODO: only check use of m1
    let m1_argc = arg_info.m1 as usize;
    for i in 0..m1_argc {
        match vm.current_regs()[i + 1].as_ref() {
            Some(_) => {
                // no problem,
            },
            None => {
                unreachable!("argument {} not passed", i + 1);
            }
        }
    }
}

pub(crate) fn op_return(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let old_irep = vm.current_irep.clone();
    let nregs = old_irep.nregs;

    // TODO: capturing env on return makes lots of clone
    //       I wish it would be optimized
    let regs0_cloned: Vec<_> = vm.current_regs()[0..nregs].iter().cloned().collect();
    if let Some(_) = vm.has_env_ref.get(&vm.current_irep.__id) {
        if let Some(environ) = vm.cur_env.get(&vm.current_irep.__id) {
            environ.capture_no_clone(regs0_cloned);
            environ.as_ref().expire();
            vm.has_env_ref.remove(&vm.current_irep.__id);
        }
    }

    let regs0 = vm.current_regs();
    if let Some(regs_a) = regs0[a].take() {
        regs0[0].replace(regs_a);
    }
    if nregs > 0 {
        for i in 1..=nregs {
            regs0[i].take();
        }
    }

    let ci = vm.current_callinfo.take();
    if ci.is_none() {
        // Will stop VM: no more opereation
        return;
    }

    let ci = ci.unwrap();
    if ci.prev.is_some() {
        vm.current_callinfo.replace(ci.prev.clone().unwrap());
    }
    vm.current_irep = ci.pc_irep.clone();
    vm.pc.set(ci.pc);
    vm.current_regs_offset = ci.current_regs_offset;
    vm.target_class = ci.target_class.clone();
    if vm.current_regs()[0].is_none() {
        todo!("debug");
    }
}

pub(crate) fn op_add(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.current_regs()[a].take().unwrap();
    let val2 = vm.current_regs()[b].take().unwrap();
    let result = match (&val1.value, &val2.value) {
        (RValue::Integer(n1), RValue::Integer(n2)) => {
            Rc::new(RObject::integer(n1 + n2))
        }
        (RValue::Float(n1), RValue::Float(n2)) => {
            Rc::new(RObject::float(n1 + n2))
        }
        (RValue::Integer(n1), RValue::Float(n2)) => {
            Rc::new(RObject::float(*n1 as f64 + n2))
        }
        (RValue::Float(n1), RValue::Integer(n2)) => {
            Rc::new(RObject::float(n1 + *n2 as f64))
        }
        (RValue::String(n1), RValue::String(n2)) => {
            let mut n1 = n1.borrow_mut();
            let n2 = n2.borrow();
            for c in n2.iter() {
                n1.push(*c);
            }
            val1.clone()
        }
        _ => {
            let args = vec![val2.clone()];
            mrb_funcall(vm, Some(val1.clone()), "+", &args).expect("cannot call + for this receiver")
        }
    };
    vm.current_regs()[a].replace(result);
}

pub(crate) fn op_addi(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val1 = vm.current_regs()[a as usize].take().unwrap();
    let val2 = b as i64;
    let result = match &val1.value {
        RValue::Integer(n1) => {
            RObject::integer(*n1 + val2)
        }
        _ => {
            unreachable!("addi supports only integer")
        }
    };
    vm.current_regs()[a as usize].replace(Rc::new(result));
}

pub(crate) fn op_sub(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.current_regs()[a].take().unwrap();
    let val2 = vm.current_regs()[b].take().unwrap();
    let result = match (&val1.value, &val2.value) {
        (RValue::Integer(n1), RValue::Integer(n2)) => {
            RObject::integer(n1 - n2)
        }
        _ => {
            unreachable!("sub supports only integer")
        }
    };
    vm.current_regs()[a].replace(Rc::new(result));
}

pub(crate) fn op_subi(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val1 = vm.current_regs()[a as usize].take().unwrap();
    let val2 = b as i64;
    let result = match &val1.value {
        RValue::Integer(n1) => {
            RObject::integer(*n1 - val2)
        }
        _ => {
            unreachable!("subi supports only integer")
        }
    };
    vm.current_regs()[a as usize].replace(Rc::new(result));
}

pub(crate) fn op_mul(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.current_regs()[a].take().unwrap();
    let val2 = vm.current_regs()[b].take().unwrap();
    let result = match (&val1.value, &val2.value) {
        (RValue::Integer(n1), RValue::Integer(n2)) => {
            RObject::integer(n1 * n2)
        }
        _ => {
            unreachable!("mul supports only integer")
        }
    };
    vm.current_regs()[a].replace(Rc::new(result));
}

pub(crate) fn op_div(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.current_regs()[a].take().unwrap();
    let val2 = vm.current_regs()[b].take().unwrap();
    let result = match (&val1.value, &val2.value) {
        (RValue::Integer(n1), RValue::Integer(n2)) => {
            RObject::integer(n1 / n2)
        }
        _ => {
            unreachable!("div supports only integer")
        }
    };
    vm.current_regs()[a].replace(Rc::new(result));
}

pub(crate) fn op_lt(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.current_regs()[a].take().unwrap();
    let val2 = vm.current_regs()[b].take().unwrap();
    let result = match (&val1.value, &val2.value) {
        (RValue::Integer(n1), RValue::Integer(n2)) => {
            RObject::boolean(n1 < n2)
        }
        _ => {
            unreachable!("lt supports only integer")
        }
    };
    vm.current_regs()[a].replace(Rc::new(result));
}

pub(crate) fn op_le(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.current_regs()[a].take().unwrap();
    let val2 = vm.current_regs()[b].take().unwrap();
    let result = match (&val1.value, &val2.value) {
        (RValue::Integer(n1), RValue::Integer(n2)) => {
            RObject::boolean(n1 <= n2)
        }
        _ => {
            unreachable!("le supports only integer")
        }
    };
    vm.current_regs()[a].replace(Rc::new(result));
}

pub(crate) fn op_eq(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let lhs = vm.current_regs()[a].take().unwrap();
    let rhs = vm.current_regs()[b].take().unwrap();
    let result = mrb_object_is_equal(vm, lhs, rhs);
    vm.current_regs()[a].replace(result);
}

pub(crate) fn op_gt(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.current_regs()[a].take().unwrap();
    let val2 = vm.current_regs()[b].take().unwrap();
    let result = match (&val1.value, &val2.value) {
        (RValue::Integer(n1), RValue::Integer(n2)) => {
            RObject::boolean(n1 > n2)
        }
        _ => {
            unreachable!("gt supports only integer")
        }
    };
    vm.current_regs()[a].replace(Rc::new(result));
}

pub(crate) fn op_ge(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.current_regs()[a].take().unwrap();
    let val2 = vm.current_regs()[b].take().unwrap();
    let result = match (&val1.value, &val2.value) {
        (RValue::Integer(n1), RValue::Integer(n2)) => {
            RObject::boolean(n1 >= n2)
        }
        _ => {
            unreachable!("ge supports only integer")
        }
    };
    vm.current_regs()[a].replace(Rc::new(result));
}

pub(crate) fn op_array(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    do_op_array(vm, a as usize, a as usize, b as usize);
}

pub(crate) fn op_array2(vm: &mut VM, operand: &Fetched) {
    let (a, b, c) = operand.as_bbb().unwrap();
    do_op_array(vm, a as usize, b as usize, c as usize);
}

fn do_op_array(vm: &mut VM, this: usize, start: usize, n: usize) {
    let mut ary = Vec::with_capacity(n);
    for i in 0..n {
        if this == start && i == 0 {
            ary.push(vm.current_regs()[start as usize].take().unwrap());
        } else {
            ary.push(vm.current_regs()[(start + i) as usize].clone().unwrap());
        }
    }
    let val = RObject::array(ary);
    vm.current_regs()[this].replace(Rc::new(val));
}

pub(crate) fn op_symbol(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let symstr = vm.current_irep.pool[b as usize].as_str().to_string();
    let sym = RSym::new(symstr);
    let val = RObject::symbol(sym);
    vm.current_regs()[a as usize].replace(Rc::new(val));
}

pub(crate) fn op_string(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let str = vm.current_irep.pool[b as usize].as_str().to_string();
    let val = RObject::string(str);
    vm.current_regs()[a as usize].replace(Rc::new(val));
}

pub(crate) fn op_strcat(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let b = a + 1;
    let val1 = vm.current_regs()[a].clone().unwrap();
    let val2 = vm.current_regs()[b].clone().unwrap();
    match (&val1.value, &val2.value) {
        (RValue::String(s1), RValue::String(s2)) => {
            let mut s1 = s1.borrow_mut();
            let s2 = s2.borrow();
            for c in s2.iter() {
                s1.push(*c);
            }
        }
        (RValue::String(s1), RValue::Integer(s2)) => {
            let mut s1 = s1.borrow_mut();
            let s2 = s2.to_string();
            for c in s2.as_bytes() {
                s1.push(*c);
            }
        }
        _ => {
            unreachable!("strcat supports only string")
        }
    };
}

pub(crate) fn op_hash(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let a = a as usize;
    let b = b as usize;
    let mut hash = HashMap::new();
    for i in 0..b {
        let key = vm.current_regs()[a + i * 2].clone().unwrap();
        let val = vm.current_regs()[a + i * 2 + 1].clone().unwrap();
        hash.insert(key.as_hash_key().expect("Not a hash key type"), (key, val));
    }
    let val = RObject::hash(hash);
    vm.current_regs()[a as usize].replace(Rc::new(val));
}

pub(crate) fn op_lambda(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let irep = Some(vm.current_irep.reps[b as usize].clone());
    let environ = ENV {
        upper: vm.upper.clone(),
        current_regs_offset: vm.current_regs_offset,
        is_expired: Cell::new(false),
        captured: RefCell::new(None),
    };
    let nregs = vm.current_irep.nregs;
    environ.capture(&vm.current_regs()[0..nregs]);
    let environ = Rc::new(environ);
    vm.cur_env.insert(vm.current_irep.__id, environ.clone());
    vm.has_env_ref.insert(vm.current_irep.__id,true);

    let val = RObject {
        tt: RType::Proc,
        value: RValue::Proc(RProc {
            irep,
            is_rb_func: true,
            sym_id: Some("<lambda>".into()),
            next: None,
            func: None,
            environ: Some(environ),
            block_self: Some(vm.getself()),
        }),
        object_id: u64::MAX.into(),
    };
    vm.current_regs()[a as usize].replace(val.to_refcount_assigned());
}

pub(crate) fn op_block(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let irep = Some(vm.current_irep.reps[b as usize].clone());
    let environ = ENV {
        upper: vm.upper.clone(),
        current_regs_offset: vm.current_regs_offset,
        is_expired: Cell::new(false),
        captured: RefCell::new(None),
    };
    let environ = Rc::new(environ);
    vm.cur_env.insert(vm.current_irep.__id, environ.clone());
    vm.has_env_ref.insert(vm.current_irep.__id,true);

    let val = RObject {
        tt: RType::Proc,
        value: RValue::Proc(RProc {
            irep,
            is_rb_func: true,
            sym_id: Some("<block>".into()),
            next: None,
            func: None,
            environ: Some(environ),
            block_self: Some(vm.getself()),
        }),
        object_id: u64::MAX.into(),
    };
    vm.current_regs()[a as usize].replace(val.to_refcount_assigned());
}

pub(crate) fn op_method(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let irep = Some(vm.current_irep.reps[b as usize].clone());
    let val = RObject {
        tt: super::value::RType::Proc,
        value: super::value::RValue::Proc(super::value::RProc {
            irep,
            is_rb_func: true,
            sym_id: None,
            next: None,
            func: None,
            environ: None,
            block_self: None,
        }),
        object_id: u64::MAX.into(),
    };
    vm.current_regs()[a as usize].replace(val.to_refcount_assigned());
}

pub(crate) fn op_range_inc(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap();
    do_op_range(vm, a as usize, a as usize + 1, false);
}

pub(crate) fn op_range_exc(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap();
    do_op_range(vm, a as usize, a as usize + 1, true);
}

fn do_op_range(vm: &mut VM, a: usize, b: usize, exclusive: bool) {
    let val1 = vm.current_regs()[a].clone().unwrap();
    let val2 = vm.current_regs()[b].clone().unwrap();
    let val = RObject {
        tt: super::value::RType::Range,
        value: super::value::RValue::Range(val1, val2, exclusive),
        object_id: u64::MAX.into(),
    };
    vm.current_regs()[a as usize].replace(val.to_refcount_assigned());
}

pub(crate) fn op_oclass(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let val = vm.object_class.clone().into();
    vm.current_regs()[a].replace(Rc::new(val));
}

pub(crate) fn op_class(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let superclass = vm.current_regs()[a as usize + 1].as_ref().cloned();
    let name = vm.current_irep.syms[b as usize].clone();
    let superclass = match superclass {
        Some(superclass) => {
            if let RValue::Class(klass) = &superclass.value {
                klass.clone()
            } else {
                vm.object_class.clone()
            }
        }
        None => {
            vm.object_class.clone()
        }
    };
    let name = name.name;
    let klass = vm.define_class(&name, Some(superclass));

    vm.current_regs()[a as usize].replace(Rc::new(klass.into()));
}

pub(crate) fn op_exec(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let recv = vm.current_regs()[a as usize].as_ref().cloned().unwrap();

    vm.current_regs()[a as usize].replace(recv.clone());
    push_callinfo(vm, "<exec>".into(), 0);

    vm.pc.set(0);
    let irep = vm.irep.reps[b as usize].clone();
    vm.current_irep = irep;
    vm.current_regs_offset += a as usize;
    vm.target_class = recv.get_class(vm);
}

pub(crate) fn op_def(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let klass = vm.current_regs()[a as usize].as_ref().cloned().unwrap();
    let method = vm.current_regs()[(a + 1) as usize].as_ref().cloned().unwrap();
    let sym = vm.current_irep.syms[b as usize].clone();

    let klass = klass.as_ref();
    let method = method.as_ref();
    if let (RValue::Class(klass), RValue::Proc(method)) = (&klass.value, &method.value) {
        let mut procs = klass.procs.borrow_mut();
        let mut method = method.clone();
        method.sym_id = Some(sym.clone());
        procs.insert(sym.name.clone(), method);
    } else {
        unreachable!("DEF must be called on class");
    }
    vm.current_regs()[a as usize].replace(RObject::symbol(sym).to_refcount_assigned());
}

pub(crate) fn op_tclass(vm: &mut VM, operand: &Fetched) {
    let a = operand.as_b().unwrap() as usize;
    let klass = vm.object_class.clone();
    let val: RObject = klass.into();
    vm.current_regs()[a].replace(val.to_refcount_assigned());
} 

pub(crate) fn op_stop(vm: &mut VM, _operand: &Fetched) {
    vm.flag_preemption.set(true);
}