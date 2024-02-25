use advent_rs::year_2015::day_01;
use advent_rs::year_2015::day_02;
use advent_rs::year_2015::day_03;
use advent_rs::year_2015::day_04;
use advent_rs::year_2015::day_05;
use advent_rs::year_2015::day_06;

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
  // assert_eq!(day_06::day_06_v2(input), 15_343_601);
}
