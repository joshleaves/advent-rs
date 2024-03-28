use advent_rs::year_2017::day_04;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use std::{collections::HashSet, time::Duration};

pub fn day_04_v1_naive(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .filter(|line| {
      line
        .split_whitespace()
        .sorted()
        .group_by(|word| *word)
        .into_iter()
        .all(|(_word, group)| group.count() == 1)
    })
    .count() as u16
}

pub fn day_04_v1_sorted(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .filter(|line| {
      !line
        .split_whitespace()
        .sorted()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|pair| pair[0] == pair[1])
    })
    .count() as u16
}

pub fn day_04_v1_with_set(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .filter(|line| {
      let mut set = HashSet::new();
      for word in line.split_whitespace() {
        if set.contains(word) {
          return false;
        }
        set.insert(word);
      }
      true
    })
    .count() as u16
}

fn bench_year_2017_day_04_v1(c: &mut Criterion) {
  let mut group = c.benchmark_group("year_2017::day_04_v1");
  group.warm_up_time(Duration::from_millis(100));
  let input = include_str!("../inputs/year_2017/day_04_input");
  assert_eq!(day_04::day_04_v1(input), 466);
  assert_eq!(day_04_v1_naive(input), 466);
  assert_eq!(day_04_v1_sorted(input), 466);
  assert_eq!(day_04_v1_with_set(input), 466);

  group.bench_with_input(BenchmarkId::new("Base", input.len()), input, |b, input| {
    b.iter(|| day_04::day_04_v1(input))
  });
  group.bench_with_input(BenchmarkId::new("Naive", input.len()), input, |b, input| {
    b.iter(|| day_04_v1_naive(input))
  });
  group.bench_with_input(
    BenchmarkId::new("Sorted", input.len()),
    input,
    |b, input| b.iter(|| day_04_v1_sorted(input)),
  );
  group.bench_with_input(
    BenchmarkId::new("WithSet", input.len()),
    input,
    |b, input| b.iter(|| day_04_v1_with_set(input)),
  );

  group.finish();
}

criterion_group!(bench_year_2017_day_04, bench_year_2017_day_04_v1);
criterion_main!(bench_year_2017_day_04);
