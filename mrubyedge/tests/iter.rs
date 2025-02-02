extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn times_test() {
    let code = "
    def test_times
      $i = 0
      3.times do |i|
        $i += i * 2
      end
      $i
    end
    ";
    let binary = mrbc_compile("times", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_times", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 6);
}

#[test]
fn times_self_test() {
    let code = "
    def test_times
      3.times do |i|
        puts \"i: #{i}\"
      end
      nil
    end
    ";
    let binary = mrbc_compile("times_self", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    mrb_funcall(&mut vm, None, "test_times", &args)
        .unwrap();
    assert!(true);
}

#[test]
fn range_each_test() {
    let code = "
    def test_each
      $i = 0
      (1..10).each do |i|
        $i += i * 2
      end
      $i
    end
    ";
    let binary = mrbc_compile("range_each", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_each", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 110);
}

#[test]
fn array_each_test() {
    let code = "
    def test_each
      $i = 0
      [1, 10, 100, 1000, 10000].each do |i|
        $i += i * 2
      end
      $i
    end
    ";
    let binary = mrbc_compile("array_each", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_each", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 22222);
}
