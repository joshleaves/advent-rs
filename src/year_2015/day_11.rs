//! Advent of Code 2015: Day 11: Corporate Policy

const PASSWORD_SIZE: usize = 8;

fn password_is_valid(password: &[u8]) -> bool {
  let mut criteria_1 = false;
  let mut criteria_3 = false;
  let mut criteria_3_chr = 0;
  for (idx, chr) in password.iter().enumerate() {
    if !criteria_1
      && idx < PASSWORD_SIZE - 2
      && password[idx + 1] == *chr + 1
      && password[idx + 2] == *chr + 2
    {
      criteria_1 = true;
    }
    if !criteria_3 && idx < PASSWORD_SIZE - 1 && *chr == password[idx + 1] {
      match criteria_3_chr {
        0 => criteria_3_chr = *chr,
        _ => criteria_3 = !criteria_3 && criteria_3_chr != *chr,
      }
    }
  }
  criteria_1 && criteria_3
}

#[inline]
fn update_password(mut password: Vec<u8>) -> Vec<u8> {
  password[7] += 1;
  let mut idx = 7;
  while password[idx] > b'z' {
    password[idx] = b'a';
    idx -= 1;
    password[idx] += 1;
  }
  for idx in (0..=6).rev() {
    if password[idx] == b'i' || password[idx] == b'l' || password[idx] == b'o' {
      for character in password[idx + 1..=7].iter_mut() {
        *character = b'z';
      }
      return update_password(password);
    }
  }
  password
}

pub fn day_11_v1(input: impl Into<String>) -> String {
  let mut password = input.into().trim_end().as_bytes().to_vec();
  while !password_is_valid(&password) {
    password = update_password(password);
  }

  String::from_utf8(password).unwrap().to_string()
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

  String::from_utf8(password).unwrap().to_string()
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
      assert!(password_is_valid(&password));
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
