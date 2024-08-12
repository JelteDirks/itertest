use criterion::{criterion_group, criterion_main, Criterion};
use std::io::Read;

extern crate itertest;

use itertest::decoder::{self, HeaderDecoderExt, LineBoundariesExt};

fn decode_v1(bytes: &[u8]) {
    let mut x: Vec<_> = Vec::default();
    for boundaries in decoder::LineBoundaries::new(bytes.iter()) {
        let h = decoder::decode_header(bytes, boundaries.0, boundaries.1);
        x.push(h);
    }
    std::hint::black_box(x);
}

fn decode_v2(bytes: &[u8]) {
    let x: Vec<_> = bytes.iter().line_boundaries().decode_headers(bytes).collect::<_>();
    std::hint::black_box(x);
}

fn benchmark(c: &mut Criterion) {
    let mut buf: Vec<u8> = Vec::new();

    let file = std::fs::File::open("response_example.txt").expect("file not found");
    let mut bufreader = std::io::BufReader::new(file);
    bufreader.read_to_end(&mut buf).expect("file read error");

    let bytes: &[u8] = &buf;

    c.bench_function("decode_v1", |b| b.iter(|| decode_v1(bytes)));
    c.bench_function("decode_v2", |b| b.iter(|| decode_v2(bytes)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
