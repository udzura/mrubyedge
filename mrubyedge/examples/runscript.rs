use std::env;
use std::fs::File;
use std::io::Read;
use std::process::Command;

extern crate mrubyedge;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0];

    let output = Command::new("mrbc")
        .arg("-v")
        .arg(path)
        .output()
        .expect("failed to compile mruby script");
    eprintln!("compiled: {}", String::from_utf8_lossy(&output.stdout));

    let path = path.strip_suffix(".rb").unwrap();
    let mut file = File::open(&format!("{}.mrb", path))?;
    let mut bin = Vec::<u8>::new();
    file.read_to_end(&mut bin)?;

    let rite = mrubyedge::rite::load(&bin).unwrap();
    // dbg!(&rite);
    let mut vm = mrubyedge::vm::VM::open(rite);
    vm.prelude().unwrap();
    // dbg!(&vm);
    vm.eval_insn().unwrap();

    eprintln!("return value:");
    let top = 0 as usize;
    match vm.regs.get(&top) {
        Some(v) => {
            eprintln!("{:?}", v);
            // eprintln!("{:?}", TryInto::<i32>::try_into(v.as_ref()).unwrap());
        }
        None => eprintln!("None"),
    }

    //   dbg!(&vm);
    Ok(())
}
