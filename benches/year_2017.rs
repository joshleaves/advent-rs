use advent_rs::year_2017::day_01;
use advent_rs::year_2017::day_02;
use advent_rs::year_2017::day_03;
use advent_rs::year_2017::day_04;
use advent_rs::year_2017::day_05;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn year_2017_day_01(c: &mut Criterion) {
  let mut g2017_day_01 = c.benchmark_group("year_2017::day_01");
  let input_year_2017_day_01 = include_str!("../inputs/year_2017/day_01_input");
  g2017_day_01.bench_function("year_2017::day_01_v1", |b| {
    b.iter(|| assert_eq!(1_069, day_01::day_01_v1(black_box(input_year_2017_day_01))))
  });
  g2017_day_01.bench_function("year_2017::day_01_v2", |b| {
    b.iter(|| assert_eq!(1_268, day_01::day_01_v2(black_box(input_year_2017_day_01))))
  });
  g2017_day_01.finish();
}

fn year_2017_day_02(c: &mut Criterion) {
  let mut g2017_day_02 = c.benchmark_group("year_2017::day_02");
  let input_year_2017_day_02 = include_str!("../inputs/year_2017/day_02_input");
  g2017_day_02.bench_function("year_2017::day_02_v1", |b| {
    b.iter(|| assert_eq!(53_978, day_02::day_02_v1(black_box(input_year_2017_day_02))))
  });
  g2017_day_02.bench_function("year_2017::day_02_v2", |b| {
    b.iter(|| assert_eq!(314, day_02::day_02_v2(black_box(input_year_2017_day_02))))
  });
  g2017_day_02.finish();
}

fn year_2017_day_03(c: &mut Criterion) {
  let mut g2017_day_03 = c.benchmark_group("year_2017::day_03");
  let input_year_2017_day_03 = include_str!("../inputs/year_2017/day_03_input");
  g2017_day_03.bench_function("year_2017::day_03_v1", |b| {
    b.iter(|| assert_eq!(552, day_03::day_03_v1(black_box(input_year_2017_day_03))))
  });
  g2017_day_03.bench_function("year_2017::day_03_v2", |b| {
    b.iter(|| {
      assert_eq!(
        330_785,
        day_03::day_03_v2(black_box(input_year_2017_day_03))
      )
    })
  });
  g2017_day_03.finish();
}

fn year_2017_day_04(c: &mut Criterion) {
  let mut g2017_day_04 = c.benchmark_group("year_2017::day_04");
  let input_year_2017_day_04 = include_str!("../inputs/year_2017/day_04_input");
  g2017_day_04.bench_function("year_2017::day_04_v1", |b| {
    b.iter(|| assert_eq!(466, day_04::day_04_v1(black_box(input_year_2017_day_04))))
  });
  g2017_day_04.bench_function("year_2017::day_04_v2", |b| {
    b.iter(|| assert_eq!(251, day_04::day_04_v2(black_box(input_year_2017_day_04))))
  });
  g2017_day_04.finish();
}

fn year_2017_day_05(c: &mut Criterion) {
  let mut g2017_day_05 = c.benchmark_group("year_2017::day_05");
  let input_year_2017_day_05 = include_str!("../inputs/year_2017/day_05_input");
  g2017_day_05.bench_function("year_2017::day_05_v1", |b| {
    b.iter(|| {
      assert_eq!(
        373_160,
        day_05::day_05_v1(black_box(input_year_2017_day_05))
      )
    })
  });
  g2017_day_05.bench_function("year_2017::day_05_v2", |b| {
    b.iter(|| {
      assert_eq!(
        26_395_586,
        day_05::day_05_v2(black_box(input_year_2017_day_05))
      )
    })
  });
  g2017_day_05.finish();
}

criterion_group!(
  benches,
  year_2017_day_01,
  year_2017_day_02,
  year_2017_day_03,
  year_2017_day_04,
  year_2017_day_05
);
criterion_main!(benches);
