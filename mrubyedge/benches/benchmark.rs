use criterion::{criterion_group, criterion_main, Criterion};
use mrubyedge::*;

use std::{any::Any, cell::RefCell, rc::Rc};

fn bm1(c: &mut Criterion) {
    let bin = include_bytes!("./hello.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
    let mut vm = mrubyedge::vm::VM::open(rite);
    vm.prelude().unwrap();
    vm.eval_insn().unwrap();

    let objclass_sym = vm.target_class.unwrap() as usize;
    let top_self = vm::RObject::RInstance {
        class_index: objclass_sym,
        data: Rc::new(RefCell::new(Box::new(()) as Box<dyn Any>)),
    };
    let args = vec![];

    c.bench_function("Hello world", |b| {
        b.iter(|| {
            match mrb_helper::mrb_funcall(&mut vm, &top_self, "hello".to_string(), &args) {
                Ok(_) => {
                    // OK
                }
                Err(ex) => {
                    dbg!(ex);
                }
            };
        })
    });
}

fn bm2(c: &mut Criterion) {
    let bin = include_bytes!("./fib.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
    let mut vm = mrubyedge::vm::VM::open(rite);
    vm.prelude().unwrap();
    vm.eval_insn().unwrap();

    let objclass_sym = vm.target_class.unwrap() as usize;
    let top_self = vm::RObject::RInstance {
        class_index: objclass_sym,
        data: Rc::new(RefCell::new(Box::new(()) as Box<dyn Any>)),
    };
    let args15 = vec![Rc::new(vm::RObject::RInteger(15))];
    let args20 = vec![Rc::new(vm::RObject::RInteger(20))];

    c.bench_function("Fib 15", |b| {
        b.iter(|| {
            match mrb_helper::mrb_funcall(&mut vm, &top_self, "fib".to_string(), &args15) {
                Ok(_) => {
                    // OK
                }
                Err(ex) => {
                    dbg!(ex);
                }
            };
        })
    });

    c.bench_function("Fib 20", |b| {
        b.iter(|| {
            match mrb_helper::mrb_funcall(&mut vm, &top_self, "fib".to_string(), &args20) {
                Ok(_) => {
                    // OK
                }
                Err(ex) => {
                    dbg!(ex);
                }
            };
        })
    });
}

fn bm3(c: &mut Criterion) {
    let bin = include_bytes!("./long.mrb");
    let rite = mrubyedge::rite::load(bin).unwrap();
    let mut vm = mrubyedge::vm::VM::open(rite);
    vm.prelude().unwrap();
    vm.eval_insn().unwrap();

    let objclass_sym = vm.target_class.unwrap() as usize;
    let top_self = vm::RObject::RInstance {
        class_index: objclass_sym,
        data: Rc::new(RefCell::new(Box::new(()) as Box<dyn Any>)),
    };
    let args = vec![];

    c.bench_function("Long inst", |b| {
        b.iter(|| {
            match mrb_helper::mrb_funcall(&mut vm, &top_self, "long".to_string(), &args) {
                Ok(_) => {
                    // OK
                }
                Err(ex) => {
                    dbg!(ex);
                }
            };
        })
    });
}

criterion_group!(benches, bm1, bm2, bm3);
criterion_main!(benches);
