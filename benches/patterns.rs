use criterion::{criterion_group, criterion_main, Criterion};
use gol::{life::Life, pattern::Pattern};

const TICKS: usize = 10;

// Test a R-pentomino methuselah: https://conwaylife.com/wiki/R-pentomino
fn r_pentomino() {
    let mut game = Life::new(
        (128, 128),
        &Pattern {
            pattern: vec![false, true, true, true, true, false, false, true, false],
            size: (3, 3),
        },
    );
    for _ in 0..TICKS {
        game.update();
    }
}

// Test a gosper glider gun: https://conwaylife.com/wiki/Gosper_glider_gun
fn gosper_glider_gun() {
    let mut game = Life::new(
        (128, 128),
        &Pattern::from_plaintext(
            "
........................O
......................O.O
............OO......OO............OO
...........O...O....OO............OO
OO........O.....O...OO
OO........O...O.OO....O.O
..........O.....O.......O
...........O...O
............OO"
                .to_string(),
        )
        .unwrap(),
    );
    for _ in 0..TICKS {
        game.update();
    }
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("patterns");
    group.sample_size(50);
    group.bench_function("r_pentomino", |b| b.iter(|| r_pentomino()));
    group.bench_function("gosper_glider_gun", |b| b.iter(|| gosper_glider_gun()));
}

criterion_group! {benches, bench}
criterion_main!(benches);
