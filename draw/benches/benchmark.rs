use draw::{line1, line2};
use tgaimage::{TGAImage, WHITE, RED};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut image = TGAImage::new(100, 100);

    c.bench_function("line1", |b| b.iter(|| {
        line1(black_box(13), black_box(20), black_box(80), black_box(40), black_box(&mut image), black_box(WHITE));
        line1(black_box(20), black_box(13), black_box(40), black_box(80), black_box(&mut image), black_box(RED));
        line1(black_box(80), black_box(40), black_box(13), black_box(20), black_box(&mut image), black_box(RED));
        }
    ));

    c.bench_function("line2", |b| b.iter(|| {
        line2(black_box(13), black_box(20), black_box(80), black_box(40), black_box(&mut image), black_box(WHITE));
        line2(black_box(20), black_box(13), black_box(40), black_box(80), black_box(&mut image), black_box(RED));
        line2(black_box(80), black_box(40), black_box(13), black_box(20), black_box(&mut image), black_box(RED));
        }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
