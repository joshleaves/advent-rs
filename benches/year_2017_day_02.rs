use advent_rs::year_2017::day_02;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use std::time::Duration;

pub fn day_02_v1_naive(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .map(|line| {
      let numbers: Vec<_> = line
        .split_whitespace()
        .map(|number| number.parse::<u16>().unwrap())
        .sorted()
        .collect();
      numbers.iter().max().unwrap() - numbers.iter().min().unwrap()
    })
    .sum()
}

fn bench_year_2017_day_02_v1(c: &mut Criterion) {
  let mut group = c.benchmark_group("year_2017::day_02_v1");
  group.warm_up_time(Duration::from_millis(100));
  let input = include_str!("../inputs/year_2017/day_02_input");
  assert_eq!(day_02::day_02_v1(input), 53_978);
  assert_eq!(day_02_v1_naive(input), 53_978);
  group.bench_with_input(BenchmarkId::new("Naive", input.len()), input, |b, input| {
    b.iter(|| day_02_v1_naive(input))
  });
  group.bench_with_input(BenchmarkId::new("Base", input.len()), input, |b, input| {
    b.iter(|| day_02::day_02_v1(input))
  });

  group.finish();
}

criterion_group!(bench_year_2017_day_02, bench_year_2017_day_02_v1);
criterion_main!(bench_year_2017_day_02);
