use advent_rs::year_2015::day_01;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::mem;
use std::time::Duration;

fn day_01_v1_naive(input: &str) -> i16 {
  let right: i16 = input.matches("(").count() as i16;
  let left: i16 = input.matches(")").count() as i16;

  right - left
}

pub fn day_01_v1_fast(input: impl Into<String>) -> i16 {
  let mut level: i16 = 0;
  let nums: Vec<u16> = unsafe { mem::transmute::<String, Vec<u16>>(input.into()) };
  for num in nums {
    match num {
      // Case for "("
      40 => level += 1,
      // Case for ")"
      41 => level -= 1,
      // Case for "(("
      10280 => level += 2,
      // Case for "))"
      10537 => level -= 2,
      // Case for "()" and ")("
      10536 | 10281 => {}
      // Early return
      _ => {
        return level;
      }
    }
  }

  level
}

pub fn day_01_v2_fast(input: impl Into<String>) -> i16 {
  let mut level: i16 = 0;
  let nums: Vec<u16> = unsafe { mem::transmute::<String, Vec<u16>>(input.into()) };
  for (index, num) in nums.iter().enumerate() {
    match (level, num) {
      // Case for "("
      (_, 40) => level += 1,
      // Case for ")"
      (0, 41) => {
        return index as i16 * 2 + 1;
      }
      (_, 41) => level -= 1,
      // Case for "(("
      (_, 10280) => level += 2,
      // Case for "))"
      (0, 10537) => {
        return index as i16 * 2 + 1;
      }
      (_, 10537) => {
        level -= 2;
      }
      // Case for ")("
      (_, 10536) => {}
      // Case for "()"
      (0, 10281) => {
        return index as i16 * 2 + 1;
      }
      (_, 10281) => {}
      _ => {
        return 0;
      }
    }
    if level < 0 {
      return (index as i16 + 1) * 2;
    }
  }

  0
}

pub fn bench_year_2015_day_01_v1(c: &mut Criterion) {
  let mut group = c.benchmark_group("year_2015::day_01_v1");
  group.warm_up_time(Duration::from_millis(100));
  let input = include_str!("../inputs/year_2015/day_01_input");
  group.bench_with_input(BenchmarkId::new("Naive", input.len()), input, |b, input| {
    b.iter(|| day_01_v1_naive(input))
  });
  group.bench_with_input(
    BenchmarkId::new("Normal", input.len()),
    input,
    |b, input| b.iter(|| day_01::day_01_v1(input)),
  );
  group.bench_with_input(BenchmarkId::new("Fast", input.len()), input, |b, input| {
    b.iter(|| day_01_v1_fast(input))
  });
  group.finish();
}

pub fn bench_year_2015_day_01_v2(c: &mut Criterion) {
  let mut group = c.benchmark_group("year_2015::day_01_v2");
  group.warm_up_time(Duration::from_millis(100));
  let input = include_str!("../inputs/year_2015/day_01_input");
  group.bench_with_input(
    BenchmarkId::new("Normal", input.len()),
    input,
    |b, input| b.iter(|| day_01::day_01_v2(input)),
  );
  group.bench_with_input(BenchmarkId::new("Fast", input.len()), input, |b, input| {
    b.iter(|| day_01_v2_fast(input))
  });
  group.finish();
}

criterion_group!(
  bench_year_2015_day_01,
  bench_year_2015_day_01_v1,
  bench_year_2015_day_01_v2
);
criterion_main!(bench_year_2015_day_01);
