extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn shared_memory_test() {
    let code = "$memory = SharedMemory.new(64)

def get_memory
  $memory
end

def read_array_from_memory
  result = $memory[0..4].unpack('c c c c')
  result[0] + result[1] + result[2] + result[3]
end";
    let binary = mrbc_compile("shared_memory", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![
    ];
    let result1  = mrb_funcall(&mut vm, None, "get_memory", &args).unwrap();
    assert!(result1.as_ref().get_class(&mut vm).as_ref().sym_id.name == "SharedMemory");

    let result2  = mrb_funcall(&mut vm, None, "read_array_from_memory", &args).unwrap();
    let result2: i64 = result2.as_ref().try_into().unwrap();
    assert_eq!(result2, 0);
}
