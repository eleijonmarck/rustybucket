use criterion::{criterion_group, criterion_main, Criterion};
use rustybucket::series::new_series;

fn slice_a_series(x : usize, y : usize)  {
    let s = new_series(String::from("hej"), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let _ = &s[x..y];
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("slicing a series", |b| b.iter(|| slice_a_series(1, 5)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
