use std::{collections::HashMap, rc::Rc};

use crate::vm::*;

pub const KLASS_SYM_ID_OBJECT: u32 = 1;

pub fn new_builtin_object_class() -> RClass<'static> {
    let mut methods = HashMap::new();
    methods.insert(
        "puts".to_string(),
        RMethod {
            sym_id: 10001,
            body: Method::CMethod(builtin_object_imethod_puts),
        },
    );

    RClass {
        sym_id: KLASS_SYM_ID_OBJECT,
        super_klass: Rc::new(None),
        methods,
    }
}

fn builtin_object_imethod_puts<'insn>(_vm: &mut VM<'insn>, args: &[Rc<RObject>]) -> RObject {
    if args.len() < 1 {
        eprintln!("invalid arg size");
        return RObject::Nil;
    }

    match args[0].clone().as_ref() {
        RObject::RString(s) => {
            println!("{}", s);
        }
        _ => {
            eprintln!("type mismatch");
            return RObject::Nil;
        }
    }
    RObject::Nil
}
