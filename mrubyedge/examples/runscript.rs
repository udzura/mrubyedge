use std::env;
use std::fs::{remove_file, File};
use std::io::Read;
use std::process::Command;

extern crate mrubyedge;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0];
    let is_verbose = env::var("MRUBYEDGE_DEBUG").is_ok();

    let mut mrbc = Command::new("mrbc");
    if is_verbose {
        mrbc.arg("-v");
    }
    let result = mrbc.arg("-o")
        .arg("/tmp/__tmp__.mrb")
        .arg(path)
        .output()
        .expect("failed to compile mruby script");
    if is_verbose {
        eprintln!("stdout: {}", String::from_utf8_lossy(&result.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&result.stderr));
    }

    let mut file = File::open("/tmp/__tmp__.mrb")?;
    let mut bin = Vec::<u8>::new();
    file.read_to_end(&mut bin)?;

    let mut rite = mrubyedge::rite::load(&bin).unwrap();
    // dbg!(&rite);
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    // dbg!(&vm.irep.reps);
    let res = vm.run().unwrap();
    remove_file("/tmp/__tmp__.mrb")?;

    eprintln!("return value: {:?}", res);
    // dbg!(&vm);
    Ok(())
}
