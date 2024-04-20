use capitalize::Capitalize;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const SAMPLE_TEXT: &str = include_str!("sample.txt");

pub fn capitalize_benchmark(c: &mut Criterion) {
    c.bench_function("capitalize lorem ipsum", |b| {
        b.iter(|| black_box(SAMPLE_TEXT).capitalize())
    });
}

pub fn capitalize_words_benchmark(c: &mut Criterion) {
    #[cfg(feature = "nightly")]
    c.bench_function("capitalize words lorem ipsum", |b| {
        b.iter(|| black_box(SAMPLE_TEXT).capitalize_words())
    });
}

criterion_group!(benches, capitalize_benchmark, capitalize_words_benchmark,);
criterion_main!(benches);
