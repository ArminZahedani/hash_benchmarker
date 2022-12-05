use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hash_benchmarker::{blake3_hash, sha2_hash, sha3_hash};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_sha2() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    sha2_hash(&rand_string);
}

pub fn random_sha3() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    sha3_hash(&rand_string);
}

pub fn random_blake3() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    blake3_hash(&rand_string);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Sha256 Random", |b| b.iter(|| random_sha2()));
    c.bench_function("Sha3 Random", |b| b.iter(|| random_sha3()));
    c.bench_function("Blake3 Random", |b| b.iter(|| random_blake3()));

    c.bench_function("Sha256 Fixed", |b| b.iter(|| sha2_hash(black_box("abc"))));
    c.bench_function("Sha3 Fixed", |b| b.iter(|| sha3_hash(black_box("abc"))));
    c.bench_function("Blake3 Fixed", |b| b.iter(|| blake3_hash(black_box("abc"))));
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
