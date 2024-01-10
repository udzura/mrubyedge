extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./simple.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
    // dbg!(&rite);
    for (i, irep) in rite.irep.into_iter().enumerate() {
        println!("irep #{}", i);
        let mut vm = mrubyedge::vm::VM::open(irep);
        dbg!(&vm);
        vm.eval_insn().unwrap();

        eprintln!("return value:");
        let top = 0 as usize;
        dbg!(vm.regs.get(&top).unwrap());

        // mrubyedge::eval::debug_eval_insn(irep.inst_head).unwrap();
    }
    ()
}
