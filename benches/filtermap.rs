use criterion::{criterion_group, criterion_main, Criterion};
use rust_samples::waitwhat::filtermap::*;
use std::time::Duration;

const N: u64 = 1_100_000;

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(Duration::from_secs(10))
}

fn bench_do_filter_map(c: &mut Criterion) {
    c.bench_function("do_filter_map", |b| {
        b.iter(|| do_filter_map(N).collect::<Vec<_>>())
    });
}

fn bench_do_flat_map_option(c: &mut Criterion) {
    c.bench_function("do_flat_map_option", |b| {
        b.iter(|| do_flat_map_option(N).collect::<Vec<_>>())
    });
}

fn bench_do_flat_map_vec(c: &mut Criterion) {
    c.bench_function("do_flat_map_vec", |b| {
        b.iter(|| do_flat_map_vec(N).collect::<Vec<_>>())
    });
}

fn bench_do_flat_map_array(c: &mut Criterion) {
    c.bench_function("do_flat_map_array", |b| {
        b.iter(|| do_flat_map_array(N).collect::<Vec<_>>())
    });
}

fn bench_do_flat_map_wrapper(c: &mut Criterion) {
    c.bench_function("do_flat_map_wrapper", |b| {
        b.iter(|| do_flat_map_wrapper(N).collect::<Vec<_>>())
    });
}

fn bench_do_flat_map_maybe(c: &mut Criterion) {
    c.bench_function("do_flat_map_maybe", |b| {
        b.iter(|| do_flat_map_maybe(N).collect::<Vec<_>>())
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets =
        bench_do_filter_map,
        bench_do_flat_map_option,
        bench_do_flat_map_array,
        bench_do_flat_map_vec,
        bench_do_flat_map_maybe,
        bench_do_flat_map_wrapper
);
criterion_main!(benches);
