extern crate serde_json;
extern crate whatlang;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use whatlang::{detect, detect_script, divide_text_by_script, detect_langs};
use std::collections::HashMap;

fn criterion_benchmark(c: &mut Criterion) {
    let example_data = include_str!("../tests/examples.json");
    let examples: HashMap<String, String> = serde_json::from_str(example_data).unwrap();

    let mut group = c.benchmark_group("detect_langs_from_examples");
    for (key, example) in examples.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(key), example, |b, example| {
            b.iter(|| detect_langs(example));
        });
    }
    group.finish();
}

fn criterion_benchmark1(c: &mut Criterion) {
    let example_data = include_str!("../tests/examples.json");
    let examples: HashMap<String, String> = serde_json::from_str(example_data).unwrap();

    let mut group = c.benchmark_group("detect_from_examples");
    for (key, example) in examples.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(key), example, |b, example| {
            b.iter(|| detect(example));
        });
    }
    group.finish();
}

fn criterion_benchmark2(c: &mut Criterion) {
    let example_data = include_str!("../tests/examples.json");
    let examples: HashMap<String, String> = serde_json::from_str(example_data).unwrap();

    let mut group = c.benchmark_group("detect_script_from_examples");
    for (key, example) in examples.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(key), example, |b, example| {
            b.iter(|| detect_script(example));
        });
    }
    group.finish();
}

fn criterion_benchmark3(c: &mut Criterion) {
    let example_data = include_str!("../tests/examples.json");
    let examples: HashMap<String, String> = serde_json::from_str(example_data).unwrap();

    let mut group = c.benchmark_group("divide_text_by_script_from_examples");
    for (key, example) in examples.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(key), example, |b, example| {
            b.iter(|| divide_text_by_script(example));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark1, criterion_benchmark2, criterion_benchmark3);
criterion_main!(benches);
