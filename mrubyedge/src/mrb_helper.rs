use std::rc::Rc;

use crate::{
    rite::Error,
    vm::{Method, RObject, VM},
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
                        return Ok(Rc::new(ret));
                    }
                    Method::RubyMethod(body) => {
                        vm.cur_irep = body.clone();
                        vm.eval_insn().unwrap();

                        for (i, arg) in args.iter().enumerate() {
                            vm.regs.insert(i + 1, arg.clone());
                        }

                        let zero = 0 as usize;
                        let ret = vm.regs.remove(&zero).unwrap();
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
