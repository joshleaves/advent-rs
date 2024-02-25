use advent_rs::year_2015::day_01;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn day_01_v1_slow(input: &str) -> i16 {
  let right: Vec<&str> = input.matches("(").collect();
  let left: Vec<&str> = input.matches(")").collect();
  let rlen = right.len() as i16;
  let llen = left.len() as i16;

  rlen - llen
}

pub fn bench_year_2015_day_01_v1(c: &mut Criterion) {
  let mut group = c.benchmark_group("year_2015::day_01_v1");
  group.warm_up_time(Duration::from_millis(100));
  let input = include_str!("../inputs/year_2015_day_01_input");
  group.bench_with_input(BenchmarkId::new("Slow", input.len()), input, |b, input| {
    b.iter(|| day_01_v1_slow(input))
  });
  group.bench_with_input(BenchmarkId::new("Fast", input.len()), input, |b, input| {
    b.iter(|| day_01::day_01_v1(input))
  });
  group.finish();
}

criterion_group!(bench_year_2015_day_01, bench_year_2015_day_01_v1);
criterion_main!(bench_year_2015_day_01);
