extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn smoke_test() {
    let code = "def add(a, b)
        a + b
    end";
    let binary = mrbc_compile("add", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![
        int(1),
        int(2),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "add", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 3);
}