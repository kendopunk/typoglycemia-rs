use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use typoglycemia::{typoglycemia, typoglycemia_leet};

// ── Single-word benchmarks ────────────────────────────────────────────────────
// These isolate scramble_word() for different word shapes:
//   short     : below the 3-grapheme floor (returned as-is, fast path)
//   plain     : typical 7-letter English word
//   long      : above the 15-grapheme ceiling (returned as-is, fast path)
//   apostrophe: triggers apostrophe normalization + split path
//   hyphenated: triggers hyphen split path
//   latin     : word with Latin-1 / accented boundary characters

fn bench_short_word(c: &mut Criterion) {
    c.bench_function("word/short (≤3 graphemes, fast path)", |b| {
        b.iter(|| typoglycemia(black_box("the")))
    });
}

fn bench_plain_word(c: &mut Criterion) {
    c.bench_function("word/plain 7-char", |b| {
        b.iter(|| typoglycemia(black_box("country")))
    });
}

fn bench_long_word(c: &mut Criterion) {
    c.bench_function("word/long (>15 graphemes, fast path)", |b| {
        b.iter(|| typoglycemia(black_box("antidisestablishmentarianism")))
    });
}

fn bench_apostrophe_word(c: &mut Criterion) {
    c.bench_function("word/apostrophe (doesn\u{2019}t)", |b| {
        b.iter(|| typoglycemia(black_box("doesn\u{2019}t")))
    });
}

fn bench_hyphenated_word(c: &mut Criterion) {
    c.bench_function("word/hyphenated (Spanish-speaking)", |b| {
        b.iter(|| typoglycemia(black_box("Spanish-speaking")))
    });
}

fn bench_latin_word(c: &mut Criterion) {
    c.bench_function("word/latin boundary (café)", |b| {
        b.iter(|| typoglycemia(black_box("café")))
    });
}

// ── Sentence benchmarks ───────────────────────────────────────────────────────
// These measure end-to-end throughput for typoglycemia() and typoglycemia_leet()
// on a realistic prose passage (the Cambridge typoglycemia paragraph, ~25 words).

const SAMPLE: &str = "According to a researcher at Cambridge University it doesn't matter in what \
     order the letters in a word are the only important thing is that the first \
     and last letter be at the right place the rest can be a total mess and you \
     can still read it without problem";

fn bench_sentence_typoglycemia(c: &mut Criterion) {
    c.bench_function("sentence/typoglycemia (~25 words)", |b| {
        b.iter(|| typoglycemia(black_box(SAMPLE)))
    });
}

fn bench_sentence_leet_level1(c: &mut Criterion) {
    c.bench_function("sentence/typoglycemia_leet level 1 (~25 words)", |b| {
        b.iter(|| typoglycemia_leet(black_box(SAMPLE), black_box(1)))
    });
}

fn bench_sentence_leet_level3(c: &mut Criterion) {
    c.bench_function("sentence/typoglycemia_leet level 3 (~25 words)", |b| {
        b.iter(|| typoglycemia_leet(black_box(SAMPLE), black_box(3)))
    });
}

// ── Groups ────────────────────────────────────────────────────────────────────

criterion_group!(
    word_benches,
    bench_short_word,
    bench_plain_word,
    bench_long_word,
    bench_apostrophe_word,
    bench_hyphenated_word,
    bench_latin_word,
);

criterion_group!(
    sentence_benches,
    bench_sentence_typoglycemia,
    bench_sentence_leet_level1,
    bench_sentence_leet_level3,
);

criterion_main!(word_benches, sentence_benches);
