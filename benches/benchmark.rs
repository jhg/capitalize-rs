use criterion::{black_box, criterion_group, criterion_main, Criterion};
use capitalize::Capitalize;

pub fn capitalize_benchmark(c: &mut Criterion) {
    c.bench_function("capitalize hello world", |b| b.iter(|| {
        black_box("hello world").capitalize()
    }));
}

pub fn capitalize_words_benchmark(c: &mut Criterion) {
    c.bench_function("capitalize words hello world", |b| b.iter(|| {
        black_box("hello world").capitalize_words()
    }));
}

criterion_group!(benches,
    capitalize_benchmark,
    capitalize_words_benchmark,
);
criterion_main!(benches);
