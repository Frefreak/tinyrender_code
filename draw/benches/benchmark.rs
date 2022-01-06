use draw::{line1, line2};
use draw::triangle1;
use tgaimage::{TGAImage, WHITE, RED, GREEN};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark_line(c: &mut Criterion) {
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

fn criterion_benchmark_triangle(c: &mut Criterion) {
    let mut image = TGAImage::new(200, 200);
    let t0 = vec![(10, 70), (50, 160), (70, 80)];
    let t1 = vec![(180, 50), (150, 1), (70, 180)];
    let t2 = vec![(180, 150), (120, 160), (130, 180)];
    c.bench_function("triangle1", |b| b.iter(|| {
        triangle1(black_box(t0[0]), black_box(t0[1]), black_box(t0[2]), &mut image, RED);
        triangle1(black_box(t1[0]), black_box(t1[1]), black_box(t1[2]), &mut image, WHITE);
        triangle1(black_box(t2[0]), black_box(t2[1]), black_box(t2[2]), &mut image, GREEN);
    }));
}

criterion_group!(benches_line, criterion_benchmark_line);
criterion_group!(benches_triangle, criterion_benchmark_triangle);
criterion_main!(benches_line, benches_triangle);
