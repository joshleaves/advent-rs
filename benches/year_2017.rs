use advent_rs::year_2017::day_01;
use advent_rs::year_2017::day_02;
use advent_rs::year_2017::day_03;
use advent_rs::year_2017::day_04;
use advent_rs::year_2017::day_05;
use advent_rs::year_2017::day_06;
use advent_rs::year_2017::day_07;
use advent_rs::year_2017::day_08;
use advent_rs::year_2017::day_09;
use advent_rs::year_2017::day_10;
use advent_rs::year_2017::day_11;
use advent_rs::year_2017::day_12;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn year_2017_day_01(c: &mut Criterion) {
  let input_day_01 = include_str!("../inputs/year_2017/day_01_input");
  assert_eq!(day_01::day_01_v1(black_box(input_day_01)), 1_069);
  assert_eq!(day_01::day_01_v2(black_box(input_day_01)), 1_268);

  let mut g2017_day_01 = c.benchmark_group("year_2017::day_01");
  g2017_day_01.bench_function("year_2017::day_01_v1", |b| {
    b.iter(|| day_01::day_01_v1(black_box(input_day_01)))
  });
  g2017_day_01.bench_function("year_2017::day_01_v2", |b| {
    b.iter(|| day_01::day_01_v2(black_box(input_day_01)))
  });
  g2017_day_01.finish();
}

fn year_2017_day_02(c: &mut Criterion) {
  let input_day_02 = include_str!("../inputs/year_2017/day_02_input");
  assert_eq!(day_02::day_02_v1(black_box(input_day_02)), 53_978);
  assert_eq!(day_02::day_02_v2(black_box(input_day_02)), 314);

  let mut g2017_day_02 = c.benchmark_group("year_2017::day_02");
  g2017_day_02.bench_function("year_2017::day_02_v1", |b| {
    b.iter(|| day_02::day_02_v1(black_box(input_day_02)))
  });
  g2017_day_02.bench_function("year_2017::day_02_v2", |b| {
    b.iter(|| day_02::day_02_v2(black_box(input_day_02)))
  });
  g2017_day_02.finish();
}

fn year_2017_day_03(c: &mut Criterion) {
  let input_day_03 = include_str!("../inputs/year_2017/day_03_input");
  assert_eq!(day_03::day_03_v1(black_box(input_day_03)), 552);
  assert_eq!(day_03::day_03_v2(black_box(input_day_03)), 330_785);

  let mut g2017_day_03 = c.benchmark_group("year_2017::day_03");
  g2017_day_03.bench_function("year_2017::day_03_v1", |b| {
    b.iter(|| day_03::day_03_v1(black_box(input_day_03)))
  });
  g2017_day_03.bench_function("year_2017::day_03_v2", |b| {
    b.iter(|| day_03::day_03_v2(black_box(input_day_03)))
  });
  g2017_day_03.finish();
}

fn year_2017_day_04(c: &mut Criterion) {
  let input_day_04 = include_str!("../inputs/year_2017/day_04_input");
  assert_eq!(day_04::day_04_v1(black_box(input_day_04)), 466);
  assert_eq!(day_04::day_04_v2(black_box(input_day_04)), 251);

  let mut g2017_day_04 = c.benchmark_group("year_2017::day_04");
  g2017_day_04.bench_function("year_2017::day_04_v1", |b| {
    b.iter(|| day_04::day_04_v1(black_box(input_day_04)))
  });
  g2017_day_04.bench_function("year_2017::day_04_v2", |b| {
    b.iter(|| day_04::day_04_v2(black_box(input_day_04)))
  });
  g2017_day_04.finish();
}

fn year_2017_day_05(c: &mut Criterion) {
  let input_day_05 = include_str!("../inputs/year_2017/day_05_input");
  assert_eq!(day_05::day_05_v1(black_box(input_day_05)), 373_160);
  assert_eq!(day_05::day_05_v2(black_box(input_day_05)), 26_395_586);

  let mut g2017_day_05 = c.benchmark_group("year_2017::day_05");
  g2017_day_05.bench_function("year_2017::day_05_v1", |b| {
    b.iter(|| day_05::day_05_v1(black_box(input_day_05)))
  });
  g2017_day_05.bench_function("year_2017::day_05_v2", |b| {
    b.iter(|| day_05::day_05_v2(black_box(input_day_05)))
  });
  g2017_day_05.finish();
}

fn year_2017_day_06(c: &mut Criterion) {
  let input_day_06 = include_str!("../inputs/year_2017/day_06_input");
  assert_eq!(day_06::day_06_v1(black_box(input_day_06)), 11_137);
  assert_eq!(day_06::day_06_v2(black_box(input_day_06)), 1_037);

  let mut g2017_day_06 = c.benchmark_group("year_2017::day_06");
  g2017_day_06.bench_function("year_2017::day_06_v1", |b| {
    b.iter(|| day_06::day_06_v1(black_box(input_day_06)))
  });
  g2017_day_06.bench_function("year_2017::day_06_v2", |b| {
    b.iter(|| day_06::day_06_v2(black_box(input_day_06)))
  });
  g2017_day_06.finish();
}

