//! Benchmarks for harness_utils

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harness_utils::{hash_str, is_palindrome, parse_kv, parse_tags};

fn bench_hash_str_short(c: &mut Criterion) {
    c.bench_function("hash_str_short", |b| {
        b.iter(|| hash_str(black_box("short")));
    });
}

fn bench_hash_str_medium(c: &mut Criterion) {
    c.bench_function("hash_str_medium", |b| {
        b.iter(|| hash_str(black_box("this is a medium length string for hashing")));
    });
}

fn bench_hash_str_long(c: &mut Criterion) {
    let long_string = "x".repeat(1000);
    c.bench_function("hash_str_long", |b| {
        b.iter(|| hash_str(black_box(&long_string)));
    });
}

fn bench_hash_str_repeat(c: &mut Criterion) {
    c.bench_function("hash_str_repeat", |b| {
        b.iter(|| {
            hash_str(black_box("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"))
        });
    });
}

fn bench_parse_kv_simple(c: &mut Criterion) {
    c.bench_function("parse_kv_simple", |b| {
        b.iter(|| parse_kv(black_box("a=1,b=2,c=3"), black_box(','), black_box('=')));
    });
}

fn bench_parse_kv_with_spaces(c: &mut Criterion) {
    c.bench_function("parse_kv_with_spaces", |b| {
        b.iter(|| parse_kv(black_box(" a = 1 , b = 2 , c = 3 "), black_box(','), black_box('=')));
    });
}

fn bench_parse_kv_many_pairs(c: &mut Criterion) {
    c.bench_function("parse_kv_many_pairs", |b| {
        b.iter(|| {
            parse_kv(
                black_box("a=1,b=2,c=3,d=4,e=5,f=6,g=7,h=8,i=9,j=10"),
                black_box(','),
                black_box('='),
            )
        });
    });
}

fn bench_parse_kv_invalid(c: &mut Criterion) {
    c.bench_function("parse_kv_invalid", |b| {
        b.iter(|| parse_kv(black_box("invalid"), black_box(','), black_box('=')));
    });
}

fn bench_parse_tags_simple(c: &mut Criterion) {
    c.bench_function("parse_tags_simple", |b| {
        b.iter(|| parse_tags(black_box("tag1,tag2,tag3")));
    });
}

fn bench_parse_tags_with_spaces(c: &mut Criterion) {
    c.bench_function("parse_tags_with_spaces", |b| {
        b.iter(|| parse_tags(black_box(" tag1 , tag2 , tag3 ")));
    });
}

fn bench_parse_tags_many(c: &mut Criterion) {
    c.bench_function("parse_tags_many", |b| {
        b.iter(|| parse_tags(black_box("tag1,tag2,tag3,tag4,tag5,tag6,tag7,tag8,tag9,tag10")));
    });
}

fn bench_parse_tags_empty(c: &mut Criterion) {
    c.bench_function("parse_tags_empty", |b| {
        b.iter(|| parse_tags(black_box("")));
    });
}

fn bench_is_palindrome_true(c: &mut Criterion) {
    c.bench_function("is_palindrome_true", |b| {
        b.iter(|| is_palindrome(black_box("radar")));
    });
}

fn bench_is_palindrome_false(c: &mut Criterion) {
    c.bench_function("is_palindrome_false", |b| {
        b.iter(|| is_palindrome(black_box("hello")));
    });
}

fn bench_is_palindrome_empty(c: &mut Criterion) {
    c.bench_function("is_palindrome_empty", |b| {
        b.iter(|| is_palindrome(black_box("")));
    });
}

fn bench_is_palindrome_single_char(c: &mut Criterion) {
    c.bench_function("is_palindrome_single_char", |b| {
        b.iter(|| is_palindrome(black_box("a")));
    });
}

fn bench_is_palindrome_long(c: &mut Criterion) {
    let long_palindrome = "A".repeat(500) + &"B".repeat(500);
    let _ = long_palindrome.clone();
    c.bench_function("is_palindrome_long", |b| {
        b.iter(|| is_palindrome(black_box(&long_palindrome)));
    });
}

criterion_group!(
    benches,
    bench_hash_str_short,
    bench_hash_str_medium,
    bench_hash_str_long,
    bench_hash_str_repeat,
    bench_parse_kv_simple,
    bench_parse_kv_with_spaces,
    bench_parse_kv_many_pairs,
    bench_parse_kv_invalid,
    bench_parse_tags_simple,
    bench_parse_tags_with_spaces,
    bench_parse_tags_many,
    bench_parse_tags_empty,
    bench_is_palindrome_true,
    bench_is_palindrome_false,
    bench_is_palindrome_empty,
    bench_is_palindrome_single_char,
    bench_is_palindrome_long
);
criterion_main!(benches);
