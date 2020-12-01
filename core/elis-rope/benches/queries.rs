extern crate rand;
extern crate ropey;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::random;
use ropey::Rope;

const TEXT: &str = include_str!("large.txt");
const SMALL_TEXT: &str = include_str!("small.txt");

//----

fn byte_to_char(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_bytes();
    c.bench_function("byte_to_char", |b| {
        b.iter(|| {
            rope.byte_to_char(random::<usize>() % (len + 1));
        })
    });
}

fn byte_to_line(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_bytes();
    c.bench_function("byte_to_line", |b| {
        b.iter(|| {
            rope.byte_to_line(random::<usize>() % (len + 1));
        })
    });
}

fn char_to_byte(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_chars();
    c.bench_function("char_to_byte", |b| {
        b.iter(|| {
            rope.char_to_byte(random::<usize>() % (len + 1));
        })
    });
}

fn char_to_line(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_chars();
    c.bench_function("char_to_line", |b| {
        b.iter(|| {
            rope.char_to_line(random::<usize>() % (len + 1));
        })
    });
}

fn line_to_byte(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_lines();
    c.bench_function("line_to_byte", |b| {
        b.iter(|| {
            rope.line_to_byte(random::<usize>() % (len + 1));
        })
    });
}

fn line_to_char(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_lines();
    c.bench_function("line_to_char", |b| {
        b.iter(|| {
            rope.line_to_char(random::<usize>() % (len + 1));
        })
    });
}

//----

fn get_byte(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_bytes();
    c.bench_function("get_byte", |b| {
        b.iter(|| {
            rope.byte(random::<usize>() % len);
        })
    });
}

fn get_char(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_chars();
    c.bench_function("get_char", |b| {
        b.iter(|| {
            rope.char(random::<usize>() % len);
        })
    });
}

fn get_line(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_lines();
    c.bench_function("get_line", |b| {
        b.iter(|| {
            rope.line(random::<usize>() % len);
        })
    });
}

fn chunk_at_byte(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_bytes();
    c.bench_function("chunk_at_byte", |b| {
        b.iter(|| {
            rope.chunk_at_byte(random::<usize>() % (len + 1));
        })
    });
}

fn chunk_at_byte_slice(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let slice = rope.slice(324..(rope.len_chars() - 213));
    let len = slice.len_bytes();
    c.bench_function("chunk_at_byte_slice", |b| {
        b.iter(|| {
            slice.chunk_at_byte(random::<usize>() % (len + 1));
        })
    });
}

fn chunk_at_char(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_chars();
    c.bench_function("chunk_at_char", |b| {
        b.iter(|| {
            rope.chunk_at_char(random::<usize>() % (len + 1));
        })
    });
}

fn chunk_at_char_slice(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let slice = rope.slice(324..(rope.len_chars() - 213));
    let len = slice.len_chars();
    c.bench_function("chunk_at_char_slice", |b| {
        b.iter(|| {
            slice.chunk_at_char(random::<usize>() % (len + 1));
        })
    });
}

fn chunk_at_line_break(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_lines();
    c.bench_function("chunk_at_line_break", |b| {
        b.iter(|| {
            rope.chunk_at_line_break(random::<usize>() % (len + 1));
        })
    });
}

fn chunk_at_line_break_slice(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let slice = rope.slice(324..(rope.len_chars() - 213));
    let len = slice.len_lines();
    c.bench_function("chunk_at_line_break_slice", |b| {
        b.iter(|| {
            slice.chunk_at_line_break(random::<usize>() % (len + 1));
        })
    });
}

//----

fn slice(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_chars();
    c.bench_function("slice", |b| {
        b.iter(|| {
            let mut start = random::<usize>() % (len + 1);
            let mut end = random::<usize>() % (len + 1);
            if start > end {
                std::mem::swap(&mut start, &mut end);
            }
            rope.slice(start..end);
        })
    });
}

fn slice_small(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_chars();
    c.bench_function("slice_small", |b| {
        b.iter(|| {
            let mut start = random::<usize>() % (len + 1);
            if start > (len - 65) {
                start = len - 65;
            }
            let end = start + 64;
            rope.slice(start..end);
        })
    });
}

fn slice_from_small_rope(c: &mut Criterion) {
    let rope = Rope::from_str(SMALL_TEXT);
    let len = rope.len_chars();
    c.bench_function("slice_from_small_rope", |b| {
        b.iter(|| {
            let mut start = random::<usize>() % (len + 1);
            let mut end = random::<usize>() % (len + 1);
            if start > end {
                std::mem::swap(&mut start, &mut end);
            }
            rope.slice(start..end);
        })
    });
}

fn slice_whole_rope(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    c.bench_function("slice_whole_rope", |b| {
        b.iter(|| {
            rope.slice(..);
        })
    });
}

fn slice_whole_slice(c: &mut Criterion) {
    let rope = Rope::from_str(TEXT);
    let len = rope.len_chars();
    let slice = rope.slice(1..len - 1);
    c.bench_function("slice_whole_slice", |b| {
        b.iter(|| {
            slice.slice(..);
        })
    });
}

//----

criterion_group!(
    benches,
    byte_to_char,
    byte_to_line,
    char_to_byte,
    char_to_line,
    line_to_byte,
    line_to_char,
    get_byte,
    get_char,
    get_line,
    chunk_at_byte,
    chunk_at_byte_slice,
    chunk_at_char,
    chunk_at_char_slice,
    chunk_at_line_break,
    chunk_at_line_break_slice,
    slice,
    slice_small,
    slice_from_small_rope,
    slice_whole_rope,
    slice_whole_slice
);
criterion_main!(benches);
