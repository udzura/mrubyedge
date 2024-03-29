extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./fib.mrb");
    //let bin = include_bytes!("./if.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
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
            eprintln!("{:?}", TryInto::<i32>::try_into(v.as_ref()).unwrap());
        }
        None => eprintln!("None"),
    }

    // dbg!(&vm);
    ()
}
