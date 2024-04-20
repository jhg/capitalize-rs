use criterion::{black_box, criterion_group, criterion_main, Criterion};
use capitalize::Capitalize;

const SAMPLE_TEXT: &str = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec porttitor lacus semper sem convallis, ac ultricies libero aliquet. Aliquam iaculis nunc id scelerisque posuere. Mauris ultrices tellus ac porttitor convallis. Suspendisse potenti. Integer venenatis sem sed arcu tristique, scelerisque varius arcu rutrum. Integer eleifend, diam sed ullamcorper hendrerit, erat odio semper arcu, id congue lectus nibh non felis. In pellentesque suscipit tincidunt. Aenean tincidunt rutrum dapibus. Pellentesque finibus elit ut sapien consectetur accumsan. Vestibulum laoreet ligula eu quam fermentum, quis tempor ligula cursus.
Phasellus vulputate feugiat nibh a fermentum. In nunc purus, ornare id lacus id, varius consequat nunc. Nam dignissim lacinia risus, at facilisis turpis ullamcorper vel. Sed gravida fermentum porttitor. Pellentesque ultrices vehicula dignissim. Vivamus bibendum id quam quis pretium. Aenean a odio imperdiet massa pretium vestibulum a et urna. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Suspendisse lectus nisl, sodales maximus volutpat ut, rhoncus vel urna.
Curabitur et ligula ut urna cursus euismod. Morbi malesuada euismod lacus sed volutpat. Vestibulum non viverra tellus. Aenean enim nulla, tempor a ipsum sed, tempor vestibulum erat. Nullam tincidunt, dui sed mollis porta, libero urna commodo metus, ac commodo tellus lectus quis magna. Donec congue massa sit amet ex egestas, eu varius urna sodales. Vestibulum efficitur mattis pretium. Etiam tincidunt ex in sapien luctus, sit amet efficitur ligula vulputate. Mauris dictum eleifend mi.
Curabitur consectetur, justo et consectetur imperdiet, diam mi egestas tortor, eu tincidunt mi odio dictum massa. Proin id placerat metus, ut elementum quam. Phasellus laoreet, elit eu auctor pulvinar, nulla orci hendrerit nunc, a tempor enim odio sit amet ipsum. Pellentesque fermentum arcu sed ex mollis faucibus. Aliquam faucibus sagittis placerat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Pellentesque non congue dolor, in blandit nulla. Etiam rhoncus lorem tristique volutpat aliquet. Ut vestibulum nisl nisi, non consequat mi volutpat nec. Ut volutpat dictum gravida.
Quisque suscipit malesuada tortor eu bibendum. Donec a aliquam sem. Donec vel orci semper, condimentum sem in, aliquet lectus. Maecenas egestas maximus metus, id molestie erat pretium a. Aliquam erat volutpat. Aenean vulputate, ex tempus varius vulputate, arcu turpis fermentum quam, eu fermentum risus neque eget orci. Aliquam ultricies turpis non elit egestas, eget aliquam arcu gravida. Morbi at varius lacus.
";

pub fn capitalize_benchmark(c: &mut Criterion) {
    c.bench_function("capitalize hello world", |b| b.iter(|| {
        black_box(SAMPLE_TEXT).capitalize()
    }));
}

pub fn capitalize_words_benchmark(c: &mut Criterion) {
    #[cfg(feature = "nightly")]
    c.bench_function("capitalize words hello world", |b| b.iter(|| {
        black_box(SAMPLE_TEXT).capitalize_words()
    }));
}

criterion_group!(benches,
    capitalize_benchmark,
    capitalize_words_benchmark,
);
criterion_main!(benches);
