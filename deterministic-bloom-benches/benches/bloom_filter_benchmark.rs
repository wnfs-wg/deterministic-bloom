use criterion::{criterion_group, criterion_main, Criterion};
use deterministic_bloom::BloomFilter;
use rand::Rng;

pub fn add_benchmark(crit: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut bloom = BloomFilter::<256, 30>::new();

    crit.bench_function("add", |bench| {
        bench.iter(|| {
            let new_val: u8 = rng.gen_range(0..10);
            bloom.insert(&[new_val; 32]);
        })
    });
}

pub fn count_ones_benchmark(crit: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut bloom = BloomFilter::<256, 30>::new();

    for _ in 1..50 {
        let new_val: u8 = rng.gen_range(0..10);
        bloom.insert(&[new_val; 32]);
    }

    crit.bench_function("count_ones", |bench| bench.iter(|| bloom.count_ones()));
}

criterion_group!(benches, add_benchmark, count_ones_benchmark);
criterion_main!(benches);
