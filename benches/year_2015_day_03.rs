use advent_rs::year_2015::day_03;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::BTreeSet;
use std::time::Duration;

#[inline]
fn move_character(pos: (i8, i8), direction: char) -> (i8, i8) {
  let mut newpos = pos.clone();
  match direction {
    '>' => newpos.0 += 1,
    '<' => newpos.0 -= 1,
    'v' => newpos.1 += 1,
    '^' => newpos.1 -= 1,
    _ => panic!("Invalid direction character: {direction}"),
  }
  newpos
}

fn day_03_v1_bset(input: &str) -> usize {
  let mut santa: (i8, i8) = (0, 0);
  let mut houses = BTreeSet::from([santa]);

  for (_idx, chr) in input.chars().enumerate() {
    santa = move_character(santa, chr);
    houses.insert(santa);
  }
  return houses.len() as usize;
}

fn day_03_v2_bset(input: &str) -> usize {
  let mut santa: (i8, i8) = (0, 0);
  let mut robot: (i8, i8) = (0, 0);
  let mut houses = BTreeSet::from([santa]);
  let moves: Vec<char> = input.chars().collect();

  for chars in moves.chunks(2) {
    santa = move_character(santa, chars[0]);
    robot = move_character(robot, chars[1]);
    houses.insert(santa);
    houses.insert(robot);
  }
  return houses.len() as usize;
}

fn bench_year_2015_day_03_sets(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2015/day_03_input");
  let mut group_v1 = c.benchmark_group("year_2015::day_03_v1");
  group_v1.warm_up_time(Duration::from_millis(100));
  group_v1.bench_with_input(
    BenchmarkId::new("BTreeSet", input.len()),
    input,
    |b, input| b.iter(|| day_03_v1_bset(input)),
  );
  group_v1.bench_with_input(
    BenchmarkId::new("HashSet", input.len()),
    input,
    |b, input| b.iter(|| day_03::day_03_v1(input)),
  );
  group_v1.finish();

  let mut group_v2 = c.benchmark_group("year_2015::day_03_v2");
  group_v2.warm_up_time(Duration::from_millis(100));
  group_v2.bench_with_input(
    BenchmarkId::new("BTreeSet", input.len()),
    input,
    |b, input| b.iter(|| day_03_v2_bset(input)),
  );
  group_v2.bench_with_input(
    BenchmarkId::new("HashSet", input.len()),
    input,
    |b, input| b.iter(|| day_03::day_03_v2(input)),
  );
  group_v2.finish();
}

criterion_group!(bench_year_2015_day_03, bench_year_2015_day_03_sets);
criterion_main!(bench_year_2015_day_03);
