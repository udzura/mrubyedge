use std::fmt::Debug;

use crate::rite::Error;

#[derive(Copy, Clone, Debug)]
pub enum Fetched {
    Z,
    B(u8),
    BB(u8, u8),
    BBB(u8, u8, u8),
    BS(u8, u16),
    BSS(u8, u16, u16),
    S(u16),
    W(u32), // u24 in real layout
}

type FetchResult<Res> = Result<Res, Error>;

impl Fetched {
    pub fn as_z(self) -> FetchResult<()> {
        match self {
            Fetched::Z => Ok(()),
            _ => Err(Error::InvalidOperand),
        }
    }

    pub fn as_b(self) -> FetchResult<u8> {
        match self {
            Fetched::B(a) => Ok(a),
            _ => Err(Error::InvalidOperand),
        }
    }

    pub fn as_bb(self) -> FetchResult<(u8, u8)> {
        match self {
            Fetched::BB(a, b) => Ok((a, b)),
            _ => Err(Error::InvalidOperand),
        }
    }

    pub fn as_bbb(self) -> FetchResult<(u8, u8, u8)> {
        match self {
            Fetched::BBB(a, b, c) => Ok((a, b, c)),
            _ => Err(Error::InvalidOperand),
        }
    }

    pub fn as_bs(self) -> FetchResult<(u8, u16)> {
        match self {
            Fetched::BS(a, b) => Ok((a, b)),
            _ => Err(Error::InvalidOperand),
        }
    }

    pub fn as_bss(self) -> FetchResult<(u8, u16, u16)> {
        match self {
            Fetched::BSS(a, b, c) => Ok((a, b, c)),
            _ => Err(Error::InvalidOperand),
        }
    }

    pub fn as_s(self) -> FetchResult<u16> {
        match self {
            Fetched::S(s) => Ok(s),
            _ => Err(Error::InvalidOperand),
        }
    }

    pub fn as_w(self) -> FetchResult<u32> {
        match self {
            Fetched::W(w) => Ok(w),
            _ => Err(Error::InvalidOperand),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Fetched::Z => 0,
            Fetched::B(_) => 1,
            Fetched::BB(_, _) => 2,
            Fetched::BBB(_, _, _) => 3,
            Fetched::BS(_, _) => 3,
            Fetched::BSS(_, _, _) => 5,
            Fetched::S(_) => 2,
            Fetched::W(_) => 3,
        }
    }
}

// from mruby 3.2.0 op.h
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
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum OpCode {
    NOP,
    MOVE,
    LOADL,
    LOADI,
    LOADINEG,
    LOADI__1,
    LOADI_0,
    LOADI_1,
    LOADI_2,
    LOADI_3,
    LOADI_4,
    LOADI_5,
    LOADI_6,
    LOADI_7,
    LOADI16,
    LOADI32,
    LOADSYM,
    LOADNIL,
    LOADSELF,
    LOADT,
    LOADF,
    GETGV,
    SETGV,
    GETSV,
    SETSV,
    GETIV,
    SETIV,
    GETCV,
    SETCV,
    GETCONST,
    SETCONST,
    GETMCNST,
    SETMCNST,
    GETUPVAR,
    SETUPVAR,
    GETIDX,
    SETIDX,
    JMP,
    JMPIF,
    JMPNOT,
    JMPNIL,
    JMPUW,
    EXCEPT,
    RESCUE,
    RAISEIF,
    SSEND,
    SSENDB,
    SEND,
    SENDB,
    CALL,
    SUPER,
    ARGARY,
    ENTER,
    KEY_P,
    KEYEND,
    KARG,
    RETURN,
    RETURN_BLK,
    BREAK,
    BLKPUSH,
    ADD,
    ADDI,
    SUB,
    SUBI,
    MUL,
    DIV,
    EQ,
    LT,
    LE,
    GT,
    GE,
    ARRAY,
    ARRAY2,
    ARYCAT,
    ARYPUSH,
    ARYSPLAT,
    AREF,
    ASET,
    APOST,
    INTERN,
    SYMBOL,
    STRING,
    STRCAT,
    HASH,
    HASHADD,
    HASHCAT,
    LAMBDA,
    BLOCK,
    METHOD,
    RANGE_INC,
    RANGE_EXC,
    OCLASS,
    CLASS,
    MODULE,
    EXEC,
    DEF,
    ALIAS,
    UNDEF,
    SCLASS,
    TCLASS,
    DEBUG,
    ERR,
    EXT1,
    EXT2,
    EXT3,
    STOP,

    NumberOfOpcode, // for fetcher table
}

