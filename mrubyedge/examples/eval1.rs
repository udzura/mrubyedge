extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./simple.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
    // dbg!(&rite);
    let mut vm = mrubyedge::vm::VM::open(rite);
    dbg!(&vm);
    vm.eval_insn().unwrap();

    eprintln!("return value:");
    let top = 0 as usize;
    dbg!(vm.regs.get(&top).unwrap());

    // mrubyedge::eval::debug_eval_insn(irep.inst_head).unwrap();
    ()
}
