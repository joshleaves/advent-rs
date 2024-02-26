pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;

pub fn solve(day: u8, version: u8, input: String) -> Option<String> {
  match (day, version) {
    (1, 1) => return Some(format!("{}", day_01::day_01_v1(&input))),
    (1, 2) => return Some(format!("{}", day_01::day_01_v2(&input))),
    (2, 1) => return Some(format!("{}", day_02::day_02_v2(&input))),
    (2, 2) => return Some(format!("{}", day_02::day_02_v2(&input))),
    (3, 1) => return Some(format!("{}", day_03::day_03_v2(&input))),
    (3, 2) => return Some(format!("{}", day_03::day_03_v2(&input))),
    (4, 1) => return Some(format!("{}", day_04::day_04_v2(&input))),
    (4, 2) => return Some(format!("{}", day_04::day_04_v2(&input))),
    (5, 1) => return Some(format!("{}", day_05::day_05_v2(&input))),
    (5, 2) => return Some(format!("{}", day_05::day_05_v2(&input))),
    (6, 1) => return Some(format!("{}", day_06::day_06_v2(&input))),
    (6, 2) => return Some(format!("{}", day_06::day_06_v2(&input))),
    (7, 1) => return Some(format!("{}", day_07::day_07_v2(&input))),
    (7, 2) => return Some(format!("{}", day_07::day_07_v2(&input))),
    _ => return None,
  }
}
