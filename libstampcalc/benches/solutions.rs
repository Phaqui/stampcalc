use criterion::{black_box, criterion_group, criterion_main, Criterion};
use libstampcalc::Solutions;
use std::iter::zip;

fn small() -> bool {
    let mut sols = Solutions::new(100, &vec![5, 10, 15]);
    let answers = sols.make_into_iterator();
    let mut nres: usize = 0;
    for _ in answers {
        nres += 1;
    }

    return nres == 44;
}

fn bench_small(c: &mut Criterion) {
    c.bench_function("small", |b| b.iter(|| small()));
}

criterion_group!(benches, bench_small);
criterion_main!(benches);
