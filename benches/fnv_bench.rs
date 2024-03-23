use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fake::faker::lorem::en::Sentence;
use fake::Fake;
use whack_a_link::fnv::{Fnv, FnvHash, OFFSET_32, PRIME_32};

fn bench_fnv(c: &mut Criterion) {
    let fnv = Fnv {
        prime: PRIME_32,
        offset: OFFSET_32,
    };
    // bench
    let mut group = c.benchmark_group("FNV");

    for i in 1..=5 {
        let sentence: String = Sentence(1..i + 1).fake();

        group.bench_function(BenchmarkId::new("fnv.hash1", i), |b| {
            b.iter(|| fnv.hash1(&sentence))
        });

        group.bench_function(BenchmarkId::new("fnv.hash1a", i), |b| {
            b.iter(|| fnv.hash1a(&sentence))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_fnv);
criterion_main!(benches);
