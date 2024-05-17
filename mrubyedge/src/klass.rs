use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::vm::*;

pub const KLASS_SYM_ID_OBJECT: u32 = 1;
pub const KLASS_SYM_ID_TIME: u32 = 1 << 6;
pub const KLASS_SYM_ID_RANDOM: u32 = 1 << 7;

pub fn new_builtin_object_class() -> RClass<'static> {
    let mut methods = HashMap::new();
    if cfg!(feature = "wasi") {
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
    }

    RClass {
        sym_id: KLASS_SYM_ID_OBJECT,
        super_klass: Rc::new(None),
        static_methods: HashMap::new(),
        methods,
    }
}

#[cfg(feature = "wasi")]
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

#[cfg(feature = "wasi")]
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

#[cfg(feature = "wasi")]
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

#[cfg(feature = "wasi")]
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

#[cfg(feature = "wasi")]
pub fn new_builtin_time_class() -> RClass<'static> {
    let mut static_methods = HashMap::new();
    static_methods.insert(
        "now".to_string(),
        RMethod {
            sym_id: 640001,
            body: Method::CMethod(builtin_time_cmethod_now),
        },
    );

    let mut methods = HashMap::new();
    methods.insert(
        "to_i".to_string(),
        RMethod {
            sym_id: 641001,
            body: Method::CMethod(builtin_time_imethod_to_i),
        },
    );

    RClass {
        sym_id: KLASS_SYM_ID_TIME,
        super_klass: Rc::new(None),
        static_methods,
        methods,
    }
}

#[cfg(feature = "wasi")]
fn builtin_time_cmethod_now<'insn>(
    _vm: &mut VM<'insn>,
    _self: &RObject,
    _args: &[Rc<RObject>],
) -> Rc<RObject> {
    let now = SystemTime::now();
    let data = now.duration_since(UNIX_EPOCH).unwrap();
    let data = data.as_nanos() as u64;
    let data = Rc::new(RefCell::new(Box::new(data) as Box<dyn Any>));

    let obj = RObject::RInstance {
        class_index: (KLASS_SYM_ID_TIME as usize),
        data,
    };
    Rc::new(obj)
}

#[cfg(feature = "wasi")]
fn builtin_time_imethod_to_i<'insn>(
    _vm: &mut VM<'insn>,
    selfobj: &RObject,
    _args: &[Rc<RObject>],
) -> Rc<RObject> {
    if let RObject::RInstance {
        class_index: _,
        data,
    } = selfobj
    {
        let unixnano = *data
            .borrow()
            .downcast_ref::<u64>()
            .expect("Time should contains u64");
        let ret = RObject::RInteger(unixnano as i64);
        Rc::new(ret)
    } else {
        unreachable!("Time but not rinstance");
    }
}
