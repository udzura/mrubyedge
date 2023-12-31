use crate::rite::insn::{self, OpCode};

pub fn debug_eval_insn(mut insns: &[u8]) -> Result<(), crate::rite::Error> {
    let ps: usize = 0;
    while insns.len() > 0 {
        let op = insns[ps];
        let opcode: OpCode = op.try_into()?;
        let fetched = insn::FETCH_TABLE[op as usize](&mut insns)?;
        println!("insn: {:?} {:?}", opcode, fetched);
    }
    Ok(())
}
