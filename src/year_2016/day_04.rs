use itertools::Itertools;

const NOS: &[u8] = b"northpole object storage";

fn verify_checksum(checksum: &str, input: &str) -> bool {
  input
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
    .collect::<String>()
    .starts_with(checksum)
}

fn verify_translation(input: &str, value: u32, reference: &[u8]) -> bool {
  for (idx, chr) in input.chars().enumerate() {
    let translated = if chr == '-' {
      ' '
    } else {
      (((((chr as u8) as i16 - 97 + value as i16) % 26) + 97) as u8) as char
    };
    if translated != reference[idx] as char {
      return false;
    }
  }
  true
}

#[inline]
fn parse_line(input: &str) -> (&str, u32, &str) {
  let input = input.strip_suffix(']').unwrap();
  let (input, checksum) = input.rsplit_once('[').unwrap();
  let (input, value_str) = input.rsplit_once('-').unwrap();
  let value = value_str.parse::<u32>().unwrap();

  (input, value, checksum)
}

pub fn day_04_v1(input: impl Into<String>) -> u32 {
  let mut result: u32 = 0;
  for line in input.into().lines() {
    let (letters, value, checksum) = parse_line(line);
    if verify_checksum(checksum, letters) {
      result += value
    }
  }

  result
}

pub fn day_04_v2(input: impl Into<String>) -> u32 {
  for line in input.into().lines() {
    let (letters, value, checksum) = parse_line(line);
    if !verify_checksum(checksum, letters) {
      continue;
    }
    // if translate_line(&letters, value) == NOS {
    if verify_translation(letters, value, NOS) {
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
  fn works_with_samples_v2() {
    assert!(verify_translation(
      "qzmt-zixmtkozy-ivhz",
      343,
      b"very encrypted name"
    ));
  }
}
