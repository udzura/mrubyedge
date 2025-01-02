use std::env;
use std::fs::{remove_file, File};
use std::io::Read;
use std::process::Command;

extern crate mrubyedge;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0];

    let output = Command::new("mrbc")
        .arg("-v")
        .arg("-o")
        .arg("/tmp/__tmp__.mrb")
        .arg(path)
        .output()
        .expect("failed to compile mruby script");
    eprintln!("debug: {}", String::from_utf8_lossy(&output.stdout));

    let mut file = File::open("/tmp/__tmp__.mrb")?;
    let mut bin = Vec::<u8>::new();
    file.read_to_end(&mut bin)?;

    let mut rite = mrubyedge::rite::load(&bin).unwrap();
    // dbg!(&rite);
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    // dbg!(&vm.irep.reps);
    let res = vm.run().unwrap();
    remove_file("/tmp/__tmp__.mrb")?;

    eprintln!("return value:");
    eprintln!("{:?}", res);
    // dbg!(&vm);
    Ok(())
}
