extern crate ropey;

use criterion::{criterion_group, criterion_main, Criterion};
use ropey::Rope;

const TEXT_SMALL: &str = include_str!("small.txt");
const TEXT_MEDIUM: &str = include_str!("medium.txt");
const TEXT_LARGE: &str = include_str!("large.txt");
const TEXT_LF: &str = include_str!("lf.txt");

//----

fn from_str_small(c: &mut Criterion) {
    c.bench_function("from_str_small", |b| {
        b.iter(|| {
            Rope::from_str(TEXT_SMALL);
        })
    });

    // c.bytes = TEXT_SMALL.len() as u64;
}

fn from_str_medium(c: &mut Criterion) {
    c.bench_function("from_str_medium", |b| {
        b.iter(|| {
            Rope::from_str(TEXT_MEDIUM);
        })
    });

    // c.bytes = TEXT_MEDIUM.len() as u64;
}

fn from_str_large(c: &mut Criterion) {
    c.bench_function("from_str_large", |b| {
        b.iter(|| {
            Rope::from_str(TEXT_LARGE);
        })
    });

    // c.bytes = TEXT_LARGE.len() as u64;
}

fn from_str_linefeeds(c: &mut Criterion) {
    c.bench_function("from_str_linefeeds", |b| {
        b.iter(|| {
            Rope::from_str(TEXT_LF);
        })
    });

    // c.bytes = TEXT_LF.len() as u64;
}

//----

fn clone(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT_LARGE);
    c.bench_function("clone", |b| {
        b.iter(|| {
            let _ = rope.clone();
        })
    });
}

//----

criterion_group!(
    benches,
    from_str_small,
    from_str_medium,
    from_str_large,
    from_str_linefeeds,
    clone,
);
criterion_main!(benches);
