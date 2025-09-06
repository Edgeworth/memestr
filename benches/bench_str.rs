use std::hint::black_box;
use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main};
use memestr::strn::StrN;

fn benchmark_methods(c: &mut Criterion) {
    let strn8 = StrN::<8>::from_literal("abcdefgh");
    let strn16 = StrN::<16>::from_literal("abcdefghabcdefgh");
    let strn24 = StrN::<24>::from_literal("abcdefghabcdefghabcdefgh");
    let strn32 = StrN::<32>::from_literal("abcdefghabcdefghabcdefghabcdefgh");

    c.bench_function("StrN<8> index", |b| b.iter(|| black_box(&strn8).index(3)));
    c.bench_function("StrN<16> index", |b| b.iter(|| black_box(&strn16).index(3)));
    c.bench_function("StrN<24> index", |b| b.iter(|| black_box(&strn24).index(3)));
    c.bench_function("StrN<32> index", |b| b.iter(|| black_box(&strn32).index(3)));

    c.bench_function("StrN<8> to_ascii_lowercase", |b| {
        b.iter(|| black_box(&strn8).to_ascii_lowercase())
    });
    c.bench_function("StrN<16> to_ascii_lowercase", |b| {
        b.iter(|| black_box(&strn16).to_ascii_lowercase())
    });
    c.bench_function("StrN<24> to_ascii_lowercase", |b| {
        b.iter(|| black_box(&strn24).to_ascii_lowercase())
    });
    c.bench_function("StrN<32> to_ascii_lowercase", |b| {
        b.iter(|| black_box(&strn32).to_ascii_lowercase())
    });

    c.bench_function("StrN<8> to_ascii_uppercase", |b| {
        b.iter(|| black_box(&strn8).to_ascii_uppercase())
    });
    c.bench_function("StrN<16> to_ascii_uppercase", |b| {
        b.iter(|| black_box(&strn16).to_ascii_uppercase())
    });
    c.bench_function("StrN<24> to_ascii_uppercase", |b| {
        b.iter(|| black_box(&strn24).to_ascii_uppercase())
    });
    c.bench_function("StrN<32> to_ascii_uppercase", |b| {
        b.iter(|| black_box(&strn32).to_ascii_uppercase())
    });

    c.bench_function("StrN<8> starts_with", |b| b.iter(|| black_box(&strn8).starts_with(strn8)));
    c.bench_function("StrN<16> starts_with", |b| b.iter(|| black_box(&strn16).starts_with(strn16)));
    c.bench_function("StrN<24> starts_with", |b| b.iter(|| black_box(&strn24).starts_with(strn24)));
    c.bench_function("StrN<32> starts_with", |b| b.iter(|| black_box(&strn32).starts_with(strn32)));

    c.bench_function("StrN<8> is_empty", |b| b.iter(|| black_box(&strn8).is_empty()));
    c.bench_function("StrN<16> is_empty", |b| b.iter(|| black_box(&strn16).is_empty()));
    c.bench_function("StrN<24> is_empty", |b| b.iter(|| black_box(&strn24).is_empty()));
    c.bench_function("StrN<32> is_empty", |b| b.iter(|| black_box(&strn32).is_empty()));

    c.bench_function("StrN<8> from_str", |b| {
        b.iter(|| StrN::<8>::from_str(black_box("abcdefgh")).unwrap())
    });
    c.bench_function("StrN<16> from_str", |b| {
        b.iter(|| StrN::<16>::from_str(black_box("abcdefghabcdefgh")).unwrap())
    });
    c.bench_function("StrN<24> from_str", |b| {
        b.iter(|| StrN::<24>::from_str(black_box("abcdefghabcdefghabcdefgh")).unwrap())
    });
    c.bench_function("StrN<32> from_str", |b| {
        b.iter(|| StrN::<32>::from_str(black_box("abcdefghabcdefghabcdefghabcdefgh")).unwrap())
    });
    c.bench_function("StrN<8> String::from", |b| b.iter(|| String::from(black_box(strn8))));
    c.bench_function("StrN<16> String::from", |b| b.iter(|| String::from(black_box(strn16))));
    c.bench_function("StrN<24> String::from", |b| b.iter(|| String::from(black_box(strn24))));
    c.bench_function("StrN<32> String::from", |b| b.iter(|| String::from(black_box(strn32))));
}

criterion_group!(benches, benchmark_methods);
criterion_main!(benches);
