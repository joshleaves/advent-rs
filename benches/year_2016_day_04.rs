use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, time::Duration};

fn verify_checksum_naive(checksum: &str, input: &str) -> bool {
  let mut letters_cnt: HashMap<char, u8> = HashMap::new();
  for chr in input.chars() {
    if chr != '-' {
      let counter = letters_cnt.entry(chr).or_insert(0);
      *counter += 1;
    }
  }

  let letters = input
    .chars()
    .unique()
    .sorted_by_key(|chr| {
      let cnt_chr = if *chr == '-' {
        0
      } else {
        *letters_cnt.get(chr).unwrap()
      };
      [
        cnt_chr as i8 * -1,
        *chr as i8,
      ]
    })
    .take(5)
    .collect::<String>();

  letters == checksum
}

fn verify_checksum_fast(checksum: &str, input: &str) -> bool {
  let letters = input
    .chars()
    .unique()
    .sorted_by_key(|chr| {
      let cnt_chr = if *chr == '-' {
        0
      } else {
        input.chars().filter(|in_chr| in_chr == chr).count()
      };
      [
        cnt_chr as i8 * -1,
        *chr as i8,
      ]
    })
    .take(5)
    .collect::<String>();

  letters == checksum
}

fn day_04_v1_naive(input: impl Into<String>) -> u32 {
  let re = Regex::new(r"(?<letters>[\w-]+)-(?<value>\d+)\[(?<checksum>\w+)\]").unwrap();
  let mut result: u32 = 0;
  for line in input.into().lines() {
    let Some(caps) = re.captures(line) else {
      panic!("Incorrect input: {}", line);
    };
    if verify_checksum_naive(&caps["checksum"], &caps["letters"]) {
      result += caps["value"].parse::<u32>().unwrap();
    }
  }
  result
}

fn day_04_v1_fast(input: impl Into<String>) -> u32 {
  let re = Regex::new(r"(?<letters>[\w-]+)-(?<value>\d+)\[(?<checksum>\w+)\]").unwrap();
  let mut result: u32 = 0;
  for line in input.into().lines() {
    let Some(caps) = re.captures(line) else {
      panic!("Incorrect input: {}", line);
    };
    if verify_checksum_fast(&caps["checksum"], &caps["letters"]) {
      result += caps["value"].parse::<u32>().unwrap();
    }
  }
  result
}

fn bench_year_2016_day_04_checksum(c: &mut Criterion) {
  let mut group = c.benchmark_group("year_2016::day_04_v1");
  group.warm_up_time(Duration::from_millis(100));
  let input = include_str!("../inputs/year_2016/day_04_input");
  group.bench_with_input(BenchmarkId::new("Naive", input.len()), input, |b, input| {
    b.iter(|| day_04_v1_naive(input))
  });
  group.bench_with_input(BenchmarkId::new("Fast", input.len()), input, |b, input| {
    b.iter(|| day_04_v1_fast(input))
  });
  group.finish();
}

criterion_group!(bench_year_2016_day_04, bench_year_2016_day_04_checksum);
criterion_main!(bench_year_2016_day_04);
