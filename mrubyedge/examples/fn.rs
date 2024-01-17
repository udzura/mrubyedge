use mrubyedge::{mrb_helper, vm::RObject};

extern crate mrubyedge;

fn main() {
    let bin = include_bytes!("./def.mrb");
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

    let top = 0;
    let top_self = vm.regs.get(&top).unwrap().clone();
    let args = vec![];
    match mrb_helper::mrb_funcall(&mut vm, top_self.as_ref(), "hello".to_string(), &args) {
        Ok(retval) => {
            dbg!(retval);
        }
        Err(ex) => {
            dbg!(ex);
        }
    };

    ()
}
