use clap::Args;
use std::{fs::File, io::Read, path::PathBuf};

use mruby_compiler2_sys as mrbc;
use mrubyedge;

#[derive(Args)]
pub struct RunArgs {
    /// Dump instruction sequences
    #[arg(long)]
    pub dump_insns: bool,

    /// Ruby source file or mrb binary to run
    pub file: PathBuf,
}

pub fn execute(args: RunArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    File::open(&args.file)?.read_to_end(&mut buf)?;
    let is_mrb_direct = &buf[0..4] == &['R' as u8, 'I' as u8, 'T' as u8, 'E' as u8][..];
    unsafe {
        let mrb_bin = if is_mrb_direct {
            buf.to_vec()
        } else {
            let buf = String::from_utf8(buf)?;
            let mut ctx = mrbc::MRubyCompiler2Context::new();
            if args.dump_insns {
                ctx.dump_bytecode(&buf)?;
            }
            ctx.compile(&buf)?
        };
        let mut rite = mrubyedge::rite::load(&mrb_bin)?;
        let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
        vm.run()?;
    }

    Ok(())
}
