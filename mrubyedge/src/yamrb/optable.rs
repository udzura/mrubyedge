// use std::rc::Rc;

use std::rc::Rc;

use crate::rite::insn::{Fetched, OpCode};

use super::{value::{RObject, RValue}, vm::VM};

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

pub(crate) fn consume_expr(vm: &mut VM, code: OpCode, operand: &Fetched) {
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
        // LOADSYM => {
        //     // op_loadsym(vm, &operand);
        // }
        // LOADNIL => {
        //     // op_loadnil(vm, &operand);
        // }
        // LOADSELF => {
        //     // op_loadself(vm, &operand);
        // }
        // LOADT => {
        //     // op_loadt(vm, &operand);
        // }
        // LOADF => {
        //     // op_loadf(vm, &operand);
        // }
        // GETGV => {
        //     // op_getgv(vm, &operand);
        // }
        // SETGV => {
        //     // op_setgv(vm, &operand);
        // }
        // GETSV => {
        //     // op_getsv(vm, &operand);
        // }
        // SETSV => {
        //     // op_setsv(vm, &operand);
        // }
        // GETIV => {
        //     // op_getiv(vm, &operand);
        // }
        // SETIV => {
        //     // op_setiv(vm, &operand);
        // }
        // GETCV => {
        //     // op_getcv(vm, &operand);
        // }
        // SETCV => {
        //     // op_setcv(vm, &operand);
        // }
        // GETCONST => {
        //     // op_getconst(vm, &operand);
        // }
        // SETCONST => {
        //     // op_setconst(vm, &operand);
        // }
        // GETMCNST => {
        //     // op_getmcnst(vm, &operand);
        // }
        // SETMCNST => {
        //     // op_setmcnst(vm, &operand);
        // }
        // GETUPVAR => {
        //     // op_getupvar(vm, &operand);
        // }
        // SETUPVAR => {
        //     // op_setupvar(vm, &operand);
        // }
        // GETIDX => {
        //     // op_getidx(vm, &operand);
        // }
        // SETIDX => {
        //     // op_setidx(vm, &operand);
        // }
        // JMP => {
        //     // op_jmp(vm, &operand);
        // }
        // JMPIF => {
        //     // op_jmpif(vm, &operand);
        // }
        // JMPNOT => {
        //     // op_jmpnot(vm, &operand);
        // }
        // JMPNIL => {
        //     // op_jmpnil(vm, &operand);
        // }
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
        // SSEND => {
        //     // op_ssend(vm, &operand);
        // }
        // SSENDB => {
        //     // op_ssendb(vm, &operand);
        // }   
        // SEND => {
        //     // op_send(vm, &operand);
        // }
        // SENDB => {
        //     // op_sendb(vm, &operand);
        // }
        // CALL => {
        //     // op_call(vm, &operand);
        // }
        // SUPER => {
        //     // op_super(vm, &operand);
        // }
        // ARGARY => {
        //     // op_argary(vm, &operand);
        // }
        // ENTER => {
        //     // op_enter(vm, &operand);
        // }
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
        // ADDI => {
        //     // op_addi(vm, &operand);
        // }
        // SUB => {
        //     // op_sub(vm, &operand);
        // }
        // SUBI => {
        //     // op_subi(vm, &operand);
        // }
        // MUL => {
        //     // op_mul(vm, &operand);
        // }
        // DIV => {
        //     // op_div(vm, &operand);
        // }
        // EQ => {
        //     // op_eq(vm, &operand);
        // }
        // LT => {
        //     // op_lt(vm, &operand);
        // }
        // LE => {
        //     // op_le(vm, &operand);
        // }
        // GT => {
        //     // op_gt(vm, &operand);
        // }
        // GE => {
        //     // op_ge(vm, &operand);
        // }
        // ARRAY => {
        //     // op_array(vm, &operand);
        // }
        // ARRAY2 => {
        //     // op_array2(vm, &operand);
        // }
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
        // SYMBOL => {
        //     // op_symbol(vm, &operand);
        // }
        // STRING => {
        //     // op_string(vm, &operand);
        // }
        // STRCAT => {
        //     // op_strcat(vm, &operand);
        // }
        // HASH => {
        //     // op_hash(vm, &operand);
        // }
        // HASHADD => {
        //     // op_hashadd(vm, &operand);
        // }
        // HASHCAT => {
        //     // op_hashcat(vm, &operand);
        // }
        // LAMBDA => {
        //     // op_lambda(vm, &operand);
        // }
        // BLOCK => {
        //     // op_block(vm, &operand);
        // }
        // METHOD => {
        //     // op_method(vm, &operand);
        // }
        // RANGE_INC => {
        //     // op_range_inc(vm, &operand);
        // }
        // RANGE_EXC => {
        //     // op_range_exc(vm, &operand);
        // }
        // OCLASS => {
        //     // op_oclass(vm, &operand);
        // }
        // CLASS => {
        //     // op_class(vm, &operand);
        // }
        // MODULE => {
        //     // op_module(vm, &operand);
        // }
        // EXEC => {
        //     // op_exec(vm, &operand);
        // }
        // DEF => {
        //     // op_def(vm, &operand);
        // }
        // ALIAS => {
        //     // op_alias(vm, &operand);
        // }
        // UNDEF => {
        //     // op_undef(vm, &operand);
        // }
        // SCLASS => {
        //     // op_sclass(vm, &operand);
        // }
        // TCLASS => {
        //     // op_tclass(vm, &operand);
        // }
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

pub(crate) fn op_loadl(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = vm.current_irep.pool[b as usize].clone();
    vm.regs[a as usize].replace(val);
}

pub(crate) fn op_loadi(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = RObject::integer(b as i64);
    vm.regs[a as usize].replace(Rc::new(val));
}

pub(crate) fn op_loadineg(vm: &mut VM, operand: &Fetched) {
    let (a, b) = operand.as_bb().unwrap();
    let val = RObject::integer(-(b as i64));
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