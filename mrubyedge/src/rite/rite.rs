extern crate simple_endian;

use super::binfmt::*;
use super::Error;

use core::ascii;
use core::ffi::CStr;
use core::mem;

use super::insn::{self, OpCode};
use super::marker::*;

use simple_endian::{u16be, u32be};

#[derive(Debug)]
pub struct Rite<'a> {
    pub binary_header: &'a RiteBinaryHeader,
    pub irep_header: &'a SectionIrepHeader,
    pub irep: Vec<Irep<'a>>,
    pub lvar: LVar<'a>,
}

#[derive(Debug)]
pub struct Irep<'a> {
    pub header: &'a IrepRecord,
    pub insn: &'a [u8],
}

#[derive(Debug)]
pub struct LVar<'a> {
    pub header: &'a SectionMiscHeader,
}

pub fn load<'a>(src: &'a [u8]) -> Result<Rite<'a>, Error> {
    let mut size = src.len();
    let mut head = src;
    let binheader_size = mem::size_of::<RiteBinaryHeader>();
    if size < binheader_size {
        return Err(Error::TooShort);
    }
    let bin_header = RiteBinaryHeader::from_bytes(&head[0..binheader_size])?;
    size -= binheader_size;
    head = &head[binheader_size..];

    // dbg!(bin_header);
    let binsize: u32 = be32_to_u32(bin_header.size);
    eprintln!("size {}", binsize);

    let irep_header_size = mem::size_of::<SectionIrepHeader>();
    if size < irep_header_size {
        return Err(Error::TooShort);
    }

    let mut irep_header: &SectionIrepHeader = &SectionIrepHeader::default();
    let mut irep: Vec<Irep> = Vec::default();
    let mut lvar: LVar = LVar {
        header: &SectionMiscHeader::default(),
    };
    loop {
        match peek4(head) {
            Some(chrs) => match chrs {
                IREP => {
                    let (cur, irep_header_, irep_) = section_irep_1(head)?;
                    irep_header = irep_header_;
                    irep = irep_;
                    head = &head[cur..];
                }
                LVAR => {
                    let (cur, lvar_) = section_lvar(head)?;
                    lvar = lvar_;
                    head = &head[cur..];
                }
                DBG => {
                    let cur = section_skip(head)?;
                    head = &head[cur..];
                }
                END => {
                    let cur = section_end(head)?;
                    head = &head[cur..];
                }
                _ => {
                    dbg!(chrs);
                    dbg!(head);
                    return Err(Error::InvalidFormat);
                }
            },
            None => {
                break;
            }
        }
    }

    let rite = Rite {
        binary_header: bin_header,
        irep_header,
        irep,
        lvar,
    };

    Ok(rite)
}

pub fn section_irep_1(head: &[u8]) -> Result<(usize, &SectionIrepHeader, Vec<Irep>), Error> {
    let mut cur = 0;

    let irep_header_size = mem::size_of::<SectionIrepHeader>();
    let irep_header = SectionIrepHeader::from_bytes(&head[cur..irep_header_size])?;
    let irep_size = be32_to_u32(irep_header.size) as usize;
    if head.len() < irep_size {
        return Err(Error::TooShort);
    }
    cur += irep_header_size;

    let mut ireps: Vec<Irep> = Vec::new();

    while cur < irep_size {
        let start_cur = cur;
        // insn
        let record_size = mem::size_of::<IrepRecord>();
        let irep_record = IrepRecord::from_bytes(&head[cur..cur + record_size])?;
        let irep_rec_size = be32_to_u32(irep_record.size) as usize;
        let ilen = be32_to_u32(irep_record.ilen) as usize;
        dbg!(ilen);
        cur += record_size;

        let mut insns = &head[cur..cur + ilen];
        let ps: usize = 0;
        while ps < insns.len() {
            let op = insns[ps];
            let opcode: OpCode = op.try_into()?;
            let fetched = insn::FETCH_TABLE[op as usize](&mut insns)?;
            println!("insn: {:?} {:?}", opcode, fetched);
        }
        // dbg!(insns);

        cur += ilen;

        // pool
        let data = &head[cur..cur + 2];
        let plen = be16_to_u16([data[0], data[1]]) as usize;
        cur += 2;
        dbg!(plen);
        for _ in 0..plen {
            let typ = head[cur];
            match typ {
                0 => {
                    cur += 1;
                    let data = &head[cur..cur + 2];
                    let strlen = be16_to_u16([data[0], data[1]]) as usize + 1;
                    cur += 2;
                    let strval = CStr::from_bytes_with_nul(&head[cur..cur + strlen])
                        .or(Err(Error::InvalidFormat))?;
                    dbg!(strval);
                    cur += strlen;
                }
                _ => {
                    unimplemented!("more support pool type");
                }
            }
        }

        // syms
        let data = &head[cur..cur + 2];
        let slen = be16_to_u16([data[0], data[1]]) as usize;
        cur += 2;
        dbg!(slen);
        for _ in 0..slen {
            let data = &head[cur..cur + 2];
            let symlen = be16_to_u16([data[0], data[1]]) as usize + 1;
            cur += 2;
            let symval = CStr::from_bytes_with_nul(&head[cur..cur + symlen])
                .or(Err(Error::InvalidFormat))?;
            dbg!(symval);
            cur += symlen;
        }

        cur = start_cur + irep_rec_size;
        dbg!(cur);
        dbg!(irep_size);

        let irep = Irep {
            header: irep_record,
            insn: insns,
        };
        ireps.push(irep);
    }

    Ok((irep_size, irep_header, ireps))
}

pub fn section_end(head: &[u8]) -> Result<usize, Error> {
    let header = SectionMiscHeader::from_bytes(head)?;
    eprintln!("end section detected");
    Ok(be32_to_u32(header.size) as usize)
}

pub fn section_lvar(head: &[u8]) -> Result<(usize, LVar), Error> {
    let header = SectionMiscHeader::from_bytes(head)?;
    let lvar = LVar { header };
    eprintln!("skipped section {:?}", header.ident.as_ascii());
    Ok((be32_to_u32(header.size) as usize, lvar))
}

pub fn section_skip(head: &[u8]) -> Result<usize, Error> {
    let header = SectionMiscHeader::from_bytes(head)?;
    eprintln!("skipped section {:?}", header.ident.as_ascii());
    Ok(be32_to_u32(header.size) as usize)
}

pub fn peek4<'a>(src: &'a [u8]) -> Option<[ascii::Char; 4]> {
    if src.len() < 4 {
        // EoD
        return None;
    }
    if let Some([a, b, c, d]) = src[0..4].as_ascii() {
        Some([*a, *b, *c, *d])
    } else {
        None
    }
}

pub fn be32_to_u32(be32: [u8; 4]) -> u32 {
    let binsize_be = unsafe { mem::transmute::<[u8; 4], u32be>(be32) };
    let binsize: u32 = binsize_be.into();
    binsize
}

pub fn be16_to_u16(be16: [u8; 2]) -> u16 {
    let binsize_be = unsafe { mem::transmute::<[u8; 2], u16be>(be16) };
    let binsize: u16 = binsize_be.into();
    binsize
}
