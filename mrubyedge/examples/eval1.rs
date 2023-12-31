extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./simple.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
    // dbg!(&rite);
    for (i, irep) in rite.irep.into_iter().enumerate() {
        println!("irep #{}", i);
        let mut irep = mrubyedge::vm::VMIrep::from_raw_record(irep);
        dbg!(&irep);
        irep.eval_insn().unwrap();

        // mrubyedge::eval::debug_eval_insn(irep.inst_head).unwrap();
    }
    ()
}
