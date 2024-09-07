use std::{any::Any, cell::RefCell, rc::Rc};

extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./def3.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
    // dbg!(&rite);
    let mut vm = mrubyedge::vm::VM::open(rite);
    vm.prelude().unwrap();
    // dbg!(&vm);
    vm.eval_insn().unwrap();

    let data = String::from("Hello World");
    let data = Box::new(data) as Box<dyn Any>;
    let data = Rc::new(RefCell::new(data));
    vm.register_ud(1, data);

    let ud = vm.fetch_ud(1).unwrap();
    dbg!(ud.borrow().as_ref().downcast_ref::<String>().unwrap());
}
