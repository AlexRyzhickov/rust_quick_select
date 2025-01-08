use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quick_select::{gen_vec, quick_select};
use quick_select::{VEC_SIZE, LAST};

pub fn criterion_benchmark(c: &mut Criterion) {
    let v = gen_vec(VEC_SIZE);
    let r = v.len() - 1;
    let k = v.len() - LAST;
    c.bench_function("quick_select", |b| {
        b.iter_batched(
            || v.clone(),
            |mut v| quick_select(black_box(&mut v), black_box(0), black_box(r), black_box(k)),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
