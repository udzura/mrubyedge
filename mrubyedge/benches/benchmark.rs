use criterion::{criterion_group, criterion_main, Criterion};
use mrubyedge::*;

fn bm1(c: &mut Criterion) {
    c.bench_function("Hello world", |b| {
        b.iter(|| {
            println!("Hola");
            println!("Hola");
        })
    });
}

criterion_group!(benches, bm1);
criterion_main!(benches);
