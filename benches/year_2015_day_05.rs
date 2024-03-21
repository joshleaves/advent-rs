use advent_rs::year_2015::day_05;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn string_is_nice_v1_contains(input: &str) -> bool {
  if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy") {
    return false;
  }
  let mut vowels = 0;
  let mut repeated = false;
  input.chars().fold(' ', |acc, elt| {
    if acc == elt {
      repeated = true
    }
    if elt == 'a' || elt == 'e' || elt == 'i' || elt == 'o' || elt == 'u' {
      vowels += 1
    }
    elt
  });

  return repeated && vowels >= 3;
}

fn day_05_v1_contains(input: &str) -> u32 {
  return input
    .lines()
    .filter(|line| string_is_nice_v1_contains(line))
    .count() as u32;
}

fn bench_year_2015_day_05_v1(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2015/day_05_input");
  let mut group = c.benchmark_group("year_2015::day_05_v1");
  group.warm_up_time(Duration::from_millis(100));
  group.bench_with_input(
    BenchmarkId::new("contains", input.len()),
    input,
    |b, input| b.iter(|| day_05_v1_contains(input)),
  );
  group.bench_with_input(BenchmarkId::new("chars", input.len()), input, |b, input| {
    b.iter(|| day_05::day_05_v1(input))
  });
  group.finish();
}

criterion_group!(bench_year_2015_day_05, bench_year_2015_day_05_v1);
criterion_main!(bench_year_2015_day_05);
