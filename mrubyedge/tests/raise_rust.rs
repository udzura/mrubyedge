extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;
use std::rc::Rc;
use mrubyedge::yamrb::helpers::mrb_define_cmethod;
use mrubyedge::yamrb::vm::*;
use mrubyedge::yamrb::value::*;
use mrubyedge::Error;

fn prelude_dummy_error_func(vm: &mut VM) {
  let klass = vm.object_class.clone();
  mrb_define_cmethod(vm, klass, "dummy_raise", Box::new(mrb_test_dummy_raise));
}

fn mrb_test_dummy_raise(_vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
  Err(Error::RuntimeError("Intentional Rust Error".to_string()))
}

#[test]
fn rust_raise_test() {
    let code = "
    def test_raise
      dummy_raise
    end
    ";
    let binary = mrbc_compile("raise_rust", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    prelude_dummy_error_func(&mut vm);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_raise", &args)
        .err();
    assert_eq!(&result.unwrap().message(), "Intentional Rust Error");
}

#[test]
fn rust_raise_nest_test() {
    let code = "
    def test_raise
      dummy_raise
    end

    def shim
      test_raise
    end
    ";
    let binary = mrbc_compile("raise_rust_2", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    prelude_dummy_error_func(&mut vm);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "shim", &args)
        .err();
    assert_eq!(&result.unwrap().message(), "Intentional Rust Error");
}

#[test]
fn rust_raise_rescue_test() {
    let code = "
    def test_raise
      dummy_raise
    rescue => e
      \"rescued: #{e.message}\"
    end
    ";
    let binary = mrbc_compile("raise_rust_3", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    prelude_dummy_error_func(&mut vm);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_raise", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(&result, "rescued: Intentional Rust Error");
}

#[test]
fn rust_nomethod_rescue_test() {
    let code = "
    def test_raise
      dummy_nomethod
    rescue => e
      \"rescued: #{e.message}\"
    end
    ";
    let binary = mrbc_compile("raise_rust_4", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_raise", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(&result, "rescued: Method not found: dummy_nomethod");
}

#[test]
fn rust_noname_rescue_test() {
    let code = "
    def test_raise
      NoName.new
    rescue => e
      \"rescued: #{e.message}\"
    end
    ";
    let binary = mrbc_compile("raise_rust_5", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_raise", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(&result, "rescued: Cannot found name: NoName");
}