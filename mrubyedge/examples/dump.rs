extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./hi.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
    dbg!(&rite);
    for (i, irep) in rite.irep.iter().enumerate() {
        println!("irep #{}", i);
        // mrubyedge::eval::eval_insn(irep.insn).unwrap();
    }
    ()
}
