extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use std::rc::Rc;

use helpers::*;
use mrubyedge::yamrb::helpers::mrb_define_cmethod;
use mrubyedge::yamrb::value::RObject;
use mrubyedge::yamrb::vm::VM;
use mrubyedge::Error;

#[test]
fn fncall_test() {
    let code = "
def double(n)
  n * 2
end
    ";
    let binary = mrbc_compile("fncall", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Rust method that calls mrb_funcall internally
    fn rust_method_calling_mrb_funcall(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
        // Get the first argument (should be an integer)
        let n = if args.len() > 0 {
            let arg: i64 = args[0].as_ref().try_into()?;
            arg
        } else {
            0
        };
        
        // Call Ruby's double method via mrb_funcall
        let args_for_call = vec![Rc::new(RObject::integer(n))];
        let result = mrb_funcall(vm, None, "double", &args_for_call)?;
        
        Ok(result)
    }

    let kernel = vm.object_class.clone();
    mrb_define_cmethod(&mut vm, kernel, "call_double", Box::new(rust_method_calling_mrb_funcall));

    // Call the Rust method which internally calls mrb_funcall
    let args = vec![Rc::new(RObject::integer(5))];
    let result = mrb_funcall(&mut vm, None, "call_double", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 10);
}

#[test]
fn nested_fncall_test() {
    let code = "
def add(a, b)
  a + b
end

def multiply(a, b)
  do_multiply(a, b)
end

complex_calc(2, 3)
    ";
    let binary = mrbc_compile("nested_fncall", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);

    fn rust_method_do_multiply(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
        let a: i64 = args[0].as_ref().try_into()?;
        let b: i64 = args[1].as_ref().try_into()?;
        
        let result = a * b;
        Ok(Rc::new(RObject::integer(result)))
    } 

    // Rust method that calls multiple Ruby methods
    fn complex_calculation(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
        let a: i64 = args[0].as_ref().try_into()?;
        let b: i64 = args[1].as_ref().try_into()?;
        
        // Call add(a, b)
        let add_args = vec![Rc::new(RObject::integer(a)), Rc::new(RObject::integer(b))];
        let sum = mrb_funcall(vm, None, "add", &add_args)?;
        
        // Call multiply(sum, 3)
        let sum_val: i64 = sum.as_ref().try_into()?;
        let mul_args = vec![Rc::new(RObject::integer(sum_val)), Rc::new(RObject::integer(3))];
        let result = mrb_funcall(vm, None, "multiply", &mul_args)?;
        
        Ok(result)
    }

    let kernel = vm.object_class.clone();
    mrb_define_cmethod(&mut vm, kernel.clone(), "do_multiply", Box::new(rust_method_do_multiply));
    mrb_define_cmethod(&mut vm, kernel.clone(), "complex_calc", Box::new(complex_calculation));

    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 15);

    // Test: (2 + 3) * 3 = 15
    let args = vec![Rc::new(RObject::integer(2)), Rc::new(RObject::integer(3))];
    let result = mrb_funcall(&mut vm, None, "complex_calc", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 15);
}
