use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use quick_select::{gen_vec, quick_select, Doc};

const VEC_SIZE: usize = 10_000;
const MIN: usize = 1000;
const MAX: usize = 2001;
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut v = gen_vec(VEC_SIZE);
    let mut rng = StdRng::seed_from_u64(0);
    let last = rng.random_range(MIN..=MAX);
    let r = v.len() - 1;
    let k = v.len() - last;
    c.bench_function("quick_select", |b| b.iter(|| quick_select(black_box(&mut v), black_box(0), black_box(r), black_box(k))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
