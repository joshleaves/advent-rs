use advent_rs::year_2016::day_01;
use advent_rs::year_2016::day_02;
use advent_rs::year_2016::day_03;
use advent_rs::year_2016::day_04;
use advent_rs::year_2016::day_05;
use advent_rs::year_2016::day_06;
use advent_rs::year_2016::day_07;
use advent_rs::year_2016::day_08;
use advent_rs::year_2016::day_09;
use advent_rs::year_2016::day_10;
use advent_rs::year_2016::day_11;
use advent_rs::year_2016::day_12;
use advent_rs::year_2016::day_13;
use advent_rs::year_2016::day_14;
use advent_rs::year_2016::day_15;
use advent_rs::year_2016::day_16;
use advent_rs::year_2016::day_17;
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

pub fn year_2016_day_04(c: &mut Criterion) {
  let mut g2016_day_04 = c.benchmark_group("year_2016::day_04");
  let input_year_2016_day_04 = include_str!("../inputs/year_2016/day_04_input");
  g2016_day_04.bench_function("year_2016::day_04_v1", |b| {
    b.iter(|| day_04::day_04_v1(black_box(input_year_2016_day_04)))
  });
  g2016_day_04.bench_function("year_2016::day_04_v2", |b| {
    b.iter(|| day_04::day_04_v2(black_box(input_year_2016_day_04)))
  });
  g2016_day_04.finish();
}

pub fn year_2016_day_05(c: &mut Criterion) {
  let mut g2016_day_05 = c.benchmark_group("year_2016::day_05");
  let input_year_2016_day_05 = include_str!("../inputs/year_2016/day_05_input");
  g2016_day_05.bench_function("year_2016::day_05_v1", |b| {
    b.iter(|| day_05::day_05_v1(black_box(input_year_2016_day_05)))
  });
  g2016_day_05.bench_function("year_2016::day_05_v2", |b| {
    b.iter(|| day_05::day_05_v2(black_box(input_year_2016_day_05)))
  });
  g2016_day_05.finish();
}

pub fn year_2016_day_06(c: &mut Criterion) {
  let mut g2016_day_06 = c.benchmark_group("year_2016::day_06");
  let input_year_2016_day_06 = include_str!("../inputs/year_2016/day_06_input");
  g2016_day_06.bench_function("year_2016::day_06_v1", |b| {
    b.iter(|| day_06::day_06_v1(black_box(input_year_2016_day_06)))
  });
  g2016_day_06.bench_function("year_2016::day_06_v2", |b| {
    b.iter(|| day_06::day_06_v2(black_box(input_year_2016_day_06)))
  });
  g2016_day_06.finish();
}

pub fn year_2016_day_07(c: &mut Criterion) {
  let mut g2016_day_07 = c.benchmark_group("year_2016::day_07");
  let input_year_2016_day_07 = include_str!("../inputs/year_2016/day_07_input");
  g2016_day_07.bench_function("year_2016::day_07_v1", |b| {
    b.iter(|| day_07::day_07_v1(black_box(input_year_2016_day_07)))
  });
  g2016_day_07.bench_function("year_2016::day_07_v2", |b| {
    b.iter(|| day_07::day_07_v2(black_box(input_year_2016_day_07)))
  });
  g2016_day_07.finish();
}

pub fn year_2016_day_08(c: &mut Criterion) {
  let mut g2016_day_08 = c.benchmark_group("year_2016::day_08");
  let input_year_2016_day_08 = include_str!("../inputs/year_2016/day_08_input");
  g2016_day_08.bench_function("year_2016::day_08_v1", |b| {
    b.iter(|| day_08::day_08_v1(black_box(input_year_2016_day_08)))
  });
  g2016_day_08.bench_function("year_2016::day_08_v2", |b| {
    b.iter(|| day_08::day_08_v2(black_box(input_year_2016_day_08)))
  });
  g2016_day_08.finish();
}

pub fn year_2016_day_09(c: &mut Criterion) {
  let mut g2016_day_09 = c.benchmark_group("year_2016::day_09");
  let input_year_2016_day_09 = include_str!("../inputs/year_2016/day_09_input");
  g2016_day_09.bench_function("year_2016::day_09_v1", |b| {
    b.iter(|| day_09::day_09_v1(black_box(input_year_2016_day_09)))
  });
  g2016_day_09.bench_function("year_2016::day_09_v2", |b| {
    b.iter(|| day_09::day_09_v2(black_box(input_year_2016_day_09)))
  });
  g2016_day_09.finish();
}

pub fn year_2016_day_10(c: &mut Criterion) {
  let mut g2016_day_10 = c.benchmark_group("year_2016::day_10");
  let input_year_2016_day_10 = include_str!("../inputs/year_2016/day_10_input");
  g2016_day_10.bench_function("year_2016::day_10_v1", |b| {
    b.iter(|| day_10::day_10_v1(black_box(input_year_2016_day_10)))
  });
  g2016_day_10.bench_function("year_2016::day_10_v2", |b| {
    b.iter(|| day_10::day_10_v2(black_box(input_year_2016_day_10)))
  });
  g2016_day_10.finish();
}

