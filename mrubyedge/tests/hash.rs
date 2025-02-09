extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;

use helpers::*;
use mrubyedge::yamrb::value::RObject;
use std::rc::Rc;

#[test]
fn hash_test() {
    let code = "
    def test_hash
      foo = {}
      foo[\"bar\"] = 42
      foo[\"bar\"]
    end
    ";
    let binary = mrbc_compile("hash", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_hash", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 42);
}

#[test]
fn hash_2_test() {
  let code = "
  $hash = {}

  def test_hash_set(key, value)
    $hash[key] = value
  end

  def test_hash_get(key)
    $hash[key]
  end
  ";
  let binary = mrbc_compile("hash_2", code);
  let mut rite = mrubyedge::rite::load(&binary).unwrap();
  let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
  vm.run().unwrap();

  // Assert
  let args = vec![
    Rc::new(RObject::symbol("bar".into())),
    Rc::new(RObject::integer(54)),
  ];
  let result: i32 = mrb_funcall(&mut vm, None, "test_hash_set", &args)
      .unwrap().as_ref().try_into().unwrap();
  assert_eq!(result, 54);

  let args = vec![
    Rc::new(RObject::symbol("bar".into())),
  ];
  let result: i32 = mrb_funcall(&mut vm, None, "test_hash_get", &args)
      .unwrap().as_ref().try_into().unwrap();
  assert_eq!(result, 54);
}

#[test]
fn hash_each_test() {
    let code = "
    def test_hash_1
      hash = {
        \"foo\" => 1,
        \"bar\" => 2,
        \"baz\" => 3,
      }
      res = 0
      hash.each do |key, value|
        puts \"key: #{key}, value: #{value}\"
        res += value
      end
      res
    end
    ";
    let binary = mrbc_compile("hash_each", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let value = mrb_funcall(&mut vm, None, "test_hash_1", &args)
        .unwrap();
    let value: i64 = value.as_ref().try_into().unwrap();
    assert_eq!(value, 6);
}

#[test]
fn hash_each_test_2() {
    let code = "
    def test_hash_1
      hash = {
        \"foo\" => 1,
        \"bar\" => 2,
        \"baz\" => 3,
      }
      res = \"\"
      hash.each do |key, value|
        res += key
      end
      res
    end
    ";
    let binary = mrbc_compile("hash_each", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let value = mrb_funcall(&mut vm, None, "test_hash_1", &args)
        .unwrap();
    let value: String = value.as_ref().try_into().unwrap();
    assert!(value.contains("foo"));
    assert!(value.contains("bar"));
    assert!(value.contains("baz"));
}