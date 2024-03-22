use super::assembunny::Assembunny;

pub fn day_12_v1(input: impl Into<String>) -> i64 {
  let mut bunny: Assembunny = Assembunny::from_input(&input.into());
  bunny.run();

  bunny.registers[0]
}
pub fn day_12_v2(input: impl Into<String>) -> i64 {
  let mut bunny: Assembunny = Assembunny::from_input(&input.into());
  bunny.set_register("c", 1);
  bunny.run();

  bunny.registers[0]
}

solvable!(day_12, day_12_v1, day_12_v2, i64);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a";
    assert_eq!(day_12_v1(sample_one), 42);
  }
}
