use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use json_typings::{read_json, Typing};

fn bench_typing(c: &mut Criterion) {
    let value = read_json::file(&Path::new("./data/sample_b.json")).unwrap();

    if let Ok(a) = value.into_array() {
        c.bench_function("from_items", |b| {
            b.iter(|| Typing::from_items("Typing", black_box(a.clone())))
        });
    }
}

criterion_group!(benches, bench_typing);
criterion_main!(benches);
