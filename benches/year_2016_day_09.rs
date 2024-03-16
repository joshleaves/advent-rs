use advent_rs::year_2016::day_09;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use regex::Regex;
use std::time::Duration;

// This snippet was taken from fornwall
// SRC: https://github.com/fornwall/advent-of-code/blob/main/crates/core/src/year2016/day09.rs
fn fornwall_uncompressed_size(text: &[u8], recursive: bool) -> Result<u64, String> {
  let error_mapper_uf8 = |_| "Invalid input";
  let error_mapper_parse = |_| "Invalid input";
  let mut start_parenthesis_idx = None;
  let mut uncompressed_len = 0_u64;

  let mut i = 0;
  while i < text.len() {
    let c = text[i];
    if c == b'(' {
      start_parenthesis_idx = Some(i);
    } else if c == b')' {
      if let Some(from) = start_parenthesis_idx {
        let inside_parenthesis = &text[from + 1..i];
        let parts = inside_parenthesis
          .split(|&c| c == b'x')
          .collect::<Vec<&[u8]>>();
        if parts.len() != 2 {
          return Err("Invalid input".into());
        }
        let chars_to_take = std::str::from_utf8(parts[0])
          .map_err(error_mapper_uf8)?
          .parse::<u64>()
          .map_err(error_mapper_parse)?;
        let repetitions = std::str::from_utf8(parts[1])
          .map_err(error_mapper_uf8)?
          .parse::<u64>()
          .map_err(error_mapper_parse)?;
        uncompressed_len += repetitions
          * if recursive {
            fornwall_uncompressed_size(&text[i + 1..i + 1 + chars_to_take as usize], true)?
          } else {
            chars_to_take
          };
        i += chars_to_take as usize;
        start_parenthesis_idx = None;
      }
    } else if start_parenthesis_idx.is_none() {
      uncompressed_len += 1;
    }
    i += 1;
  }

  Ok(uncompressed_len)
}

// This snippet was taken from galenelias:
// SRC: https://github.com/galenelias/AdventOfCode_2016/blob/master/src/Day9/mod.rs
fn galenelias_decompress(mut input: &str, recursive: bool) -> String {
  let mut result = String::new();
  let repeat_regex: Regex = Regex::new(r"([^(]*)\((\d+)x(\d+)\)").unwrap();

  while !input.is_empty() {
    if let Some(m) = repeat_regex.captures(input) {
      let rep_len = m.get(2).unwrap().as_str().parse::<usize>().unwrap();
      let rep_count = m.get(3).unwrap().as_str().parse::<usize>().unwrap();
      let match_end = m.get(0).unwrap().end();

      // Push the prefix into the result
      result.push_str(m.get(1).unwrap().as_str());
      if !recursive {
        result.push_str(&input[match_end..match_end + rep_len].repeat(rep_count));
      } else {
        result.push_str(
          &galenelias_decompress(&input[match_end..match_end + rep_len], true).repeat(rep_count),
        );
      }
      input = &input[match_end + rep_len..];
    } else {
      result.push_str(input);
      return result;
    }
  }

  return result;
}

pub fn bench_year_2016_day_09_v1(c: &mut Criterion) {
  let mut group = c.benchmark_group("year_2016::day_09_v1");
  group.warm_up_time(Duration::from_millis(100));
  let input = include_str!("../inputs/year_2016/day_09_input");

  group.bench_with_input(
    BenchmarkId::new("fornwall", input.len()),
    input,
    |b, input| b.iter(|| fornwall_uncompressed_size(input.as_bytes(), false)),
  );
  group.bench_with_input(
    BenchmarkId::new("galenelias", input.len()),
    input,
    |b, input| b.iter(|| galenelias_decompress(&input, false)),
  );
  group.bench_with_input(
    BenchmarkId::new("joshleaves", input.len()),
    input,
    |b, input| b.iter(|| day_09::day_09_v1(input)),
  );
  group.finish();
}

pub fn bench_year_2016_day_09_v2(c: &mut Criterion) {
  let mut group = c.benchmark_group("year_2016::day_09_v2");
  group.warm_up_time(Duration::from_millis(100));
  let input = include_str!("../inputs/year_2016/day_09_input");

  group.bench_with_input(
    BenchmarkId::new("fornwall", input.len()),
    input,
    |b, input| b.iter(|| fornwall_uncompressed_size(input.as_bytes(), true)),
  );
  group.bench_with_input(
    BenchmarkId::new("galenelias", input.len()),
    input,
    |b, input| b.iter(|| galenelias_decompress(&input, true)),
  );
  group.bench_with_input(
    BenchmarkId::new("joshleaves", input.len()),
    input,
    |b, input| b.iter(|| day_09::day_09_v2(input)),
  );
  group.finish();
}

criterion_group!(
  bench_year_2016_day_09,
  bench_year_2016_day_09_v1,
  bench_year_2016_day_09_v2
);
criterion_main!(bench_year_2016_day_09);