use self::OpCode::*;
const ENUM_TABLE: [OpCode; OpCode::NumberOfOpcode as usize] = [
    NOP, MOVE, LOADL, LOADI, LOADINEG, LOADI__1, LOADI_0, LOADI_1, LOADI_2, LOADI_3, LOADI_4,
    LOADI_5, LOADI_6, LOADI_7, LOADI16, LOADI32, LOADSYM, LOADNIL, LOADSELF, LOADT, LOADF, GETGV,
    SETGV, GETSV, SETSV, GETIV, SETIV, GETCV, SETCV, GETCONST, SETCONST, GETMCNST, SETMCNST,
    GETUPVAR, SETUPVAR, GETIDX, SETIDX, JMP, JMPIF, JMPNOT, JMPNIL, JMPUW, EXCEPT, RESCUE, RAISEIF,
    SSEND, SSENDB, SEND, SENDB, CALL, SUPER, ARGARY, ENTER, KEY_P, KEYEND, KARG, RETURN,
    RETURN_BLK, BREAK, BLKPUSH, ADD, ADDI, SUB, SUBI, MUL, DIV, EQ, LT, LE, GT, GE, ARRAY, ARRAY2,
    ARYCAT, ARYPUSH, ARYSPLAT, AREF, ASET, APOST, INTERN, SYMBOL, STRING, STRCAT, HASH, HASHADD,
    HASHCAT, LAMBDA, BLOCK, METHOD, RANGE_INC, RANGE_EXC, OCLASS, CLASS, MODULE, EXEC, DEF, ALIAS,
    UNDEF, SCLASS, TCLASS, DEBUG, ERR, EXT1, EXT2, EXT3, STOP,
];

