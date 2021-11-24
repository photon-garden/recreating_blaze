use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nannou::prelude::*;

type Points = Vec<Point2<f32>>;

fn smooth(points: &Points) -> Points {
    let last_index = points.len() - 1;
    let num_line_segments = points.len() - 1;

    // Each line segment creates two points.
    let mut smoothed: Points = Vec::with_capacity(num_line_segments * 2);

    for index in 0..last_index {
        let point = points[index];
        let next_point = points[index + 1];

        let near = point * 0.75 + next_point * 0.25;
        let far = point * 0.25 + next_point * 0.75;

        smoothed.push(near);
        smoothed.push(far);
    }

    smoothed
}

fn criterion_benchmark(c: &mut Criterion) {
    let points: Points = (0..10_000)
        .map(|_| pt2(random_f32(), random_f32()))
        .collect();

    c.bench_function("smooth", |b| b.iter(|| smooth(black_box(&points))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
