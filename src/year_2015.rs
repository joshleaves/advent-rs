pub mod day_01;

pub fn solve(day: u8, version: u8, input: String) -> String {
  match (day, version) {
    (1, 1) => return format!("{}", day_01::day_01_v1(&input)),
    (1, 2) => return format!("{}", day_01::day_01_v2(&input)),
    _ => {
      panic!("NOPE")
    }
  }
}