impl TryFrom<u8> for OpCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=105 => Ok(ENUM_TABLE[value as usize]),
            _ => Err(Error::InvalidOpCode),
        }
    }
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOP => write!(f, "NOP"),
            Self::MOVE => write!(f, "MOVE"),
            Self::LOADL => write!(f, "LOADL"),
            Self::LOADI => write!(f, "LOADI"),
            Self::LOADINEG => write!(f, "LOADINEG"),
            Self::LOADI__1 => write!(f, "LOADI__1"),
            Self::LOADI_0 => write!(f, "LOADI_0"),
            Self::LOADI_1 => write!(f, "LOADI_1"),
            Self::LOADI_2 => write!(f, "LOADI_2"),
            Self::LOADI_3 => write!(f, "LOADI_3"),
            Self::LOADI_4 => write!(f, "LOADI_4"),
            Self::LOADI_5 => write!(f, "LOADI_5"),
            Self::LOADI_6 => write!(f, "LOADI_6"),
            Self::LOADI_7 => write!(f, "LOADI_7"),
            Self::LOADI16 => write!(f, "LOADI16"),
            Self::LOADI32 => write!(f, "LOADI32"),
            Self::LOADSYM => write!(f, "LOADSYM"),
            Self::LOADNIL => write!(f, "LOADNIL"),
            Self::LOADSELF => write!(f, "LOADSELF"),
            Self::LOADT => write!(f, "LOADT"),
            Self::LOADF => write!(f, "LOADF"),
            Self::GETGV => write!(f, "GETGV"),
            Self::SETGV => write!(f, "SETGV"),
            Self::GETSV => write!(f, "GETSV"),
            Self::SETSV => write!(f, "SETSV"),
            Self::GETIV => write!(f, "GETIV"),
            Self::SETIV => write!(f, "SETIV"),
            Self::GETCV => write!(f, "GETCV"),
            Self::SETCV => write!(f, "SETCV"),
            Self::GETCONST => write!(f, "GETCONST"),
            Self::SETCONST => write!(f, "SETCONST"),
            Self::GETMCNST => write!(f, "GETMCNST"),
            Self::SETMCNST => write!(f, "SETMCNST"),
            Self::GETUPVAR => write!(f, "GETUPVAR"),
            Self::SETUPVAR => write!(f, "SETUPVAR"),
            Self::GETIDX => write!(f, "GETIDX"),
            Self::SETIDX => write!(f, "SETIDX"),
            Self::JMP => write!(f, "JMP"),
            Self::JMPIF => write!(f, "JMPIF"),
            Self::JMPNOT => write!(f, "JMPNOT"),
            Self::JMPNIL => write!(f, "JMPNIL"),
            Self::JMPUW => write!(f, "JMPUW"),
            Self::EXCEPT => write!(f, "EXCEPT"),
            Self::RESCUE => write!(f, "RESCUE"),
            Self::RAISEIF => write!(f, "RAISEIF"),
            Self::SSEND => write!(f, "SSEND"),
            Self::SSENDB => write!(f, "SSENDB"),
            Self::SEND => write!(f, "SEND"),
            Self::SENDB => write!(f, "SENDB"),
            Self::CALL => write!(f, "CALL"),
            Self::SUPER => write!(f, "SUPER"),
            Self::ARGARY => write!(f, "ARGARY"),
            Self::ENTER => write!(f, "ENTER"),
            Self::KEY_P => write!(f, "KEY_P"),
            Self::KEYEND => write!(f, "KEYEND"),
            Self::KARG => write!(f, "KARG"),
            Self::RETURN => write!(f, "RETURN"),
            Self::RETURN_BLK => write!(f, "RETURN_BLK"),
            Self::BREAK => write!(f, "BREAK"),
            Self::BLKPUSH => write!(f, "BLKPUSH"),
            Self::ADD => write!(f, "ADD"),
            Self::ADDI => write!(f, "ADDI"),
            Self::SUB => write!(f, "SUB"),
            Self::SUBI => write!(f, "SUBI"),
            Self::MUL => write!(f, "MUL"),
            Self::DIV => write!(f, "DIV"),
            Self::EQ => write!(f, "EQ"),
            Self::LT => write!(f, "LT"),
            Self::LE => write!(f, "LE"),
            Self::GT => write!(f, "GT"),
            Self::GE => write!(f, "GE"),
            Self::ARRAY => write!(f, "ARRAY"),
            Self::ARRAY2 => write!(f, "ARRAY2"),
            Self::ARYCAT => write!(f, "ARYCAT"),
            Self::ARYPUSH => write!(f, "ARYPUSH"),
            Self::ARYSPLAT => write!(f, "ARYSPLAT"),
            Self::AREF => write!(f, "AREF"),
            Self::ASET => write!(f, "ASET"),
            Self::APOST => write!(f, "APOST"),
            Self::INTERN => write!(f, "INTERN"),
            Self::SYMBOL => write!(f, "SYMBOL"),
            Self::STRING => write!(f, "STRING"),
            Self::STRCAT => write!(f, "STRCAT"),
            Self::HASH => write!(f, "HASH"),
            Self::HASHADD => write!(f, "HASHADD"),
            Self::HASHCAT => write!(f, "HASHCAT"),
            Self::LAMBDA => write!(f, "LAMBDA"),
            Self::BLOCK => write!(f, "BLOCK"),
            Self::METHOD => write!(f, "METHOD"),
            Self::RANGE_INC => write!(f, "RANGE_INC"),
            Self::RANGE_EXC => write!(f, "RANGE_EXC"),
            Self::OCLASS => write!(f, "OCLASS"),
            Self::CLASS => write!(f, "CLASS"),
            Self::MODULE => write!(f, "MODULE"),
            Self::EXEC => write!(f, "EXEC"),
            Self::DEF => write!(f, "DEF"),
            Self::ALIAS => write!(f, "ALIAS"),
            Self::UNDEF => write!(f, "UNDEF"),
            Self::SCLASS => write!(f, "SCLASS"),
            Self::TCLASS => write!(f, "TCLASS"),
            Self::DEBUG => write!(f, "DEBUG"),
            Self::ERR => write!(f, "ERR"),
            Self::EXT1 => write!(f, "EXT1"),
            Self::EXT2 => write!(f, "EXT2"),
            Self::EXT3 => write!(f, "EXT3"),
            Self::STOP => write!(f, "STOP"),
            Self::NumberOfOpcode => write!(f, "[BUG] overflow opcode"),
        }
    }
}

