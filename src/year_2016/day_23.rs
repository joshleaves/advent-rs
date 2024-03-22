use super::assembunny::Assembunny;

// pub fn day_12_v1(input: impl Into<String>) -> i64 {
// }
// pub fn day_12_v2(input: impl Into<String>) -> i64 {
//   let mut bunny: Assembunny = Assembunny::from_input(&input.into());
//   bunny.set_register("c", 1);
//   bunny.run();

//   bunny.registers[0]
// }

pub fn day_23_v1(input: impl Into<String>) -> i64 {
  let mut bunny: Assembunny = Assembunny::from_input(&input.into());
  bunny.set_register("a", 7);
  bunny.run();

  bunny.registers[0]
}

pub fn day_23_v2(input: impl Into<String>) -> i64 {
  let mut bunny: Assembunny = Assembunny::from_input(&input.into());
  bunny.set_register("a", 12);
  bunny.run();

  bunny.registers[0]
}

solvable!(day_23, day_23_v1, day_23_v2, i64);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "cpy 2 a\n\
      tgl a\n\
      tgl a\n\
      tgl a\n\
      cpy 1 a\n\
      dec a\n\
      dec a";
    assert_eq!(day_23_v1(sample_one), 3);
  }
}
