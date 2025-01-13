extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn object_test() {
    let code = "
    class Hello
      def world
        puts \"hello world\"
        1
      end
    end

    def test_main
      Hello.new.world
    end
    ";
    let binary = mrbc_compile("add", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 1);
}