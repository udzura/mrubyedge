use std::rc::Rc;

use crate::{
    rite::Error,
    vm::{self, Method, RObject, VM},
};

// pub fn mrb_get_args<'insn>(vm: &mut VM<'insn>, nr: usize) -> Vec<RObject<'insn>> {
//     return Vec::default();
// }

pub fn mrb_funcall<'insn>(
    vm: &mut VM<'insn>,
    recv: &RObject,
    sym: String,
    args: &[Rc<RObject>],
) -> Result<Rc<RObject>, Error> {
    match recv {
        RObject::RInstance { class_index } => {
            let klass = vm.class_arena.get(class_index).unwrap().clone();
            let klass = klass.as_ref().borrow();
            match klass.methods.get(&sym) {
                Some(method) => match &method.body {
                    Method::CMethod(func) => {
                        let ret = func(vm, args);
                        return Ok(ret);
                    }
                    Method::RubyMethod(body) => {
                        vm::push_callinfo(vm);

                        for (i, obj) in args.iter().enumerate() {
                            vm.regs.insert(i + 1, obj.clone());
                        }

                        vm.cur_irep = body.clone();
                        vm.eval_insn().unwrap();

                        let zero = 0 as usize;
                        let ret = vm.regs.remove(&zero).unwrap();

                        vm::pop_callinfo(vm);

                        return Ok(ret);
                    }
                },
                None => {
                    eprint!("todo: method_missing");
                    return Err(Error::General);
                }
            }
        }
        _ => {
            todo!("some day")
        }
    }
    // Ok(RObject::Nil)
}
