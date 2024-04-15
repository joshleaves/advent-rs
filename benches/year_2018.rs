use advent_rs::year_2018::day_01;
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

criterion_group!(benches, year_2018_day_01);
criterion_main!(benches);
