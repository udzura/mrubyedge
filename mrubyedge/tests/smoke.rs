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

#[test]
fn fib_test() {
    let code = "
        def fib(n)
          if n < 1
            return 0
          elsif n < 3
            return 1
          else
            return fib(n-1)+fib(n-2)
          end
        end
    ";
    let binary = mrbc_compile("fib", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![
        int(1),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 1);

    let args = vec![
        int(2),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 1);

    let args = vec![
        int(3),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 2);

    let args = vec![
        int(10),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 55);

    let args = vec![
        int(15),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 610);
}

#[test]
fn fib2_test() {
    let code = "
        def fib(n)
          case n
          when 0
            0
          when 1..2
            1
          else
            fib(n - 1) + fib(n - 2)
          end
        end
    ";
    let binary = mrbc_compile("fib", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![
        int(1),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 1);

    let args = vec![
        int(2),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 1);

    let args = vec![
        int(3),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 2);

    let args = vec![
        int(10),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 55);

    let args = vec![
        int(15),
    ];
    let result: i32 = mrb_funcall(&mut vm, None, "fib", &args).unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 610);
}