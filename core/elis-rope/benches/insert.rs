extern crate rand;
extern crate ropey;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::random;
use ropey::Rope;

const TEXT: &str = include_str!("large.txt");

//----

fn inserts_random_char(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_random_char", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert_char(random::<usize>() % len, 'a');
        })
    });
}

fn inserts_start_char(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_start_char", |b| {
        b.iter(|| {
            rope.insert_char(0, 'a');
        })
    });
}

fn inserts_middle_char(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_middle_char", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert_char(len / 2, 'a');
        })
    });
}

fn inserts_end_char(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_end_char", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert_char(len, 'a');
        })
    });
}

//----

fn inserts_random_small(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_random_small", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert(random::<usize>() % len, "a");
        })
    });
}

fn inserts_start_small(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_start_small", |b| {
        b.iter(|| {
            rope.insert(0, "a");
        })
    });
}

fn inserts_middle_small(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_middle_small", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert(len / 2, "a");
        })
    });
}

fn inserts_end_small(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_end_small", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert(len, "a");
        })
    });
}

//----

fn inserts_random_medium(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_random_medium", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert(random::<usize>() % len, "This is some text.");
        })
    });
}

fn inserts_start_medium(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_start_medium", |b| {
        b.iter(|| {
            rope.insert(0, "This is some text.");
        })
    });
}

fn inserts_middle_medium(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_middle_medium", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert(len / 2, "This is some text.");
        })
    });
}

fn inserts_end_medium(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_end_medium", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert(len, "This is some text.");
        })
    });
}

//----

const INSERT_TEXT: &str = include_str!("small.txt");

fn inserts_random_large(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_random_large", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert(random::<usize>() % len, INSERT_TEXT);
        })
    });
}

fn inserts_start_large(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_start_large", |b| {
        b.iter(|| {
            rope.insert(0, INSERT_TEXT);
        })
    });
}

fn inserts_middle_large(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_middle_large", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert(len / 2, INSERT_TEXT);
        })
    });
}

fn inserts_end_large(c: &mut Criterion) {
    let mut rope = Rope::from_str(TEXT);
    c.bench_function("inserts_end_large", |b| {
        b.iter(|| {
            let len = rope.len_chars();
            rope.insert(len, INSERT_TEXT);
        })
    });
}

//----

fn initial_insert_after_clone(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let mut rope_clone = rope.clone();
    let mut i = 0;
    c.bench_function("initial_insert_after_clone", |b| {
        b.iter(|| {
            if i > 32 {
                i = 0;
                rope_clone = rope.clone();
            }
            let len = rope_clone.len_chars();
            rope_clone.insert(random::<usize>() % len, "a");
            i += 1;
        })
    });
}

//----

criterion_group!(
    benches,
    inserts_random_char,
    inserts_start_char,
    inserts_middle_char,
    inserts_end_char,
    inserts_random_small,
    inserts_start_small,
    inserts_middle_small,
    inserts_end_small,
    inserts_random_medium,
    inserts_start_medium,
    inserts_middle_medium,
    inserts_end_medium,
    inserts_random_large,
    inserts_start_large,
    inserts_middle_large,
    inserts_end_large,
    initial_insert_after_clone
);
criterion_main!(benches);
