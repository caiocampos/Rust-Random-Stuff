#[macro_use]
extern crate criterion;

use criterion::Criterion;
use pythagorean_triplet::find_va;
use pythagorean_triplet::find_vb;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("A * A", |b| b.iter(|| (1..90000_u32).map(|a| a * a)));
    c.bench_function("A POW 2", |b| b.iter(|| (1..90000_u32).map(|a| a.pow(2))));
    c.bench_function("triplets VA 100", |b| b.iter(|| find_va(100)));
    c.bench_function("triplets VB 100", |b| b.iter(|| find_vb(100)));
    c.bench_function("triplets VA 3000", |b| b.iter(|| find_va(3000)));
    c.bench_function("triplets VB 3000", |b| b.iter(|| find_vb(3000)));
    c.bench_function("triplets VA 90000", |b| b.iter(|| find_va(90000)));
    c.bench_function("triplets VB 90000", |b| b.iter(|| find_vb(90000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
