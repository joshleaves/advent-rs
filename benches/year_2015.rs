use advent_rs::year_2015::day_01;
use advent_rs::year_2015::day_02;
use advent_rs::year_2015::day_03;
use advent_rs::year_2015::day_04;
use advent_rs::year_2015::day_05;
use advent_rs::year_2015::day_06;
use advent_rs::year_2015::day_07;
use advent_rs::year_2015::day_08;
use advent_rs::year_2015::day_09;
use advent_rs::year_2015::day_10;
use advent_rs::year_2015::day_11;
use advent_rs::year_2015::day_12;
use advent_rs::year_2015::day_13;
use advent_rs::year_2015::day_14;
use advent_rs::year_2015::day_15;
use advent_rs::year_2015::day_16;
use advent_rs::year_2015::day_17;
use advent_rs::year_2015::day_18;
use advent_rs::year_2015::day_19;
use advent_rs::year_2015::day_20;
use advent_rs::year_2015::day_21;
use advent_rs::year_2015::day_22;
use advent_rs::year_2015::day_23;
use advent_rs::year_2015::day_24;
use advent_rs::year_2015::day_25;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn year_2015_day_01(c: &mut Criterion) {
  let input_day_01 = include_str!("../inputs/year_2015/day_01_input");
  assert_eq!(day_01::day_01_v1(input_day_01), 138);
  assert_eq!(day_01::day_01_v2(input_day_01), 1771);

  let mut g2015_day_01 = c.benchmark_group("year_2015::day_01");
  g2015_day_01.bench_function("year_2015::day_01_v1", |b| {
    b.iter(|| day_01::day_01_v1(black_box(input_day_01)))
  });
  g2015_day_01.bench_function("year_2015::day_01_v2", |b| {
    b.iter(|| day_01::day_01_v2(black_box(input_day_01)))
  });
  g2015_day_01.finish();
}

fn year_2015_day_02(c: &mut Criterion) {
  let input_day_02 = include_str!("../inputs/year_2015/day_02_input");
  assert_eq!(day_02::day_02_v1(input_day_02), 1_588_178);
  assert_eq!(day_02::day_02_v2(input_day_02), 3_783_758);

  let mut g2015_day_02 = c.benchmark_group("year_2015::day_02");
  g2015_day_02.bench_function("year_2015::day_02_v1", |b| {
    b.iter(|| day_02::day_02_v1(black_box(input_day_02)))
  });
  g2015_day_02.bench_function("year_2015::day_02_v2", |b| {
    b.iter(|| day_02::day_02_v2(black_box(input_day_02)))
  });
  g2015_day_02.finish();
}

fn year_2015_day_03(c: &mut Criterion) {
  let input_day_03 = include_str!("../inputs/year_2015/day_03_input");
  assert_eq!(day_03::day_03_v1(input_day_03), 2081);
  assert_eq!(day_03::day_03_v2(input_day_03), 2341);

  let mut g2015_day_03 = c.benchmark_group("year_2015::day_03");
  g2015_day_03.bench_function("year_2015::day_03_v1", |b| {
    b.iter(|| day_03::day_03_v1(black_box(input_day_03)))
  });
  g2015_day_03.bench_function("year_2015::day_03_v2", |b| {
    b.iter(|| day_03::day_03_v2(black_box(input_day_03)))
  });
  g2015_day_03.finish();
}

fn year_2015_day_04(c: &mut Criterion) {
  let input_day_04 = include_str!("../inputs/year_2015/day_04_input");
  assert_eq!(day_04::day_04_v1(input_day_04), 346_386);
  assert_eq!(day_04::day_04_v2(input_day_04), 9_958_218);

  let mut g2015_day_04 = c.benchmark_group("year_2015::day_04");
  g2015_day_04.bench_function("year_2015::day_04_v1", |b| {
    b.iter(|| day_04::day_04_v1(black_box(input_day_04)))
  });
  g2015_day_04.bench_function("year_2015::day_04_v2", |b| {
    b.iter(|| day_04::day_04_v2(black_box(input_day_04)))
  });
  g2015_day_04.finish();
}

