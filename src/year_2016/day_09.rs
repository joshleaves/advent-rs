fn decompressed_size(mut input: &str, deep_decompress: bool) -> usize {
  let mut result: usize = 0;
  while !input.is_empty() {
    if let Some(next_paren_l) = input.find('(') {
      let next_paren_r = input.find(')').unwrap();
      let nums: Vec<_> = input[next_paren_l + 1..next_paren_r].split('x').collect();
      let (duration, repeat) = (
        nums[0].parse::<usize>().unwrap(),
        nums[1].parse::<usize>().unwrap(),
      );
      result += next_paren_l;
      input = &input[next_paren_r + 1..];
      if deep_decompress {
        let dup_elt = &input[..duration];
        result += decompressed_size(dup_elt, true) * repeat;
      } else {
        result += duration * repeat;
      }
      input = &input[duration..];
    } else {
      result += input.len();
      input = "";
    }
  }

  result
}

pub fn day_09_v1(input: impl Into<String>) -> usize {
  decompressed_size(&input.into(), false)
}

pub fn day_09_v2(input: impl Into<String>) -> usize {
  decompressed_size(&input.into(), true)
}

solvable!(day_09, day_09_v1, day_09_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, usize); 6] = [
      ("ADVENT", 6),
      ("A(1x5)BC", 7),
      ("(3x3)XYZ", 9),
      ("A(2x2)BCD(2x2)EFG", 11),
      ("(6x1)(1x3)A", 6),
      ("X(8x2)(3x3)ABCY", 18),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_09_v1(sample), result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, usize); 4] = [
      ("(3x3)XYZ", 9),
      ("X(8x2)(3x3)ABCY", 20),
      ("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241_920),
      (
        "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
        445,
      ),
    ];
    for (sample, result) in sample_two {
      assert_eq!(day_09_v2(sample), result);
    }
  }
}
