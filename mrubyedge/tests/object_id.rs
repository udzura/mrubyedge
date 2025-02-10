extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn object_id_test() {
    let code = "def myid
      o = Object.new
      puts \"#{o.object_id}\"
      o2 = o
      puts \"#{o2.object_id}\"
      o.object_id == o2.object_id
    end";
    let binary = mrbc_compile("object_id", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: bool = mrb_funcall(&mut vm, None, "myid", &args).unwrap().as_ref().try_into().unwrap();
    assert!(result);
}

#[test]
fn object_id_2_test() {
    let code = "def myid
      o = Object.new
      puts \"#{o.object_id}\"
      o2 = Object.new
      puts \"#{o2.object_id}\"
      o3 = Object.new
      puts \"#{o3.object_id}\"
      o2.object_id == o3.object_id
    end";
    let binary = mrbc_compile("object_id_2", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: bool = mrb_funcall(&mut vm, None, "myid", &args).unwrap().as_ref().try_into().unwrap();
    assert!(!result);
}
