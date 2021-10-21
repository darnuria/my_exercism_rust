use criterion::{ criterion_group, criterion_main, Criterion};

use prime_factors;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("factor mine", {
        move |b| b.iter(|| prime_factors::factors(93_819_012_551))
    });
    c.bench_function("factor wheel", {
        move |b| b.iter(|| prime_factors::_factors(93_819_012_551))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);