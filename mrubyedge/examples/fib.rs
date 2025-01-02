extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./fib.mrb");
    //let bin = include_bytes!("./if.mrb");
    let mut rite = mrubyedge::rite::load(bin).unwrap();
    // dbg!(&rite);
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
 
    eprintln!("return value:");
    eprintln!("{:?}", vm.run().unwrap());
    // dbg!(&vm);
    ()
}
