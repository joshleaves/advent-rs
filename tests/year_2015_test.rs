use advent_rs::year_2015::day_01;
use advent_rs::year_2015::day_02;
use advent_rs::year_2015::day_03;
use std::fs;

#[test]
fn year_2015_day_01() {
  let input: &str = &fs::read_to_string("./inputs/year_2015_day_01_input")
    .expect("Unable to read file")
    .to_string();

  assert_eq!(day_01::day_01_v1(input), 138);
  assert_eq!(day_01::day_01_v2(input), 1771);
}

#[test]
fn year_2015_day_02() {
  let input: &str = &fs::read_to_string("./inputs/year_2015_day_02_input")
    .expect("Unable to read file")
    .to_string();

  assert_eq!(day_02::day_02_v1(input), 1_588_178);
  assert_eq!(day_02::day_02_v2(input), 3_783_758);
}

#[test]
fn year_2015_day_03() {
  let input: &str = &fs::read_to_string("./inputs/year_2015_day_03_input")
    .expect("Unable to read file")
    .to_string();

  assert_eq!(day_03::day_03_v1(input), 2_081);
  assert_eq!(day_03::day_03_v2(input), 2_341);
}
