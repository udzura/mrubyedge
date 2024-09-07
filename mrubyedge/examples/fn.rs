use std::{cell::RefCell, rc::Rc};

use mrubyedge::{mrb_helper, vm::RObject};

extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./def3.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
    // dbg!(&rite);
    let mut vm = mrubyedge::vm::VM::open(rite);
    vm.prelude().unwrap();
    // dbg!(&vm);
    vm.eval_insn().unwrap();

    eprintln!("return value:");
    let top = 0 as usize;
    match vm.regs.get(&top) {
        Some(v) => eprintln!("{:?}", v),
        None => eprintln!("None"),
    }

    let objclass_sym = vm.target_class.unwrap() as usize;
    let top_self = RObject::RInstance {
        data: Rc::new(RefCell::new(Box::new(()))),
        class_index: objclass_sym,
    };
    let args = vec![Rc::new(RObject::RString("WASM! 4".to_string()))];
    // let args = vec![];
    match mrb_helper::mrb_funcall(&mut vm, &top_self, "hello".to_string(), &args) {
        Ok(retval) => {
            dbg!(retval);
        }
        Err(ex) => {
            dbg!(ex);
        }
    };

    ()
}
