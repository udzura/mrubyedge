extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use std::rc::Rc;

use helpers::*;
use mrubyedge::yamrb::value::RObject;

#[test]
fn return_test() {
    let code = "
def fib(n)
  return 1 if n <= 1
  fib(n - 1) + fib(n - 2)
end
    ";
    let binary = mrbc_compile("return_fib", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![Rc::new(RObject::integer(10))];
    let result  = mrb_funcall(&mut vm, None, "fib", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 89);

    // Assert 2
    let args = vec![Rc::new(RObject::integer(1))];
    let result  = mrb_funcall(&mut vm, None, "fib", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 1);
  }
