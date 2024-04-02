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
use advent_rs::year_2017::day_13;
use advent_rs::year_2017::day_14;
use advent_rs::year_2017::day_15;
use advent_rs::year_2017::day_16;
use advent_rs::year_2017::day_17;
use advent_rs::year_2017::day_18;
use advent_rs::year_2017::day_19;
use advent_rs::year_2017::day_20;
use advent_rs::year_2017::day_21;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn year_2017_day_01(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_01_input");
  assert_eq!(day_01::day_01_v1(input), 1_069);
  assert_eq!(day_01::day_01_v2(input), 1_268);

  let mut g2017_day_01 = c.benchmark_group("year_2017::day_01");
  g2017_day_01.bench_function("year_2017::day_01_v1", |b| {
    b.iter(|| day_01::day_01_v1(black_box(input)))
  });
  g2017_day_01.bench_function("year_2017::day_01_v2", |b| {
    b.iter(|| day_01::day_01_v2(black_box(input)))
  });
  g2017_day_01.finish();
}

fn year_2017_day_02(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_02_input");
  assert_eq!(day_02::day_02_v1(input), 53_978);
  assert_eq!(day_02::day_02_v2(input), 314);

  let mut g2017_day_02 = c.benchmark_group("year_2017::day_02");
  g2017_day_02.bench_function("year_2017::day_02_v1", |b| {
    b.iter(|| day_02::day_02_v1(black_box(input)))
  });
  g2017_day_02.bench_function("year_2017::day_02_v2", |b| {
    b.iter(|| day_02::day_02_v2(black_box(input)))
  });
  g2017_day_02.finish();
}

fn year_2017_day_03(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_03_input");
  assert_eq!(day_03::day_03_v1(input), 552);
  assert_eq!(day_03::day_03_v2(input), 330_785);

  let mut g2017_day_03 = c.benchmark_group("year_2017::day_03");
  g2017_day_03.bench_function("year_2017::day_03_v1", |b| {
    b.iter(|| day_03::day_03_v1(black_box(input)))
  });
  g2017_day_03.bench_function("year_2017::day_03_v2", |b| {
    b.iter(|| day_03::day_03_v2(black_box(input)))
  });
  g2017_day_03.finish();
}

fn year_2017_day_04(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_04_input");
  assert_eq!(day_04::day_04_v1(input), 466);
  assert_eq!(day_04::day_04_v2(input), 251);

  let mut g2017_day_04 = c.benchmark_group("year_2017::day_04");
  g2017_day_04.bench_function("year_2017::day_04_v1", |b| {
    b.iter(|| day_04::day_04_v1(black_box(input)))
  });
  g2017_day_04.bench_function("year_2017::day_04_v2", |b| {
    b.iter(|| day_04::day_04_v2(black_box(input)))
  });
  g2017_day_04.finish();
}

fn year_2017_day_05(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_05_input");
  assert_eq!(day_05::day_05_v1(input), 373_160);
  assert_eq!(day_05::day_05_v2(input), 26_395_586);

  let mut g2017_day_05 = c.benchmark_group("year_2017::day_05");
  g2017_day_05.bench_function("year_2017::day_05_v1", |b| {
    b.iter(|| day_05::day_05_v1(black_box(input)))
  });
  g2017_day_05.bench_function("year_2017::day_05_v2", |b| {
    b.iter(|| day_05::day_05_v2(black_box(input)))
  });
  g2017_day_05.finish();
}

fn year_2017_day_06(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_06_input");
  assert_eq!(day_06::day_06_v1(input), 11_137);
  assert_eq!(day_06::day_06_v2(input), 1_037);

  let mut g2017_day_06 = c.benchmark_group("year_2017::day_06");
  g2017_day_06.bench_function("year_2017::day_06_v1", |b| {
    b.iter(|| day_06::day_06_v1(black_box(input)))
  });
  g2017_day_06.bench_function("year_2017::day_06_v2", |b| {
    b.iter(|| day_06::day_06_v2(black_box(input)))
  });
  g2017_day_06.finish();
}

fn year_2017_day_07(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_07_input");
  assert_eq!(day_07::day_07_v1(input), "ykpsek");
  assert_eq!(day_07::day_07_v2(input), "1060");

  let mut g2017_day_07 = c.benchmark_group("year_2017::day_07");
  g2017_day_07.bench_function("year_2017::day_07_v1", |b| {
    b.iter(|| day_07::day_07_v1(black_box(input)))
  });
  g2017_day_07.bench_function("year_2017::day_07_v2", |b| {
    b.iter(|| day_07::day_07_v2(black_box(input)))
  });
  g2017_day_07.finish();
}

