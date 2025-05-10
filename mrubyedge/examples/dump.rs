use std::env;
use std::fs::File;
use std::io::Read;

extern crate mrubyedge;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0];

    let mut file = File::open(path)?;
    let mut bin = Vec::<u8>::new();
    file.read_to_end(&mut bin)?;

    let mut rite = mrubyedge::rite::load(&bin).unwrap();
    dbg!(&rite);
    let vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    dbg!(&vm.irep);
    Ok(())
}
