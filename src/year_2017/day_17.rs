pub fn day_17_v1(input: impl Into<String>) -> u32 {
  let input = input.into().trim_end().parse::<u32>().unwrap();
  let mut spinlock: Vec<u32> = vec![0];
  let mut idx = 0;
  for i in 1..=2017 {
    idx = ((idx + input) % spinlock.len() as u32) + 1;
    spinlock.insert(idx as usize, i);
  }

  idx = spinlock.iter().position(|n| *n == 2017).unwrap() as u32;
  idx = (idx + 1) % spinlock.len() as u32;
  spinlock[idx as usize]
}

pub fn day_17_v2(input: impl Into<String>) -> u32 {
  let input = input.into().trim_end().parse::<u32>().unwrap();
  let mut answer = 0;
  let mut idx = 0;
  for i in 1..=5_000_000 {
    idx = (idx + input) % i;
    if idx == 0 {
      answer = i;
    }
    idx += 1;
  }
  answer
}
solvable!(day_17, day_17_v1, day_17_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_17_v1("3"), 638);
  }
}