fn fetch_z(bin: &mut &[u8]) -> Result<Fetched, Error> {
    if bin.len() < 1 {
        return Err(Error::TooShort);
    }
    *bin = &bin[1..];
    Ok(Fetched::Z)
}
fn fetch_b(bin: &mut &[u8]) -> Result<Fetched, Error> {
    if bin.len() < 2 {
        return Err(Error::TooShort);
    }
    let a = bin[1];
    let operand = Fetched::B(a);

    *bin = &bin[2..];
    Ok(operand)
}
fn fetch_bb(bin: &mut &[u8]) -> Result<Fetched, Error> {
    if bin.len() < 3 {
        return Err(Error::TooShort);
    }
    let a = bin[1];
    let b = bin[2];
    let operand = Fetched::BB(a, b);

    *bin = &bin[3..];
    Ok(operand)
}
fn fetch_bbb(bin: &mut &[u8]) -> Result<Fetched, Error> {
    if bin.len() < 4 {
        return Err(Error::TooShort);
    }
    let a = bin[1];
    let b = bin[2];
    let c = bin[3];
    let operand = Fetched::BBB(a, b, c);

    *bin = &bin[4..];
    Ok(operand)
}
fn fetch_bs(bin: &mut &[u8]) -> Result<Fetched, Error> {
    if bin.len() < 4 {
        return Err(Error::TooShort);
    }
    let a = bin[1];
    let s = ((bin[2] as u16) << 8) | bin[3] as u16;
    let operand = Fetched::BS(a, s);

    *bin = &bin[4..];
    Ok(operand)
}
fn fetch_bss(bin: &mut &[u8]) -> Result<Fetched, Error> {
    if bin.len() < 6 {
        return Err(Error::TooShort);
    }
    let a = bin[1];
    let s1 = ((bin[2] as u16) << 8) | bin[3] as u16;
    let s2 = ((bin[2] as u16) << 8) | bin[5] as u16;
    let operand = Fetched::BSS(a, s1, s2);

    *bin = &bin[6..];
    Ok(operand)
}
fn fetch_s(bin: &mut &[u8]) -> Result<Fetched, Error> {
    if bin.len() < 3 {
        return Err(Error::TooShort);
    }
    let s = ((bin[1] as u16) << 8) | bin[2] as u16;
    let operand = Fetched::S(s);

    *bin = &bin[3..];
    Ok(operand)
}
fn fetch_w(bin: &mut &[u8]) -> Result<Fetched, Error> {
    if bin.len() < 4 {
        return Err(Error::TooShort);
    }
    let w = ((bin[1] as u32) << 16) | ((bin[2] as u32) << 8) | bin[3] as u32;
    let operand = Fetched::W(w);

    *bin = &bin[4..];
    Ok(operand)
}

const Z: fn(&mut &[u8]) -> Result<Fetched, Error> = fetch_z;
const B: fn(&mut &[u8]) -> Result<Fetched, Error> = fetch_b;
const BB: fn(&mut &[u8]) -> Result<Fetched, Error> = fetch_bb;
const BBB: fn(&mut &[u8]) -> Result<Fetched, Error> = fetch_bbb;
const BS: fn(&mut &[u8]) -> Result<Fetched, Error> = fetch_bs;
const BSS: fn(&mut &[u8]) -> Result<Fetched, Error> = fetch_bss;
const S: fn(&mut &[u8]) -> Result<Fetched, Error> = fetch_s;
const W: fn(&mut &[u8]) -> Result<Fetched, Error> = fetch_w;

pub const FETCH_TABLE: [fn(&mut &[u8]) -> Result<Fetched, Error>; OpCode::NumberOfOpcode as usize] = [
    Z, BB, BB, BB, BB, B, B, B, B, B, B, B, B, B, BS, BSS, BB, B, B, B, B, BB, BB, BB, BB, BB, BB,
    BB, BB, BB, BB, BB, BB, BBB, BBB, B, B, S, BS, BS, BS, S, B, BB, B, BBB, BBB, BBB, BBB, Z, BB,
    BS, W, BB, Z, BB, B, B, B, BS, B, BB, B, BB, B, B, B, B, B, B, B, BB, BBB, B, BB, B, BBB, BBB,
    BBB, B, BB, BB, B, BB, BB, B, BB, BB, BB, B, B, B, BB, BB, BB, BB, BB, B, B, B, BBB, B, Z, Z,
    Z, Z,
];
