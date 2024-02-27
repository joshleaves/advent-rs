use itertools::Itertools;

fn look_and_say(input: Vec<u8>) -> Vec<u8> {
  let mut current_cnt: u8 = 0;
  let mut current_chr: u8 = input[0];
  let mut new_seq = input.iter().fold(vec![], |mut acc, chr| {
    if *chr == current_chr {
      current_cnt += 1;
    } else {
      acc.push(current_cnt);
      acc.push(current_chr);
      current_cnt = 1;
      current_chr = *chr;
    }
    acc
  });
  new_seq.push(current_cnt);
  new_seq.push(current_chr);

  new_seq
}

fn string_to_chr(input: &str) -> Vec<u8> {
  let Some(first_line) = input.lines().next() else {
    panic!("Invalid input: {}", input);
  };
  first_line
    .as_bytes()
    .iter()
    .map(|chr| chr - 48)
    .collect_vec()
}

pub fn day_10_v1(input: &str) -> u32 {
  let mut next_input: Vec<u8> = string_to_chr(input);
  for _n in 1..=40 {
    next_input = look_and_say(next_input);
  }
  next_input.len() as u32
}

pub fn day_10_v2(input: &str) -> u32 {
  let mut next_input: Vec<u8> = string_to_chr(input);
  for _n in 1..=50 {
    next_input = look_and_say(next_input);
  }
  next_input.len() as u32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn looks_and_says_over_strings() {
    let sample_one: [(Vec<u8>, Vec<u8>); 5] = [
      (vec![1], vec![1, 1]),
      (vec![1, 1], vec![2, 1]),
      (vec![2, 1], vec![1, 2, 1, 1]),
      (vec![1, 2, 1, 1], vec![1, 1, 1, 2, 2, 1]),
      (vec![1, 1, 1, 2, 2, 1], vec![3, 1, 2, 2, 1, 1]),
    ];
    for (sample, result) in sample_one {
      assert_eq!(look_and_say(sample), result);
    }
  }

  // #[test]
  // fn works_with_samples_v2() {
  //   let sample_two = "London to Dublin = 464
  //   London to Belfast = 518
  //   Dublin to Belfast = 141";
  //   assert_eq!(day_10_v2(sample_two), 982);
  // }
}
