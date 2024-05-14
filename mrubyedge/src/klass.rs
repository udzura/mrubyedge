use std::{collections::HashMap, rc::Rc};

use crate::vm::*;

pub const KLASS_SYM_ID_OBJECT: u32 = 1;
pub const KLASS_SYM_ID_TIME: u32 = 1 << 6;
pub const KLASS_SYM_ID_RANDOM: u32 = 1 << 7;

pub fn new_builtin_object_class() -> RClass<'static> {
    let mut methods = HashMap::new();
    methods.insert(
        "p".to_string(),
        RMethod {
            sym_id: 10000,
            body: Method::CMethod(builtin_object_imethod_p),
        },
    );
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
        static_methods: HashMap::new(),
        methods,
    }
}

fn builtin_object_imethod_puts<'insn>(
    _vm: &mut VM<'insn>,
    _: &RObject,
    args: &[Rc<RObject>],
) -> Rc<RObject> {
    if args.len() < 1 {
        eprintln!("invalid arg size");
        return Rc::new(RObject::Nil);
    }

    match args[0].clone().as_ref() {
        RObject::RString(s) => {
            println!("{}", s);
        }
        _ => {
            eprintln!("type mismatch");
            return Rc::new(RObject::Nil);
        }
    }
    Rc::new(RObject::Nil)
}

fn builtin_object_imethod_p<'insn>(
    _vm: &mut VM<'insn>,
    _: &RObject,
    args: &[Rc<RObject>],
) -> Rc<RObject> {
    if args.len() < 1 {
        eprintln!("invalid arg size");
        return Rc::new(RObject::Nil);
    }

    let ret = args[0].clone();
    dbg!(ret.clone());
    ret
}

pub fn new_builtin_random_class() -> RClass<'static> {
    let mut static_methods = HashMap::new();
    static_methods.insert(
        "rand".to_string(),
        RMethod {
            sym_id: 1280001,
            body: Method::CMethod(builtin_random_cmethod_rand),
        },
    );

    RClass {
        sym_id: KLASS_SYM_ID_RANDOM,
        super_klass: Rc::new(None),
        static_methods,
        methods: HashMap::new(),
    }
}

fn builtin_random_cmethod_rand<'insn>(
    _vm: &mut VM<'insn>,
    _: &RObject,
    args: &[Rc<RObject>],
) -> Rc<RObject> {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf).unwrap();
    let rand = unsafe { std::mem::transmute::<[u8; 4], u32>(buf) };

    if args.len() == 0 {
        let ans = rand as f32 / 0xffffffffu32 as f32;
        return Rc::new(RObject::RFloat(ans as f64));
    }

    if args.len() == 1 {
        match args[0].clone().as_ref() {
            RObject::RInteger(i) => {
                let base = *i as u32;

                let ans = rand % base;
                return Rc::new(RObject::RInteger(ans.into()));
            }
            _ => {
                eprintln!("Argument Error");
                return Rc::new(RObject::Nil);
            }
        }
    }

    eprintln!("Argument Error");
    return Rc::new(RObject::Nil);
}
