use advent_rs::year_2017::day_13;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use std::time::Duration;

// Found here: https://colab.research.google.com/github/hhoppe/advent_of_code/blob/main/2017/advent_of_code_2017.ipynb#scrollTo=84e08a8d-ca2e-4bc7-99ca-8a29a98d0bf9&line=15&uniqifier=1
pub fn day_13_v2_sieve(input: impl Into<String>) -> u32 {
  let firewalls: Vec<_> = input
    .into()
    .lines()
    .map(|line| {
      let mut tuple: (i32, i32) = line
        .split(": ")
        .map(|num| num.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
      tuple.1 = (tuple.1 - 1) * 2;
      tuple
    })
    .collect();

  let mut delay = 0;
  let chunk: i32 = 200_000;
  loop {
    let mut ok = vec![true; chunk as usize];
    for (depth, range) in &firewalls {
      let first = (-(delay as i32 + *depth as i32))
        .rem_euclid(*range as i32)
        .abs();
      for idx in (first..chunk).step_by(*range as usize) {
        ok[idx as usize] = false
      }
    }
    if let Some(index) = ok.iter().position(|v| *v == true) {
      return delay as u32 + index as u32;
    }
    delay += chunk as i32;
  }
}

fn bench_year_2017_day_13_v2(c: &mut Criterion) {
  let mut group = c.benchmark_group("year_2017::day_13_v2");
  group.warm_up_time(Duration::from_millis(100));
  let input = include_str!("../inputs/year_2017/day_13_input");
  assert_eq!(day_13_v2_sieve(input), 3_875_838);
  assert_eq!(day_13::day_13_v2(input), 3_875_838);
  group.bench_with_input(BenchmarkId::new("Sieve", input.len()), input, |b, input| {
    b.iter(|| day_13_v2_sieve(input))
  });
  group.bench_with_input(BenchmarkId::new("Base", input.len()), input, |b, input| {
    b.iter(|| day_13::day_13_v2(input))
  });

  group.finish();
}

criterion_group!(bench_year_2017_day_13, bench_year_2017_day_13_v2);
criterion_main!(bench_year_2017_day_13);
