use criterion::{criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use gol::{life::Life, pattern::Pattern};

fn random(size: usize) {
    let mut game = Life::new(
        (size, size),
        &Pattern::from_random((size, size)),
    );
    game.update();
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("random");
    group.sample_size(20);
    for size in [64usize, 128, 256, 512, 1024, 2048, 3072, 4096, 6144, 8192].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| random(size))
        });
    }
    group.finish();
}

criterion_group!{benches, bench}
criterion_main!(benches);