use std::{collections::HashMap, rc::Rc};

use crate::vm::*;

const KLASS_SYM_ID_OBJECT: u32 = 1;

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

fn builtin_object_imethod_puts() {
    println!("Hello, world");
}
