use criterion::{black_box, criterion_group, criterion_main, Criterion};
use libstampcalc::Solutions;
use libstampcalc::new_algo::solutions;
use std::iter::zip;

fn small_old() -> bool {
    let mut sols = Solutions::new(100, &vec![5, 10, 15]);
    let answers = sols.make_into_iterator();
    let mut nres: usize = 0;
    for _ in answers {
        nres += 1;
    }

    return nres == 44;
}

fn small_new() -> bool {
    let _ = solutions(100, &vec![5, 10, 15]);
    true
}

fn medium_old() -> bool {
    let mut sols = Solutions::new(175, &vec![5, 6, 7, 10, 14]);
    let answers = sols.make_into_iterator();
    let mut nres: usize = 0;
    for _ in answers {
        nres += 1;
    }
    return nres == 500;
}

fn medium_new() -> bool {
    let _ = solutions(175, &vec![5, 6, 7, 10, 14]);
    true
}

fn bench_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("small solutions");
    group.bench_function("old", |b| b.iter(|| small_old()));
    group.bench_function("new", |b| b.iter(|| small_new()));
    group.finish();
}

fn bench_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("medium solutions");
    group.bench_function("old", |b| b.iter(|| medium_old()));
    group.bench_function("new", |b| b.iter(|| medium_new()));
    group.finish();
}

criterion_group!(benches, bench_small, bench_medium);
criterion_main!(benches);
