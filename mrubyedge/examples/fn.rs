extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./def3.mrb");
    let mut rite = mrubyedge::rite::load(bin).unwrap();
    // dbg!(&rite);
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);

    eprintln!("return value:");
    eprintln!("{:?}", vm.run().unwrap());

    // match mrb_helper::mrb_funcall(&mut vm, &top_self, "hello".to_string(), &args) {
    //     Ok(retval) => {
    //         dbg!(retval);
    //     }
    //     Err(ex) => {
    //         dbg!(ex);
    //     }
    // };

    ()
}
