use advent_rs::year_2016::day_01;
use advent_rs::year_2016::day_02;
use advent_rs::year_2016::day_03;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn year_2016_day_01(c: &mut Criterion) {
  let mut g2016_day_01 = c.benchmark_group("year_2016::day_01");
  let input_year_2016_day_01 = include_str!("../inputs/year_2016/day_01_input");
  g2016_day_01.bench_function("year_2016::day_01_v1", |b| {
    b.iter(|| day_01::day_01_v1(black_box(input_year_2016_day_01)))
  });
  g2016_day_01.bench_function("year_2016::day_01_v2", |b| {
    b.iter(|| day_01::day_01_v2(black_box(input_year_2016_day_01)))
  });
  g2016_day_01.finish();
}

pub fn year_2016_day_02(c: &mut Criterion) {
  let mut g2016_day_02 = c.benchmark_group("year_2016::day_02");
  let input_year_2016_day_02 = include_str!("../inputs/year_2016/day_02_input");
  g2016_day_02.bench_function("year_2016::day_02_v1", |b| {
    b.iter(|| day_02::day_02_v1(black_box(input_year_2016_day_02)))
  });
  g2016_day_02.bench_function("year_2016::day_02_v2", |b| {
    b.iter(|| day_02::day_02_v2(black_box(input_year_2016_day_02)))
  });
  g2016_day_02.finish();
}

pub fn year_2016_day_03(c: &mut Criterion) {
  let mut g2016_day_03 = c.benchmark_group("year_2016::day_03");
  let input_year_2016_day_03 = include_str!("../inputs/year_2016/day_03_input");
  g2016_day_03.bench_function("year_2016::day_03_v1", |b| {
    b.iter(|| day_03::day_03_v1(black_box(input_year_2016_day_03)))
  });
  g2016_day_03.bench_function("year_2016::day_03_v2", |b| {
    b.iter(|| day_03::day_03_v2(black_box(input_year_2016_day_03)))
  });
  g2016_day_03.finish();
}

criterion_group!(
  benches,
  year_2016_day_01,
  year_2016_day_02,
  year_2016_day_03
);
criterion_main!(benches);
