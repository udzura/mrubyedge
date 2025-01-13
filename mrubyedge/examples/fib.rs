use std::rc::Rc;

use mrubyedge::yamrb::{helpers::mrb_funcall, value::RObject};

extern crate mrubyedge;

fn main() {
    // let bin = include_bytes!("./fib.mrb");
    // //let bin = include_bytes!("./if.mrb");
    // let mut rite = mrubyedge::rite::load(bin).unwrap();
    // // dbg!(&rite);
    // let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    // // dbg!(&vm.irep.reps);

    // eprintln!("return value(1):");
    // eprintln!("{:?}", vm.run().unwrap());
    // // dbg!(&vm);
    // let args = vec![
    //     Rc::new(RObject::integer(25))
    // ];
    // match mrb_funcall(&mut vm, None, "fib", &args) {
    //     Ok(retval) => {
    //         eprintln!("return value(2):");
    //         dbg!(retval);
    //     }
    //     Err(ex) => {
    //         eprintln!("Error");
    //         dbg!(ex);
    //     }
    // };

    ()
}
