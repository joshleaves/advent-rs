use advent_rs::year_2018::day_01;
use advent_rs::year_2018::day_02;
use advent_rs::year_2018::day_03;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn year_2018_day_01(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2018/day_01_input");
  assert_eq!(day_01::day_01_v1(input), 484);
  assert_eq!(day_01::day_01_v2(input), 367);

  let mut g2018_day_01 = c.benchmark_group("year_2018::day_01");
  g2018_day_01.bench_function("year_2018::day_01_v1", |b| {
    b.iter(|| day_01::day_01_v1(black_box(input)))
  });
  g2018_day_01.bench_function("year_2018::day_01_v2", |b| {
    b.iter(|| day_01::day_01_v2(black_box(input)))
  });
  g2018_day_01.finish();
}

fn year_2018_day_02(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2018/day_02_input");
  assert_eq!(day_02::day_02_v1(input), "7872");
  assert_eq!(day_02::day_02_v2(input), "tjxmoewpdkyaihvrndfluwbzc");

  let mut g2018_day_02 = c.benchmark_group("year_2018::day_02");
  g2018_day_02.bench_function("year_2018::day_02_v1", |b| {
    b.iter(|| day_02::day_02_v1(black_box(input)))
  });
  g2018_day_02.bench_function("year_2018::day_02_v2", |b| {
    b.iter(|| day_02::day_02_v2(black_box(input)))
  });
  g2018_day_02.finish();
}

fn year_2018_day_03(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2018/day_03_input");
  assert_eq!(day_03::day_03_v1(input), 101_565);
  assert_eq!(day_03::day_03_v2(input), 656);

  let mut g2018_day_03 = c.benchmark_group("year_2018::day_03");
  g2018_day_03.bench_function("year_2018::day_03_v1", |b| {
    b.iter(|| day_03::day_03_v1(black_box(input)))
  });
  g2018_day_03.bench_function("year_2018::day_03_v2", |b| {
    b.iter(|| day_03::day_03_v2(black_box(input)))
  });
  g2018_day_03.finish();
}

criterion_group!(
  benches,
  year_2018_day_01,
  year_2018_day_02,
  year_2018_day_03
);
criterion_main!(benches);