fn year_2015_day_05(c: &mut Criterion) {
  let input_day_05 = include_str!("../inputs/year_2015/day_05_input");
  assert_eq!(day_05::day_05_v1(input_day_05), 238);
  assert_eq!(day_05::day_05_v2(input_day_05), 69);

  let mut g2015_day_05 = c.benchmark_group("year_2015::day_05");
  g2015_day_05.bench_function("year_2015::day_05_v1", |b| {
    b.iter(|| day_05::day_05_v1(black_box(input_day_05)))
  });
  g2015_day_05.bench_function("year_2015::day_05_v2", |b| {
    b.iter(|| day_05::day_05_v2(black_box(input_day_05)))
  });
  g2015_day_05.finish();
}

fn year_2015_day_06(c: &mut Criterion) {
  let input_day_06 = include_str!("../inputs/year_2015/day_06_input");
  assert_eq!(day_06::day_06_v1(input_day_06), 400_410);
  assert_eq!(day_06::day_06_v2(input_day_06), 15_343_601);

  let mut g2015_day_06 = c.benchmark_group("year_2015::day_06");
  g2015_day_06.bench_function("year_2015::day_06_v1", |b| {
    b.iter(|| day_06::day_06_v1(black_box(input_day_06)))
  });
  g2015_day_06.bench_function("year_2015::day_06_v2", |b| {
    b.iter(|| day_06::day_06_v2(black_box(input_day_06)))
  });
  g2015_day_06.finish();
}

fn year_2015_day_07(c: &mut Criterion) {
  let input_day_07 = include_str!("../inputs/year_2015/day_07_input");
  assert_eq!(day_07::day_07_v1(input_day_07), 46_065);
  assert_eq!(day_07::day_07_v2(input_day_07), 14_134);

  let mut g2015_day_07 = c.benchmark_group("year_2015::day_07");
  g2015_day_07.bench_function("year_2015::day_07_v1", |b| {
    b.iter(|| day_07::day_07_v1(black_box(input_day_07)))
  });
  g2015_day_07.bench_function("year_2015::day_07_v2", |b| {
    b.iter(|| day_07::day_07_v2(black_box(input_day_07)))
  });
  g2015_day_07.finish();
}

fn year_2015_day_08(c: &mut Criterion) {
  let input_day_08 = include_str!("../inputs/year_2015/day_08_input");
  assert_eq!(day_08::day_08_v1(input_day_08), 1_333);
  assert_eq!(day_08::day_08_v2(input_day_08), 2_046);

  let mut g2015_day_08 = c.benchmark_group("year_2015::day_08");
  g2015_day_08.bench_function("year_2015::day_08_v1", |b| {
    b.iter(|| day_08::day_08_v1(black_box(input_day_08)))
  });
  g2015_day_08.bench_function("year_2015::day_08_v2", |b| {
    b.iter(|| day_08::day_08_v2(black_box(input_day_08)))
  });
  g2015_day_08.finish();
}

fn year_2015_day_09(c: &mut Criterion) {
  let input_day_09 = include_str!("../inputs/year_2015/day_09_input");
  assert_eq!(day_09::day_09_v1(input_day_09), 117);
  assert_eq!(day_09::day_09_v2(input_day_09), 909);

  let mut g2015_day_09 = c.benchmark_group("year_2015::day_09");
  g2015_day_09.bench_function("year_2015::day_09_v1", |b| {
    b.iter(|| day_09::day_09_v1(black_box(input_day_09)))
  });
  g2015_day_09.bench_function("year_2015::day_09_v2", |b| {
    b.iter(|| day_09::day_09_v2(black_box(input_day_09)))
  });
  g2015_day_09.finish();
}

