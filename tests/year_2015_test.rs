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

#[test]
fn year_2015_day_01() {
  let input = include_str!("../inputs/year_2015_day_01_input");
  assert_eq!(day_01::day_01_v1(input), 138);
  assert_eq!(day_01::day_01_v2(input), 1771);
}

#[test]
fn year_2015_day_02() {
  let input = include_str!("../inputs/year_2015_day_02_input");
  assert_eq!(day_02::day_02_v1(input), 1_588_178);
  assert_eq!(day_02::day_02_v2(input), 3_783_758);
}

#[test]
fn year_2015_day_03() {
  let input = include_str!("../inputs/year_2015_day_03_input");
  assert_eq!(day_03::day_03_v1(input), 2081);
  assert_eq!(day_03::day_03_v2(input), 2341);
}

#[test]
fn year_2015_day_04() {
  let input = include_str!("../inputs/year_2015_day_04_input");
  assert_eq!(day_04::day_04_v1(input), 346_386);
  assert_eq!(day_04::day_04_v2(input), 9_958_218);
}

#[test]
fn year_2015_day_05() {
  let input = include_str!("../inputs/year_2015_day_05_input");
  assert_eq!(day_05::day_05_v1(input), 238);
  assert_eq!(day_05::day_05_v2(input), 69);
}

#[test]
fn year_2015_day_06() {
  let input = include_str!("../inputs/year_2015_day_06_input");
  assert_eq!(day_06::day_06_v1(input), 400_410);
  assert_eq!(day_06::day_06_v2(input), 15_343_601);
}

#[test]
fn year_2015_day_07() {
  let input = include_str!("../inputs/year_2015_day_07_input");
  assert_eq!(day_07::day_07_v1(input), 46_065);
  assert_eq!(day_07::day_07_v2(input), 14_134);
}

#[test]
fn year_2015_day_08() {
  let input = include_str!("../inputs/year_2015_day_08_input");
  assert_eq!(day_08::day_08_v1(input), 1_333);
  assert_eq!(day_08::day_08_v2(input), 2_046);
}

#[test]
fn year_2015_day_09() {
  let input = include_str!("../inputs/year_2015_day_09_input");
  assert_eq!(day_09::day_09_v1(input), 117);
  assert_eq!(day_09::day_09_v2(input), 909);
}
#[test]
fn year_2015_day_10() {
  let input = include_str!("../inputs/year_2015_day_10_input");
  assert_eq!(day_10::day_10_v1(input), 252_594);
  assert_eq!(day_10::day_10_v2(input), 3_579_328);
}

#[test]
fn year_2015_day_11() {
  let input = include_str!("../inputs/year_2015_day_11_input");
  assert_eq!(day_11::day_11_v1(input), "vzbxxyzz");
  assert_eq!(day_11::day_11_v2(input), "vzcaabcc");
}

#[test]
fn year_2015_day_12() {
  let input = include_str!("../inputs/year_2015_day_12_input");
  assert_eq!(day_12::day_12_v1(input), 111_754);
  assert_eq!(day_12::day_12_v2(input), 65_402);
}

#[test]
fn year_2015_day_13() {
  let input = include_str!("../inputs/year_2015_day_13_input");
  assert_eq!(day_13::day_13_v1(input), 709);
  assert_eq!(day_13::day_13_v2(input), 668);
}