fn year_2017_day_07(c: &mut Criterion) {
  let input_day_07 = include_str!("../inputs/year_2017/day_07_input");
  assert_eq!(day_07::day_07_v1(black_box(input_day_07)), "ykpsek");
  assert_eq!(day_07::day_07_v2(black_box(input_day_07)), "1060");

  let mut g2017_day_07 = c.benchmark_group("year_2017::day_07");
  g2017_day_07.bench_function("year_2017::day_07_v1", |b| {
    b.iter(|| day_07::day_07_v1(black_box(input_day_07)))
  });
  g2017_day_07.bench_function("year_2017::day_07_v2", |b| {
    b.iter(|| day_07::day_07_v2(black_box(input_day_07)))
  });
  g2017_day_07.finish();
}

fn year_2017_day_08(c: &mut Criterion) {
  let input_day_08 = include_str!("../inputs/year_2017/day_08_input");
  assert_eq!(day_08::day_08_v1(black_box(input_day_08)), 4_163);
  assert_eq!(day_08::day_08_v2(black_box(input_day_08)), 5_347);

  let mut g2017_day_08 = c.benchmark_group("year_2017::day_08");
  g2017_day_08.bench_function("year_2017::day_08_v1", |b| {
    b.iter(|| day_08::day_08_v1(black_box(input_day_08)))
  });
  g2017_day_08.bench_function("year_2017::day_08_v2", |b| {
    b.iter(|| day_08::day_08_v2(black_box(input_day_08)))
  });
  g2017_day_08.finish();
}

fn year_2017_day_09(c: &mut Criterion) {
  let input_day_09 = include_str!("../inputs/year_2017/day_09_input");
  assert_eq!(day_09::day_09_v1(black_box(input_day_09)), 17_537);
  assert_eq!(day_09::day_09_v2(black_box(input_day_09)), 7_539);

  let mut g2017_day_09 = c.benchmark_group("year_2017::day_09");
  g2017_day_09.bench_function("year_2017::day_09_v1", |b| {
    b.iter(|| day_09::day_09_v1(black_box(input_day_09)))
  });
  g2017_day_09.bench_function("year_2017::day_09_v2", |b| {
    b.iter(|| day_09::day_09_v2(black_box(input_day_09)))
  });
  g2017_day_09.finish();
}

fn year_2017_day_10(c: &mut Criterion) {
  let input_day_10 = include_str!("../inputs/year_2017/day_10_input");
  assert_eq!(day_10::day_10_v1(input_day_10), "6909");
  assert_eq!(
    day_10::day_10_v2(input_day_10),
    "9d5f4561367d379cfbf04f8c471c0095"
  );

  let mut g2017_day_10 = c.benchmark_group("year_2017::day_10");
  g2017_day_10.bench_function("year_2017::day_10_v1", |b| {
    b.iter(|| day_10::day_10_v1(black_box(input_day_10)))
  });
  g2017_day_10.bench_function("year_2017::day_10_v2", |b| {
    b.iter(|| day_10::day_10_v2(black_box(input_day_10)))
  });
  g2017_day_10.finish();
}

fn year_2017_day_11(c: &mut Criterion) {
  let input_day_11 = include_str!("../inputs/year_2017/day_11_input");
  assert_eq!(day_11::day_11_v1(black_box(input_day_11)), 682);
  assert_eq!(day_11::day_11_v2(black_box(input_day_11)), 1_406);

  let mut g2017_day_11 = c.benchmark_group("year_2017::day_11");
  g2017_day_11.bench_function("year_2017::day_11_v1", |b| {
    b.iter(|| day_11::day_11_v1(black_box(input_day_11)))
  });
  g2017_day_11.bench_function("year_2017::day_11_v2", |b| {
    b.iter(|| day_11::day_11_v2(black_box(input_day_11)))
  });
  g2017_day_11.finish();
}

fn year_2017_day_12(c: &mut Criterion) {
  let input_day_12 = include_str!("../inputs/year_2017/day_12_input");
  assert_eq!(day_12::day_12_v1(black_box(input_day_12)), 130);
  assert_eq!(day_12::day_12_v2(black_box(input_day_12)), 189);

  let mut g2017_day_12 = c.benchmark_group("year_2017::day_12");
  g2017_day_12.bench_function("year_2017::day_12_v1", |b| {
    b.iter(|| day_12::day_12_v1(black_box(input_day_12)))
  });
  g2017_day_12.bench_function("year_2017::day_12_v2", |b| {
    b.iter(|| day_12::day_12_v2(black_box(input_day_12)))
  });
  g2017_day_12.finish();
}

criterion_group!(
  benches,
  year_2017_day_01,
  year_2017_day_02,
  year_2017_day_03,
  year_2017_day_04,
  year_2017_day_05,
  year_2017_day_06,
  year_2017_day_07,
  year_2017_day_08,
  year_2017_day_09,
  year_2017_day_10,
  year_2017_day_11,
  year_2017_day_12
);
criterion_main!(benches);