pub fn year_2016_day_11(c: &mut Criterion) {
  let mut g2016_day_11 = c.benchmark_group("year_2016::day_11");
  let input_year_2016_day_11 = include_str!("../inputs/year_2016/day_11_input");
  g2016_day_11.bench_function("year_2016::day_11_v1", |b| {
    b.iter(|| day_11::day_11_v1(black_box(input_year_2016_day_11)))
  });
  g2016_day_11.bench_function("year_2016::day_11_v2", |b| {
    b.iter(|| day_11::day_11_v2(black_box(input_year_2016_day_11)))
  });
  g2016_day_11.finish();
}

pub fn year_2016_day_12(c: &mut Criterion) {
  let mut g2016_day_12 = c.benchmark_group("year_2016::day_12");
  let input_year_2016_day_12 = include_str!("../inputs/year_2016/day_12_input");
  g2016_day_12.bench_function("year_2016::day_12_v1", |b| {
    b.iter(|| day_12::day_12_v1(black_box(input_year_2016_day_12)))
  });
  g2016_day_12.bench_function("year_2016::day_12_v2", |b| {
    b.iter(|| day_12::day_12_v2(black_box(input_year_2016_day_12)))
  });
  g2016_day_12.finish();
}

pub fn year_2016_day_13(c: &mut Criterion) {
  let mut g2016_day_13 = c.benchmark_group("year_2016::day_13");
  let input_year_2016_day_13 = include_str!("../inputs/year_2016/day_13_input");
  g2016_day_13.bench_function("year_2016::day_13_v1", |b| {
    b.iter(|| day_13::day_13_v1(black_box(input_year_2016_day_13)))
  });
  g2016_day_13.bench_function("year_2016::day_13_v2", |b| {
    b.iter(|| day_13::day_13_v2(black_box(input_year_2016_day_13)))
  });
  g2016_day_13.finish();
}

pub fn year_2016_day_14(c: &mut Criterion) {
  let mut g2016_day_14 = c.benchmark_group("year_2016::day_14");
  let input_year_2016_day_14 = include_str!("../inputs/year_2016/day_14_input");
  g2016_day_14.bench_function("year_2016::day_14_v1", |b| {
    b.iter(|| day_14::day_14_v1(black_box(input_year_2016_day_14)))
  });
  g2016_day_14.bench_function("year_2016::day_14_v2", |b| {
    b.iter(|| day_14::day_14_v2(black_box(input_year_2016_day_14)))
  });
  g2016_day_14.finish();
}

pub fn year_2016_day_15(c: &mut Criterion) {
  let mut g2016_day_15 = c.benchmark_group("year_2016::day_15");
  let input_year_2016_day_15 = include_str!("../inputs/year_2016/day_15_input");
  g2016_day_15.bench_function("year_2016::day_15_v1", |b| {
    b.iter(|| day_15::day_15_v1(black_box(input_year_2016_day_15)))
  });
  g2016_day_15.bench_function("year_2016::day_15_v2", |b| {
    b.iter(|| day_15::day_15_v2(black_box(input_year_2016_day_15)))
  });
  g2016_day_15.finish();
}

pub fn year_2016_day_16(c: &mut Criterion) {
  let mut g2016_day_16 = c.benchmark_group("year_2016::day_16");
  let input_year_2016_day_16 = include_str!("../inputs/year_2016/day_16_input");
  g2016_day_16.bench_function("year_2016::day_16_v1", |b| {
    b.iter(|| day_16::day_16_v1(black_box(input_year_2016_day_16)))
  });
  g2016_day_16.bench_function("year_2016::day_16_v2", |b| {
    b.iter(|| day_16::day_16_v2(black_box(input_year_2016_day_16)))
  });
  g2016_day_16.finish();
}

pub fn year_2016_day_17(c: &mut Criterion) {
  let mut g2016_day_17 = c.benchmark_group("year_2016::day_17");
  let input_year_2016_day_17 = include_str!("../inputs/year_2016/day_17_input");
  g2016_day_17.bench_function("year_2016::day_17_v1", |b| {
    b.iter(|| day_17::day_17_v1(black_box(input_year_2016_day_17)))
  });
  g2016_day_17.bench_function("year_2016::day_17_v2", |b| {
    b.iter(|| day_17::day_17_v2(black_box(input_year_2016_day_17)))
  });
  g2016_day_17.finish();
}

criterion_group!(
  benches,
  year_2016_day_01,
  year_2016_day_02,
  year_2016_day_03,
  year_2016_day_04,
  year_2016_day_05,
  year_2016_day_06,
  year_2016_day_07,
  year_2016_day_08,
  year_2016_day_09,
  year_2016_day_10,
  year_2016_day_11,
  year_2016_day_12,
  year_2016_day_13,
  year_2016_day_14,
  year_2016_day_15,
  year_2016_day_16,
  year_2016_day_17
);
criterion_main!(benches);
