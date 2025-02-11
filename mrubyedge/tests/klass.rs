extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn attr_reader_test() {
    let code = "
    class Hello
      attr_reader :world

      def update_world
        @world = 123
      end
    end

    def test_main
      w = Hello.new
      w.update_world
      w.world
    end
    ";
    let binary = mrbc_compile("attr_reader", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: i32 = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(result, 123);
}

#[test]
fn attr_reader_2_test() {
    let code = "
    class Hello
      attr_reader :world
    end

    def test_main
      w = Hello.new
      w.world
    end
    ";
    let binary = mrbc_compile("attr_reader_2", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap();
    assert!(result.as_ref().is_nil());
}

#[test]
fn attr_accessor_test() {
    let code = "
    class Hello
      attr_accessor :world
    end

    def test_main
      w = Hello.new
      w.world = \"Hola, attr\"
      w.world
    end
    ";
    let binary = mrbc_compile("attr_accessor", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result: String = mrb_funcall(&mut vm, None, "test_main", &args)
        .unwrap().as_ref().try_into().unwrap();
    assert_eq!(&result, "Hola, attr");
}