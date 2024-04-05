use advent_rs::year_2017::day_01;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn day_01_v2_naive(input: impl Into<String>) -> u16 {
  let characters = input.into().trim_end().as_bytes().to_vec();
  let length = characters.len();
  let dem_length = length / 2;
  characters
    .iter()
    .enumerate()
    .filter_map(|(idx, chr)| {
      if *chr == characters[(idx + dem_length) % length] {
        Some((*chr - 48) as u16)
      } else {
        None
      }
    })
    .sum()
}

fn day_01_v2_zip(input: impl Into<String>) -> u16 {
  let characters = input.into().trim_end().as_bytes().to_vec();
  characters
    .iter()
    .zip(characters.iter().cycle().skip(characters.len() / 2))
    .filter_map(|(lhs, rhs)| {
      if lhs == rhs {
        Some((lhs - 48) as u16)
      } else {
        None
      }
    })
    .sum()
}

fn bench_year_2017_day_01_v2(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_01_input");
  assert_eq!(day_01_v2_naive(input), 1_268);
  assert_eq!(day_01_v2_zip(input), 1_268);
  assert_eq!(day_01::day_01_v2(input), 1_268);

  let mut group = c.benchmark_group("year_2017::day_01_v2");
  group.warm_up_time(Duration::from_millis(100));
  group.bench_with_input(BenchmarkId::new("Naive", input.len()), input, |b, input| {
    b.iter(|| day_01_v2_naive(input))
  });
  group.bench_with_input(BenchmarkId::new("Zip", input.len()), input, |b, input| {
    b.iter(|| day_01_v2_zip(input))
  });
  group.bench_with_input(BenchmarkId::new("Base", input.len()), input, |b, input| {
    b.iter(|| day_01::day_01_v2(input))
  });

  group.finish();
}

criterion_group!(bench_year_2017_day_01, bench_year_2017_day_01_v2);
criterion_main!(bench_year_2017_day_01);
