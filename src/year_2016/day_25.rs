use super::assembunny::Assembunny;

pub fn day_25(input: impl Into<String>) -> u64 {
  let mut bunny: Assembunny = Assembunny::from_input(&input.into());
  let mut cnt = 0;
  loop {
    bunny.reset().set_register("a", cnt);
    if bunny.output() {
      return cnt as u64;
    }
    cnt += 1;
  }
}
