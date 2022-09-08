use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gol::{life::Life, pattern::Pattern};

fn r_pentomino() {
    // Test a R-pentomino methuselah: https://conwaylife.com/wiki/R-pentomino
    let mut game = Life::new(
        (64, 64),
        &Pattern {
            pattern: vec![false, true, true, true, true, false, false, true, false],
            size: (3, 3),
        },
    );
    for _ in 0..500 {
        game.update();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("R-pentomino", |b| b.iter(|| black_box(r_pentomino())));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = criterion_benchmark
}
criterion_main!(benches);
