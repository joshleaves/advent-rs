use advent_rs::year_2018::day_01;
use advent_rs::year_2018::day_02;
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

criterion_group!(benches, year_2018_day_01, year_2018_day_02);
criterion_main!(benches);
