use advent_rs::year_2015::day_01;
use advent_rs::year_2015::day_02;
use advent_rs::year_2015::day_03;
use advent_rs::year_2015::day_04;
use advent_rs::year_2015::day_05;
use advent_rs::year_2015::day_06;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn year_2015_benchmark(c: &mut Criterion) {
  let warm_up_time = Duration::from_millis(100);
  let measurement_time = Duration::from_millis(2000);

  let mut g2015_01 = c.benchmark_group("year_2015::day_01");
  g2015_01.warm_up_time(warm_up_time);
  g2015_01.measurement_time(measurement_time);
  let input_year_2015_day_01 = include_str!("../inputs/year_2015_day_01_input");
  g2015_01.bench_function("year_2015::day_01_v1", |b| {
    b.iter(|| day_01::day_01_v1(black_box(input_year_2015_day_01)))
  });
  g2015_01.bench_function("year_2015::day_01_v2", |b| {
    b.iter(|| day_01::day_01_v2(black_box(input_year_2015_day_01)))
  });
  g2015_01.finish();

  let mut g2015_02 = c.benchmark_group("year_2015::day_02");
  g2015_02.warm_up_time(warm_up_time);
  g2015_02.measurement_time(measurement_time);
  let input_year_2015_day_02 = include_str!("../inputs/year_2015_day_02_input");
  g2015_02.bench_function("year_2015::day_02_v1", |b| {
    b.iter(|| day_02::day_02_v1(black_box(input_year_2015_day_02)))
  });
  g2015_02.bench_function("year_2015::day_02_v2", |b| {
    b.iter(|| day_02::day_02_v2(black_box(input_year_2015_day_02)))
  });
  g2015_02.finish();

  let mut g2015_03 = c.benchmark_group("year_2015::day_03");
  g2015_03.warm_up_time(warm_up_time);
  g2015_03.measurement_time(measurement_time);
  g2015_03.sample_size(60);
  let input_year_2015_day_03 = include_str!("../inputs/year_2015_day_03_input");
  g2015_03.bench_function("year_2015::day_03_v1", |b| {
    b.iter(|| day_03::day_03_v1(black_box(input_year_2015_day_03)))
  });
  g2015_03.bench_function("year_2015::day_03_v2", |b| {
    b.iter(|| day_03::day_03_v2(black_box(input_year_2015_day_03)))
  });
  g2015_03.finish();

  let mut g2015_04 = c.benchmark_group("year_2015::day_04");
  g2015_04.warm_up_time(warm_up_time);
  // g2015_04.measurement_time(measurement_time);
  g2015_04.sample_size(10);
  let input_year_2015_day_04 = include_str!("../inputs/year_2015_day_04_input");
  g2015_04.bench_function("year_2015::day_04_v1", |b| {
    b.iter(|| day_04::day_04_v1(black_box(input_year_2015_day_04)))
  });
  g2015_04.bench_function("year_2015::day_04_v2", |b| {
    b.iter(|| day_04::day_04_v2(black_box(input_year_2015_day_04)))
  });
  g2015_04.finish();

  let mut g2015_05 = c.benchmark_group("year_2015::day_05");
  g2015_05.warm_up_time(warm_up_time);
  g2015_05.measurement_time(measurement_time);
  let input_year_2015_day_05 = include_str!("../inputs/year_2015_day_05_input");
  g2015_05.bench_function("year_2015::day_05_v1", |b| {
    b.iter(|| day_05::day_05_v1(black_box(input_year_2015_day_05)))
  });
  g2015_05.bench_function("year_2015::day_05_v2", |b| {
    b.iter(|| day_05::day_05_v2(black_box(input_year_2015_day_05)))
  });
  g2015_05.finish();

  let mut g2015_06 = c.benchmark_group("year_2015::day_06");
  g2015_06.warm_up_time(warm_up_time);
  g2015_06.measurement_time(measurement_time);
  let input_year_2015_day_06 = include_str!("../inputs/year_2015_day_06_input");
  g2015_06.bench_function("year_2015::day_06_v1", |b| {
    b.iter(|| day_06::day_06_v1(black_box(input_year_2015_day_06)))
  });
  g2015_06.bench_function("year_2015::day_06_v2", |b| {
    b.iter(|| day_06::day_06_v2(black_box(input_year_2015_day_06)))
  });
  g2015_06.finish();
}

criterion_group!(benches, year_2015_benchmark);
criterion_main!(benches);