fn year_2015_day_10(c: &mut Criterion) {
  let input_day_10 = include_str!("../inputs/year_2015/day_10_input");
  assert_eq!(day_10::day_10_v1(input_day_10), 252_594);
  assert_eq!(day_10::day_10_v2(input_day_10), 3_579_328);

  let mut g2015_day_10 = c.benchmark_group("year_2015::day_10");
  g2015_day_10.bench_function("year_2015::day_10_v1", |b| {
    b.iter(|| day_10::day_10_v1(black_box(input_day_10)))
  });
  g2015_day_10.bench_function("year_2015::day_10_v2", |b| {
    b.iter(|| day_10::day_10_v2(black_box(input_day_10)))
  });
  g2015_day_10.finish();
}

fn year_2015_day_11(c: &mut Criterion) {
  let input_day_11 = include_str!("../inputs/year_2015/day_11_input");
  assert_eq!(day_11::day_11_v1(input_day_11), "vzbxxyzz");
  assert_eq!(day_11::day_11_v2(input_day_11), "vzcaabcc");

  let mut g2015_day_11 = c.benchmark_group("year_2015::day_11");
  g2015_day_11.bench_function("year_2015::day_11_v1", |b| {
    b.iter(|| day_11::day_11_v1(black_box(input_day_11)))
  });
  g2015_day_11.bench_function("year_2015::day_11_v2", |b| {
    b.iter(|| day_11::day_11_v2(black_box(input_day_11)))
  });
  g2015_day_11.finish();
}

fn year_2015_day_12(c: &mut Criterion) {
  let input_day_12 = include_str!("../inputs/year_2015/day_12_input");
  assert_eq!(day_12::day_12_v1(input_day_12), 111_754);
  assert_eq!(day_12::day_12_v2(input_day_12), 65_402);

  let mut g2015_day_12 = c.benchmark_group("year_2015::day_12");
  g2015_day_12.bench_function("year_2015::day_12_v1", |b| {
    b.iter(|| day_12::day_12_v1(black_box(input_day_12)))
  });
  g2015_day_12.bench_function("year_2015::day_12_v2", |b| {
    b.iter(|| day_12::day_12_v2(black_box(input_day_12)))
  });
  g2015_day_12.finish();
}

fn year_2015_day_13(c: &mut Criterion) {
  let input_day_13 = include_str!("../inputs/year_2015/day_13_input");
  assert_eq!(day_13::day_13_v1(input_day_13), 709);
  assert_eq!(day_13::day_13_v2(input_day_13), 668);

  let mut g2015_day_13 = c.benchmark_group("year_2015::day_13");
  g2015_day_13.bench_function("year_2015::day_13_v1", |b| {
    b.iter(|| day_13::day_13_v1(black_box(input_day_13)))
  });
  g2015_day_13.bench_function("year_2015::day_13_v2", |b| {
    b.iter(|| day_13::day_13_v2(black_box(input_day_13)))
  });
  g2015_day_13.finish();
}

fn year_2015_day_14(c: &mut Criterion) {
  let input_day_14 = include_str!("../inputs/year_2015/day_14_input");
  assert_eq!(day_14::day_14_v1(input_day_14), 2655);
  assert_eq!(day_14::day_14_v2(input_day_14), 1059);

  let mut g2015_day_14 = c.benchmark_group("year_2015::day_14");
  g2015_day_14.bench_function("year_2015::day_14_v1", |b| {
    b.iter(|| day_14::day_14_v1(black_box(input_day_14)))
  });
  g2015_day_14.bench_function("year_2015::day_14_v2", |b| {
    b.iter(|| day_14::day_14_v2(black_box(input_day_14)))
  });
  g2015_day_14.finish();
}

fn year_2015_day_15(c: &mut Criterion) {
  let input_day_15 = include_str!("../inputs/year_2015/day_15_input");
  assert_eq!(day_15::day_15_v1(input_day_15), 222_870);
  assert_eq!(day_15::day_15_v2(input_day_15), 117_936);

  let mut g2015_day_15 = c.benchmark_group("year_2015::day_15");
  g2015_day_15.bench_function("year_2015::day_15_v1", |b| {
    b.iter(|| day_15::day_15_v1(black_box(input_day_15)))
  });
  g2015_day_15.bench_function("year_2015::day_15_v2", |b| {
    b.iter(|| day_15::day_15_v2(black_box(input_day_15)))
  });
  g2015_day_15.finish();
}

