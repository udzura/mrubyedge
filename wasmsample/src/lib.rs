extern crate mrubyedge;

use mrubyedge::{mrb_helper, vm::RObject};

const DATA: &'static [u8] = include_bytes!("./hello.mrb");
const FNNAME: &'static str = "hello_mruby_from_wasm";

#[no_mangle]
pub fn hello_mruby_from_wasm() {
    let rite = mrubyedge::rite::load(DATA).unwrap();
    let mut vm = mrubyedge::vm::VM::open(rite);
    vm.prelude().unwrap();
    vm.eval_insn().unwrap();

    let objclass_sym = vm.target_class.unwrap() as usize;
    let top_self = RObject::RInstance {
        class_index: objclass_sym,
    };
    let args = vec![];

    match mrb_helper::mrb_funcall(&mut vm, &top_self, FNNAME.to_string(), &args) {
        Ok(retval) => {
            eprintln!("{:?}", retval);
        }
        Err(ex) => {
            dbg!(ex);
        }
    };

    ()
}
