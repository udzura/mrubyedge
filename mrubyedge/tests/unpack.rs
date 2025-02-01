extern crate mec_mrbc_sys;
extern crate mrubyedge;

mod helpers;
use helpers::*;

#[test]
fn unpack_test() {
    let code = "
def sum_unpack
  data = \"\\x01\\x02\\x03\\x04\"
  result = data.unpack('c c c c')
  result[0] + result[1] + result[2] + result[3]
end";
    let binary = mrbc_compile("shared_memory", code);
    let mut rite = mrubyedge::rite::load(&binary).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    // Assert
    let args = vec![];
    let result  = mrb_funcall(&mut vm, None, "sum_unpack", &args).unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();
    assert_eq!(result, 10);
}
