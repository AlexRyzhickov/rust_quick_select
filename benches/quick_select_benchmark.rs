use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quick_select::{gen_doc_vec, quick_select, quick_select_no_generic};
use quick_select::{VEC_SIZE, LAST};

pub fn criterion_benchmark(c: &mut Criterion) {
    let v = gen_doc_vec(VEC_SIZE);
    let r = v.len() - 1;
    let k = v.len() - LAST;
    let mut group = c.benchmark_group("quick_select_group");
    group.measurement_time(std::time::Duration::from_secs(50));
    group.bench_function("quick_select", |b| {
        b.iter_batched(
            || v.clone(),
            |mut v| quick_select(black_box(&mut v), black_box(0), black_box(r), black_box(k)),
            criterion::BatchSize::SmallInput,
        );
    });
    group.bench_function("quick_select_no_generic", |b| {
        b.iter_batched(
            || v.clone(),
            |mut v| quick_select_no_generic(black_box(&mut v), black_box(0), black_box(r), black_box(k)),
            criterion::BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// Bench results
// fn quick_select for structures without generics is faster than fn quick_select_no_generic with the generic: 13.339 ms vs 23.555 ms
// fn quick_select for primitive types without generics has the same performance as fn quick_select_no_generic with the generic

// quick_select_group/quick_select
// time:   [23.555 ms 24.520 ms 25.506 ms]
// change: [-5.1185% +0.7602% +6.6955%] (p = 0.80 > 0.05)
// No change in performance detected.
// Benchmarking quick_select_group/quick_select_no_generic: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 50.0s. You may wish to increase target time to 71.8s, enable flat sampling, or reduce sample count to 50.
// quick_select_group/quick_select_no_generic
// time:   [13.339 ms 13.578 ms 13.854 ms]
// change: [+2.3724% +4.3193% +6.5686%] (p = 0.00 < 0.05)
// Performance has regressed.
