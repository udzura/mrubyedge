extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn raise_test() {
    let code = "
    def test_raise
      raise \"Intentional Error\"
    end
    ";
    let binary = mrbc_compile("raise", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_raise", &args)
        .err();
    assert_eq!(&result.unwrap().message(), "Intentional Error");
}

#[test]
fn raise_nest_test() {
    let code = "
    def do_raise
      raise \"Intentional Error 2\"
      p :HOGE
    end

    def test_raise
      do_raise
      p :NG
    end
    ";
    let binary = mrbc_compile("raise_nest", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_raise", &args)
        .err();
    assert_eq!(&result.unwrap().message(), "Intentional Error 2");
}

#[test]
fn rescue_test() {
    let code = "
    def test_raise
      begin
        raise \"Intentional Error 3\"
      rescue => e
        puts \"rescue: #{e.message}\"
        \"rescue: #{e.message}\"
      end
    end
    ";
    let binary = mrbc_compile("rescue", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_raise", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(&result, "rescue: Intentional Error 3");
}

#[test]
fn rescue_nest_test() {
    let code = "
    def test_raise
      raise \"Intentional Error 4\"
    end

    def test_raise_parent
      begin
        test_raise
        \"NG\"
      rescue => e
        puts \"rescue: #{e.message}\"
        \"rescue: #{e.message}\"
      end
    end
    ";
    let binary = mrbc_compile("rescue_nest", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_raise_parent", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(&result, "rescue: Intentional Error 4");
}