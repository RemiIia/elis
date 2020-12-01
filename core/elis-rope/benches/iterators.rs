extern crate ropey;

use criterion::{criterion_group, criterion_main, Criterion};
use ropey::Rope;

const TEXT: &str = include_str!("large.txt");
const TEXT_TINY: &str = include_str!("tiny.txt");

//----

fn create_bytes_iter(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    c.bench_function("create_bytes_iter", |b| {
        b.iter(|| {
            r.bytes();
        })
    });
}

fn create_bytes_iter_at(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let len = r.len_bytes();
    let mut i = 0;
    c.bench_function("create_bytes_iter_at", |b| {
        b.iter(|| {
            r.bytes_at(i % (len + 1));
            i += 1;
        })
    });
}

fn create_bytes_iter_at_end(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let len = r.len_bytes();
    c.bench_function("create_bytes_iter_at_end", |b| {
        b.iter(|| {
            r.bytes_at(len);
        })
    });
}

fn create_chars_iter(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    c.bench_function("create_chars_iter", |b| {
        b.iter(|| {
            r.chars();
        })
    });
}

fn create_chars_iter_at(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let len = r.len_chars();
    let mut i = 0;
    c.bench_function("create_chars_iter_at", |b| {
        b.iter(|| {
            r.chars_at(i % (len + 1));
            i += 1;
        })
    });
}

fn create_chars_iter_at_end(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let len = r.len_chars();
    c.bench_function("create_chars_iter_at_end", |b| {
        b.iter(|| {
            r.chars_at(len);
        })
    });
}

fn create_lines_iter(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    c.bench_function("create_lines_iter", |b| {
        b.iter(|| {
            r.lines();
        })
    });
}

fn create_lines_iter_at(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let len = r.len_lines();
    let mut i = 0;
    c.bench_function("create_lines_iter_at", |b| {
        b.iter(|| {
            r.lines_at(i % (len + 1));
            i += 1;
        })
    });
}

fn create_chunks_iter(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    c.bench_function("create_chunks_iter", |b| {
        b.iter(|| {
            r.chunks();
        })
    });
}

fn create_chunks_iter_at_byte(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let len = r.len_bytes();
    let mut i = 0;
    c.bench_function("create_chunks_iter_at_byte", |b| {
        b.iter(|| {
            r.chunks_at_byte(i % (len + 1));
            i += 1;
        })
    });
}

fn create_chunks_iter_at_char(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let len = r.len_chars();
    let mut i = 0;
    c.bench_function("create_chunks_iter_at_char", |b| {
        b.iter(|| {
            r.chunks_at_char(i % (len + 1));
            i += 1;
        })
    });
}

fn create_chunks_iter_at_line_break(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let len = r.len_lines();
    let mut i = 0;
    c.bench_function("create_chunks_iter_at_line_break", |b| {
        b.iter(|| {
            r.chunks_at_line_break(i % (len + 1));
            i += 1;
        })
    });
}

//----

fn bytes_iter_next(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let mut itr = r.bytes().cycle();
    c.bench_function("bytes_iter_next", |b| {
        b.iter(|| {
            itr.next();
        })
    });
}

fn bytes_iter_prev(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let itr_src = r.bytes_at(r.len_bytes());
    let mut itr = itr_src.clone();
    c.bench_function("bytes_iter_prev", |b| {
        b.iter(|| {
            if itr.prev().is_none() {
                itr = itr_src.clone();
            }
        });
    });
}

fn chars_iter_next(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let mut itr = r.chars().cycle();
    c.bench_function("chars_iter_next", |b| {
        b.iter(|| {
            itr.next();
        })
    });
}

fn chars_iter_prev(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let itr_src = r.chars_at(r.len_chars());
    let mut itr = itr_src.clone();
    c.bench_function("chars_iter_prev", |b| {
        b.iter(|| {
            if itr.prev().is_none() {
                itr = itr_src.clone();
            }
        });
    });
}

fn lines_iter_next(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let mut itr = r.lines().cycle();
    c.bench_function("lines_iter_next", |b| {
        b.iter(|| {
            itr.next();
        })
    });
}

fn lines_iter_next_tiny(c: &mut Criterion) {
    let r = Rope::from_str(TEXT_TINY);
    let mut itr = r.lines().cycle();
    c.bench_function("lines_iter_next_tiny", |b| {
        b.iter(|| {
            itr.next();
        })
    });
}

fn lines_iter_prev(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let itr_src = r.lines_at(r.len_lines());
    let mut itr = itr_src.clone();
    c.bench_function("lines_iter_prev", |b| {
        b.iter(|| {
            if itr.prev().is_none() {
                itr = itr_src.clone();
            }
        });
    });
}

fn lines_iter_prev_tiny(c: &mut Criterion) {
    let r = Rope::from_str(TEXT_TINY);
    let itr_src = r.lines_at(r.len_lines());
    let mut itr = itr_src.clone();
    c.bench_function("lines_iter_prev_tiny", |b| {
        b.iter(|| {
            if itr.prev().is_none() {
                itr = itr_src.clone();
            }
        });
    });
}

fn chunks_iter_next(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let mut itr = r.chunks().cycle();
    c.bench_function("chunks_iter_next", |b| {
        b.iter(|| {
            itr.next();
        })
    });
}

fn chunks_iter_prev(c: &mut Criterion) {
    let r = Rope::from_str(TEXT);
    let itr_src = r.chunks_at_char(r.len_chars()).0;
    let mut itr = itr_src.clone();
    c.bench_function("chunks_iter_prev", |b| {
        b.iter(|| {
            if itr.prev().is_none() {
                itr = itr_src.clone();
            }
        });
    });
}

//----

criterion_group!(
    benches,
    create_bytes_iter,
    create_bytes_iter_at,
    create_bytes_iter_at_end,
    create_chars_iter,
    create_chars_iter_at,
    create_chars_iter_at_end,
    create_lines_iter,
    create_lines_iter_at,
    create_chunks_iter,
    create_chunks_iter_at_byte,
    create_chunks_iter_at_char,
    create_chunks_iter_at_line_break,
    bytes_iter_next,
    bytes_iter_prev,
    chars_iter_next,
    chars_iter_prev,
    lines_iter_next,
    lines_iter_next_tiny,
    lines_iter_prev,
    lines_iter_prev_tiny,
    chunks_iter_next,
    chunks_iter_prev,
);
criterion_main!(benches);
