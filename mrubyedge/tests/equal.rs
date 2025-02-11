extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;
use mrubyedge::yamrb::value::RObject;

#[test]
fn equal_test() {
    let code = "
    def check_eq_1(a, b)
      3 == a + b
    end

    def check_eq_2(a, b)
      \"foobar\" == a + b
    end

    def check_eq_3(a, b)
      [:foo, :bar] == [a, b]
    end

    def check_eq_4(a, b, c, d)
      ha = {}
      ha[a] = b
      ha[c] = d

      {foo: 1, bar: \"str\"} == ha
    end
    ";
    let binary = mrbc_compile("eq", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![RObject::integer(1).to_refcount_assigned(), RObject::integer(2).to_refcount_assigned()];
    let result: bool = mrb_funcall(&mut vm, None, "check_eq_1", &args).unwrap().as_ref().try_into().unwrap();
    assert!(result);

    let args = vec![RObject::string("foo".into()).to_refcount_assigned(), RObject::string("bar".into()).to_refcount_assigned()];
    let result: bool = mrb_funcall(&mut vm, None, "check_eq_2", &args).unwrap().as_ref().try_into().unwrap();
    assert!(result);

    let args = vec![RObject::symbol("foo".into()).to_refcount_assigned(), RObject::symbol("bar".into()).to_refcount_assigned()];
    let result: bool = mrb_funcall(&mut vm, None, "check_eq_3", &args).unwrap().as_ref().try_into().unwrap();
    assert!(result);

    let args = vec![
      RObject::symbol("foo".into()).to_refcount_assigned(),
      RObject::integer(1).to_refcount_assigned(),
      RObject::symbol("bar".into()).to_refcount_assigned(),
      RObject::string("str".into()).to_refcount_assigned(),
    ];
    let result: bool = mrb_funcall(&mut vm, None, "check_eq_4", &args).unwrap().as_ref().try_into().unwrap();
    assert!(result);
}