fn year_2017_day_08(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_08_input");
  assert_eq!(day_08::day_08_v1(input), 4_163);
  assert_eq!(day_08::day_08_v2(input), 5_347);

  let mut g2017_day_08 = c.benchmark_group("year_2017::day_08");
  g2017_day_08.bench_function("year_2017::day_08_v1", |b| {
    b.iter(|| day_08::day_08_v1(black_box(input)))
  });
  g2017_day_08.bench_function("year_2017::day_08_v2", |b| {
    b.iter(|| day_08::day_08_v2(black_box(input)))
  });
  g2017_day_08.finish();
}

fn year_2017_day_09(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_09_input");
  assert_eq!(day_09::day_09_v1(input), 17_537);
  assert_eq!(day_09::day_09_v2(input), 7_539);

  let mut g2017_day_09 = c.benchmark_group("year_2017::day_09");
  g2017_day_09.bench_function("year_2017::day_09_v1", |b| {
    b.iter(|| day_09::day_09_v1(black_box(input)))
  });
  g2017_day_09.bench_function("year_2017::day_09_v2", |b| {
    b.iter(|| day_09::day_09_v2(black_box(input)))
  });
  g2017_day_09.finish();
}

fn year_2017_day_10(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_10_input");
  assert_eq!(day_10::day_10_v1(input), "6909");
  assert_eq!(day_10::day_10_v2(input), "9d5f4561367d379cfbf04f8c471c0095");

  let mut g2017_day_10 = c.benchmark_group("year_2017::day_10");
  g2017_day_10.bench_function("year_2017::day_10_v1", |b| {
    b.iter(|| day_10::day_10_v1(black_box(input)))
  });
  g2017_day_10.bench_function("year_2017::day_10_v2", |b| {
    b.iter(|| day_10::day_10_v2(black_box(input)))
  });
  g2017_day_10.finish();
}

fn year_2017_day_11(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_11_input");
  assert_eq!(day_11::day_11_v1(input), 682);
  assert_eq!(day_11::day_11_v2(input), 1_406);

  let mut g2017_day_11 = c.benchmark_group("year_2017::day_11");
  g2017_day_11.bench_function("year_2017::day_11_v1", |b| {
    b.iter(|| day_11::day_11_v1(black_box(input)))
  });
  g2017_day_11.bench_function("year_2017::day_11_v2", |b| {
    b.iter(|| day_11::day_11_v2(black_box(input)))
  });
  g2017_day_11.finish();
}

fn year_2017_day_12(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_12_input");
  assert_eq!(day_12::day_12_v1(input), 130);
  assert_eq!(day_12::day_12_v2(input), 189);

  let mut g2017_day_12 = c.benchmark_group("year_2017::day_12");
  g2017_day_12.bench_function("year_2017::day_12_v1", |b| {
    b.iter(|| day_12::day_12_v1(black_box(input)))
  });
  g2017_day_12.bench_function("year_2017::day_12_v2", |b| {
    b.iter(|| day_12::day_12_v2(black_box(input)))
  });
  g2017_day_12.finish();
}

fn year_2017_day_13(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_13_input");
  assert_eq!(day_13::day_13_v1(input), 2_264);
  assert_eq!(day_13::day_13_v2(input), 3_875_838);

  let mut g2017_day_13 = c.benchmark_group("year_2017::day_13");
  g2017_day_13.bench_function("year_2017::day_13_v1", |b| {
    b.iter(|| day_13::day_13_v1(black_box(input)))
  });
  g2017_day_13.bench_function("year_2017::day_13_v2", |b| {
    b.iter(|| day_13::day_13_v2(black_box(input)))
  });
  g2017_day_13.finish();
}

fn year_2017_day_14(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_14_input");
  assert_eq!(day_14::day_14_v1(input), 8_230);
  assert_eq!(day_14::day_14_v2(input), 1_103);

  let mut g2017_day_14 = c.benchmark_group("year_2017::day_14");
  g2017_day_14.bench_function("year_2017::day_14_v1", |b| {
    b.iter(|| day_14::day_14_v1(black_box(input)))
  });
  g2017_day_14.bench_function("year_2017::day_14_v2", |b| {
    b.iter(|| day_14::day_14_v2(black_box(input)))
  });
  g2017_day_14.finish();
}