fn year_2015_day_16(c: &mut Criterion) {
  let input_day_16 = include_str!("../inputs/year_2015/day_16_input");
  assert_eq!(day_16::day_16_v1(input_day_16), 373);
  assert_eq!(day_16::day_16_v2(input_day_16), 260);

  let mut g2015_day_16 = c.benchmark_group("year_2015::day_16");
  g2015_day_16.bench_function("year_2015::day_16_v1", |b| {
    b.iter(|| day_16::day_16_v1(black_box(input_day_16)))
  });
  g2015_day_16.bench_function("year_2015::day_16_v2", |b| {
    b.iter(|| day_16::day_16_v2(black_box(input_day_16)))
  });
  g2015_day_16.finish();
}

fn year_2015_day_17(c: &mut Criterion) {
  let input_day_17 = include_str!("../inputs/year_2015/day_17_input");
  assert_eq!(day_17::day_17_v1(input_day_17), 1_638);
  assert_eq!(day_17::day_17_v2(input_day_17), 17);

  let mut g2015_day_17 = c.benchmark_group("year_2015::day_17");
  g2015_day_17.bench_function("year_2015::day_17_v1", |b| {
    b.iter(|| day_17::day_17_v1(black_box(input_day_17)))
  });
  g2015_day_17.bench_function("year_2015::day_17_v2", |b| {
    b.iter(|| day_17::day_17_v2(black_box(input_day_17)))
  });
  g2015_day_17.finish();
}

fn year_2015_day_18(c: &mut Criterion) {
  let input_day_18 = include_str!("../inputs/year_2015/day_18_input");
  assert_eq!(day_18::day_18_v1(input_day_18), 821);
  assert_eq!(day_18::day_18_v2(input_day_18), 886);

  let mut g2015_day_18 = c.benchmark_group("year_2015::day_18");
  g2015_day_18.bench_function("year_2015::day_18_v1", |b| {
    b.iter(|| day_18::day_18_v1(black_box(input_day_18)))
  });
  g2015_day_18.bench_function("year_2015::day_18_v2", |b| {
    b.iter(|| day_18::day_18_v2(black_box(input_day_18)))
  });
  g2015_day_18.finish();
}

fn year_2015_day_19(c: &mut Criterion) {
  let input_day_19 = include_str!("../inputs/year_2015/day_19_input");
  assert_eq!(day_19::day_19_v1(input_day_19), 576);
  assert_eq!(day_19::day_19_v2(input_day_19), 207);

  let mut g2015_day_19 = c.benchmark_group("year_2015::day_19");
  g2015_day_19.bench_function("year_2015::day_19_v1", |b| {
    b.iter(|| day_19::day_19_v1(black_box(input_day_19)))
  });
  g2015_day_19.bench_function("year_2015::day_19_v2", |b| {
    b.iter(|| day_19::day_19_v2(black_box(input_day_19)))
  });
  g2015_day_19.finish();
}

fn year_2015_day_20(c: &mut Criterion) {
  let input_day_20 = include_str!("../inputs/year_2015/day_20_input");
  assert_eq!(day_20::day_20_v1(input_day_20), 831_600);
  assert_eq!(day_20::day_20_v2(input_day_20), 884_520);

  let mut g2015_day_20 = c.benchmark_group("year_2015::day_20");
  g2015_day_20.bench_function("year_2015::day_20_v1", |b| {
    b.iter(|| day_20::day_20_v1(black_box(input_day_20)))
  });
  g2015_day_20.bench_function("year_2015::day_20_v2", |b| {
    b.iter(|| day_20::day_20_v2(black_box(input_day_20)))
  });
  g2015_day_20.finish();
}

