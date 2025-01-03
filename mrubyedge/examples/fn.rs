use mrubyedge::yamrb::helpers::mrb_funcall;

extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./def3.mrb");
    let mut rite = mrubyedge::rite::load(bin).unwrap();
    // dbg!(&rite);
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);

    eprintln!("evaluate value:");
    eprintln!("{:?}", vm.run().unwrap());

    let args = vec![
    ];

    match mrb_funcall(&mut vm, None, "hello", &args) {
        Ok(retval) => {
            dbg!(retval);
        }
        Err(ex) => {
            eprintln!("Error");
            dbg!(ex);
        }
    };

    ()
}
