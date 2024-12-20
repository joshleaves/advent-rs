use advent_rs::year_2015::day_01;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::mem;
use std::time::Duration;

fn day_01_v1_naive(input: &str) -> i16 {
  let right: i16 = input.matches('(').count() as i16;
  let left: i16 = input.matches(')').count() as i16;

  right - left
}

fn day_01_v1_normal(input: &str) -> i16 {
  input
    .chars()
    .filter_map(|chr| match chr {
      '(' => Some(1),
      ')' => Some(-1),
      _ => None,
    })
    .sum::<i16>()
}

fn day_01_v1_fast(input: impl Into<String>) -> i16 {
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

fn day_01_v2_fast(input: impl Into<String>) -> i16 {
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

fn day_01_v2_flash(input: impl Into<String>) -> i16 {
  unsafe { mem::transmute::<String, Vec<u16>>(input.into()) }
    .iter()
    .try_fold((0, 0), |mut acc, pair| {
      match (acc.0, pair) {
        // Case for ")" or "))" or "()"
        (0, 41) | (0, 10537) | (0, 10281) => return Err((acc.1) + 1),

        // Case for "("
        (_, 40) => acc.0 += 1,
        // Case for ")"
        (_, 41) => acc.0 -= 1,
        // Case for "(("
        (_, 10280) => acc.0 += 2,
        // Case for "))"
        (_, 10537) => acc.0 -= 2,
        // Case for ")(" or "()"
        (_, 10536) | (_, 10281) => (),
        _ => (),
      }
      acc.1 += 2;
      Ok(acc)
    })
    .and::<i16>(Err(0))
    .unwrap_err()
}

fn bench_year_2015_day_01_v1(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2015/day_01_input");
  assert_eq!(day_01::day_01_v1(input), 138);
  assert_eq!(day_01_v1_naive(input), 138);
  assert_eq!(day_01_v1_normal(input), 138);
  assert_eq!(day_01_v1_fast(input), 138);

  let mut group = c.benchmark_group("year_2015::day_01_v1");
  group.warm_up_time(Duration::from_millis(100));
  group.bench_with_input(BenchmarkId::new("Base", input.len()), input, |b, input| {
    b.iter(|| day_01::day_01_v1(input))
  });

  group.bench_with_input(BenchmarkId::new("Naive", input.len()), input, |b, input| {
    b.iter(|| day_01_v1_naive(input))
  });
  group.bench_with_input(
    BenchmarkId::new("Normal", input.len()),
    input,
    |b, input| b.iter(|| day_01_v1_normal(input)),
  );
  group.bench_with_input(BenchmarkId::new("Fast", input.len()), input, |b, input| {
    b.iter(|| day_01_v1_fast(input))
  });
  group.finish();
}

fn bench_year_2015_day_01_v2(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2015/day_01_input");
  assert_eq!(day_01::day_01_v2(input), 1_771);
  assert_eq!(day_01_v2_fast(input), 1_771);
  assert_eq!(day_01_v2_flash(input), 1_771);

  let mut group = c.benchmark_group("year_2015::day_01_v2");
  group.warm_up_time(Duration::from_millis(100));

  group.bench_with_input(BenchmarkId::new("Base", input.len()), input, |b, input| {
    b.iter(|| day_01::day_01_v2(input))
  });
  group.bench_with_input(BenchmarkId::new("Fast", input.len()), input, |b, input| {
    b.iter(|| day_01_v2_fast(input))
  });
  group.bench_with_input(BenchmarkId::new("Flash", input.len()), input, |b, input| {
    b.iter(|| day_01_v2_flash(input))
  });
  group.finish();
}

criterion_group!(
  bench_year_2015_day_01,
  bench_year_2015_day_01_v1,
  bench_year_2015_day_01_v2
);
criterion_main!(bench_year_2015_day_01);
