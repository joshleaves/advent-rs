use itertools::Itertools;
use regex::Regex;

const NOS: &str = "northpole object storage";

fn verify_checksum(checksum: &str, input: &str) -> bool {
  let letters = input
    .chars()
    .unique()
    .sorted_by_key(|chr| {
      let cnt_chr = if *chr == '-' {
        0
      } else {
        input.chars().filter(|in_chr| in_chr == chr).count()
      };
      [
        -(cnt_chr as i8),
        *chr as i8,
      ]
    })
    .take(5)
    .collect::<String>();

  letters == checksum
}

fn translate_line(input: &str, value: u32) -> String {
  input
    .chars()
    .map(|chr| {
      if chr == '-' {
        ' '
      } else {
        (((((chr as u8) as i16 - 97 + value as i16) % 26) + 97) as u8) as char
      }
    })
    .collect::<String>()
}

pub fn day_04_v1(input: impl Into<String>) -> u32 {
  let re = Regex::new(r"(?<letters>[\w-]+)-(?<value>\d+)\[(?<checksum>\w+)\]").unwrap();
  let mut result: u32 = 0;
  for line in input.into().lines() {
    let Some(caps) = re.captures(line) else {
      panic!("Incorrect input: {}", line);
    };
    if verify_checksum(&caps["checksum"], &caps["letters"]) {
      result += caps["value"].parse::<u32>().unwrap();
    }
  }

  result
}

pub fn day_04_v2(input: impl Into<String>) -> u32 {
  let re = Regex::new(r"(?<letters>[\w-]+)-(?<value>\d+)\[(?<checksum>\w+)\]").unwrap();
  for line in input.into().lines() {
    let Some(caps) = re.captures(line) else {
      panic!("Incorrect input: {}", line);
    };
    if !verify_checksum(&caps["checksum"], &caps["letters"]) {
      continue;
    }
    let value = caps["value"].parse::<u32>().unwrap();
    if translate_line(&caps["letters"], value) == NOS {
      return value;
    }
  }

  0
}

solvable!(day_04, day_04_v1, day_04_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u32); 4] = [
      ("aaaaa-bbb-z-y-x-123[abxyz]", 123),
      ("a-b-c-d-e-f-g-h-987[abcde]", 987),
      ("not-a-real-room-404[oarel]", 404),
      ("totally-real-room-200[decoy]", 0),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_04_v1(sample), result);
    }
  }

  #[test]
  fn can_translate_string() {
    let sample = ("qzmt-zixmtkozy-ivhz", 343, "very encrypted name");

    assert_eq!(translate_line(sample.0, sample.1), sample.2);
  }
}
