use clap::Args;
use std::{fs::File, io::Read, path::PathBuf};

use mruby_compiler2_sys as mrbc;

#[derive(Args)]
pub struct CompileMrbArgs {
    /// Output file path
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Dump instruction sequences
    #[arg(long)]
    pub dump_insns: bool,

    /// Skip generating mrb file
    #[arg(long)]
    pub skip_generate_mrb: bool,

    /// Ruby source file to compile
    pub file: PathBuf,
}

pub fn execute(args: CompileMrbArgs) -> Result<(), Box<dyn std::error::Error>> {
    let output = if let Some(output) = &args.output {
        output.clone()
    } else {
        args.file.with_extension("mrb")
    };

    let mut buf = Vec::new();
    File::open(&args.file)?.read_to_end(&mut buf)?;
    let buf = String::from_utf8(buf)?;
    unsafe {
        let mut ctx = mrbc::MRubyCompiler2Context::new();
        if args.dump_insns {
            ctx.dump_bytecode(&buf)?;
        }
        if !args.skip_generate_mrb {
            ctx.compile_to_file(&buf, &output)?;
        }
    }
    
    Ok(())
}