fn year_2015_day_21(c: &mut Criterion) {
  let input_day_21 = include_str!("../inputs/year_2015/day_21_input");
  assert_eq!(day_21::day_21_v1(input_day_21), 91);
  assert_eq!(day_21::day_21_v2(input_day_21), 158);

  let mut g2015_day_21 = c.benchmark_group("year_2015::day_21");
  g2015_day_21.bench_function("year_2015::day_21_v1", |b| {
    b.iter(|| day_21::day_21_v1(black_box(input_day_21)))
  });
  g2015_day_21.bench_function("year_2015::day_21_v2", |b| {
    b.iter(|| day_21::day_21_v2(black_box(input_day_21)))
  });
  g2015_day_21.finish();
}

fn year_2015_day_22(c: &mut Criterion) {
  let input_day_22 = include_str!("../inputs/year_2015/day_22_input");
  assert_eq!(day_22::day_22_v1(input_day_22), 953);
  assert_eq!(day_22::day_22_v2(input_day_22), 1289);

  let mut g2015_day_22 = c.benchmark_group("year_2015::day_22");
  g2015_day_22.bench_function("year_2015::day_22_v1", |b| {
    b.iter(|| day_22::day_22_v1(black_box(input_day_22)))
  });
  g2015_day_22.bench_function("year_2015::day_22_v2", |b| {
    b.iter(|| day_22::day_22_v2(black_box(input_day_22)))
  });
  g2015_day_22.finish();
}

fn year_2015_day_23(c: &mut Criterion) {
  let input_day_23 = include_str!("../inputs/year_2015/day_23_input");
  assert_eq!(day_23::day_23_v1(input_day_23), 307);
  assert_eq!(day_23::day_23_v2(input_day_23), 160);

  let mut g2015_day_23 = c.benchmark_group("year_2015::day_23");
  g2015_day_23.bench_function("year_2015::day_23_v1", |b| {
    b.iter(|| day_23::day_23_v1(black_box(input_day_23)))
  });
  g2015_day_23.bench_function("year_2015::day_23_v2", |b| {
    b.iter(|| day_23::day_23_v2(black_box(input_day_23)))
  });
  g2015_day_23.finish();
}

fn year_2015_day_24(c: &mut Criterion) {
  let input_day_24 = include_str!("../inputs/year_2015/day_24_input");
  assert_eq!(day_24::day_24_v1(input_day_24), 10_439_961_859);
  assert_eq!(day_24::day_24_v2(input_day_24), 72_050_269);

  let mut g2015_day_24 = c.benchmark_group("year_2015::day_24");
  g2015_day_24.bench_function("year_2015::day_24_v1", |b| {
    b.iter(|| day_24::day_24_v1(black_box(input_day_24)))
  });
  g2015_day_24.bench_function("year_2015::day_24_v2", |b| {
    b.iter(|| day_24::day_24_v2(black_box(input_day_24)))
  });
  g2015_day_24.finish();
}

fn year_2015_day_25(c: &mut Criterion) {
  let input_day_25 = include_str!("../inputs/year_2015/day_25_input");
  assert_eq!(day_25::day_25(input_day_25), 19_980_801);

  let mut g2015_day_25 = c.benchmark_group("year_2015::day_25");
  g2015_day_25.bench_function("year_2015::day_25", |b| {
    b.iter(|| day_25::day_25(black_box(input_day_25)))
  });
  g2015_day_25.finish();
}

criterion_group!(
  benches,
  year_2015_day_01,
  year_2015_day_02,
  year_2015_day_03,
  year_2015_day_04,
  year_2015_day_05,
  year_2015_day_06,
  year_2015_day_07,
  year_2015_day_08,
  year_2015_day_09,
  year_2015_day_10,
  year_2015_day_11,
  year_2015_day_12,
  year_2015_day_13,
  year_2015_day_14,
  year_2015_day_15,
  year_2015_day_16,
  year_2015_day_17,
  year_2015_day_18,
  year_2015_day_19,
  year_2015_day_20,
  year_2015_day_21,
  year_2015_day_22,
  year_2015_day_23,
  year_2015_day_24,
  year_2015_day_25
);
criterion_main!(benches);