fn year_2017_day_15(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_15_input");
  assert_eq!(day_15::day_15_v1(input), 567);
  assert_eq!(day_15::day_15_v2(input), 323);

  let mut g2017_day_15 = c.benchmark_group("year_2017::day_15");
  g2017_day_15.bench_function("year_2017::day_15_v1", |b| {
    b.iter(|| day_15::day_15_v1(black_box(input)))
  });
  g2017_day_15.bench_function("year_2017::day_15_v2", |b| {
    b.iter(|| day_15::day_15_v2(black_box(input)))
  });
  g2017_day_15.finish();
}

fn year_2017_day_16(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_16_input");
  assert_eq!(day_16::day_16_v1(input), "lgpkniodmjacfbeh");
  assert_eq!(day_16::day_16_v2(input), "hklecbpnjigoafmd");

  let mut g2017_day_16 = c.benchmark_group("year_2017::day_16");
  g2017_day_16.bench_function("year_2017::day_16_v1", |b| {
    b.iter(|| day_16::day_16_v1(black_box(input)))
  });
  g2017_day_16.bench_function("year_2017::day_16_v2", |b| {
    b.iter(|| day_16::day_16_v2(black_box(input)))
  });
  g2017_day_16.finish();
}

fn year_2017_day_17(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_17_input");
  assert_eq!(day_17::day_17_v1(input), 1_173);
  assert_eq!(day_17::day_17_v2(input), 1_930_815);

  let mut g2017_day_17 = c.benchmark_group("year_2017::day_17");
  g2017_day_17.bench_function("year_2017::day_17_v1", |b| {
    b.iter(|| day_17::day_17_v1(black_box(input)))
  });
  g2017_day_17.bench_function("year_2017::day_17_v2", |b| {
    b.iter(|| day_17::day_17_v2(black_box(input)))
  });
  g2017_day_17.finish();
}

fn year_2017_day_18(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_18_input");
  assert_eq!(day_18::day_18_v1(input), 1_187);
  assert_eq!(day_18::day_18_v2(input), 5_969);

  let mut g2017_day_18 = c.benchmark_group("year_2017::day_18");
  g2017_day_18.bench_function("year_2017::day_18_v1", |b| {
    b.iter(|| day_18::day_18_v1(black_box(input)))
  });
  g2017_day_18.bench_function("year_2017::day_18_v2", |b| {
    b.iter(|| day_18::day_18_v2(black_box(input)))
  });
  g2017_day_18.finish();
}

fn year_2017_day_19(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_19_input");
  assert_eq!(day_19::day_19_v1(input), "RYLONKEWB");
  assert_eq!(day_19::day_19_v2(input), "16016");

  let mut g2017_day_19 = c.benchmark_group("year_2017::day_19");
  g2017_day_19.bench_function("year_2017::day_19_v1", |b| {
    b.iter(|| day_19::day_19_v1(black_box(input)))
  });
  g2017_day_19.bench_function("year_2017::day_19_v2", |b| {
    b.iter(|| day_19::day_19_v2(black_box(input)))
  });
  g2017_day_19.finish();
}

fn year_2017_day_20(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_20_input");
  assert_eq!(day_20::day_20_v1(input), 157);
  assert_eq!(day_20::day_20_v2(input), 499);

  let mut g2017_day_20 = c.benchmark_group("year_2017::day_20");
  g2017_day_20.bench_function("year_2017::day_20_v1", |b| {
    b.iter(|| day_20::day_20_v1(black_box(input)))
  });
  g2017_day_20.bench_function("year_2017::day_20_v2", |b| {
    b.iter(|| day_20::day_20_v2(black_box(input)))
  });
  g2017_day_20.finish();
}

fn year_2017_day_21(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2017/day_21_input");
  assert_eq!(day_21::day_21_v1(input), 179);
  assert_eq!(day_21::day_21_v2(input), 2_766_750);

  let mut g2017_day_21 = c.benchmark_group("year_2017::day_21");
  g2017_day_21.bench_function("year_2017::day_21_v1", |b| {
    b.iter(|| day_21::day_21_v1(black_box(input)))
  });
  g2017_day_21.bench_function("year_2017::day_21_v2", |b| {
    b.iter(|| day_21::day_21_v2(black_box(input)))
  });
  g2017_day_21.finish();
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
  year_2017_day_12,
  year_2017_day_13,
  year_2017_day_14,
  year_2017_day_15,
  year_2017_day_16,
  year_2017_day_17,
  year_2017_day_18,
  year_2017_day_19,
  year_2017_day_20,
  year_2017_day_21
);
criterion_main!(benches);
