pub mod day_01;
pub mod day_02;
pub mod day_03;

pub fn solve(day: u8, version: u8, input: String) -> String {
  match (day, version) {
    (1, 1) => return format!("{}", day_01::day_01_v1(&input)),
    (1, 2) => return format!("{}", day_01::day_01_v2(&input)),
    (2, 1) => return format!("{}", day_02::day_02_v2(&input)),
    (2, 2) => return format!("{}", day_02::day_02_v2(&input)),
    (3, 1) => return format!("{}", day_03::day_03_v2(&input)),
    (3, 2) => return format!("{}", day_03::day_03_v2(&input)),
    _ => {
      eprintln!("advent-rs: Not implemented (Year 2015 Day {day:02}v{version})");
      std::process::exit(1)
    }
  }
}
