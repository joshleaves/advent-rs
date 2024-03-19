//! Advent of Code 2015: Day 11: Corporate Policy

const PASSWORD_SIZE: usize = 8;

fn chr_to_string(input: &Vec<u8>) -> String {
  let Ok(s) = String::from_utf8(input.clone()) else {
    panic!("Invalid byte sequence: {:?}", input);
  };
  s.to_string()
}

fn password_is_valid(password: &Vec<u8>) -> bool {
  let mut criteria_1 = false;
  let mut criteria_3 = false;
  let mut criteria_3_chr = 0;
  for (idx, chr) in password.iter().enumerate() {
    if *chr == b'i' || *chr == b'l' || *chr == b'o' {
      return false;
    }
    if idx < PASSWORD_SIZE - 2 && *chr <= b'x' {
      if password[idx + 1] == *chr + 1 && password[idx + 2] == *chr + 2 {
        criteria_1 = true;
      }
    }
    if idx < PASSWORD_SIZE - 1 && *chr == password[idx + 1] {
      match criteria_3_chr {
        0 => criteria_3_chr = *chr,
        _ => criteria_3 = !criteria_3 && criteria_3_chr != *chr,
      }
    }
  }
  criteria_1 && criteria_3
}

fn update_password(password: Vec<u8>) -> Vec<u8> {
  let mut next_password = password.clone();
  next_password[7] += 1;
  let mut idx = 7;
  while next_password[idx] > b'z' {
    next_password[idx] = b'a';
    idx -= 1;
    next_password[idx] += 1;
  }
  for idx in (0..=6).rev() {
    if next_password[idx] == b'i' || next_password[idx] == b'l' || next_password[idx] == b'o' {
      for idxz in idx + 1..=7 {
        next_password[idxz] = b'z';
      }
      return update_password(next_password);
    }
  }
  next_password
}

pub fn day_11_v1(input: impl Into<String>) -> String {
  let mut password = input.into().trim_end().as_bytes().to_vec();
  loop {
    password = update_password(password);
    if password_is_valid(&password) {
      break;
    }
  }
  return chr_to_string(&password);
}

pub fn day_11_v2(input: impl Into<String>) -> String {
  let mut password = input.into().trim_end().as_bytes().to_vec();
  let mut pass_valid = false;
  loop {
    password = update_password(password);
    if password_is_valid(&password) {
      if pass_valid {
        break;
      }
      pass_valid = true;
    }
  }
  return chr_to_string(&password);
}

solvable!(day_11, day_11_v1, day_11_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn passwords_are_valid() {
    let sample_one: [&str; 2] = [
      "abcdffaa",
      "ghjaabcc",
    ];
    for sample in sample_one.iter() {
      let password = sample.as_bytes().to_vec();
      assert_eq!(password_is_valid(&password), true);
    }
  }

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, &str); 2] = [
      ("abcdefgh", "abcdffaa"),
      ("ghijklmn", "ghjaabcc"),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_11_v1(*sample), *result);
    }
  }
}
