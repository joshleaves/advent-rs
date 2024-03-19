fn parse_disc(input: &str) -> (u32, u32, u32) {
  let words: Vec<&str> = input.split_whitespace().collect();
  let disc = words[1].strip_prefix('#').unwrap().parse::<u32>().unwrap();
  let npos = words[3].parse::<u32>().unwrap();
  let spos = words[11].strip_suffix('.').unwrap().parse::<u32>().unwrap();

  (disc, npos, spos)
}

fn spin_the_discs(discs: Vec<(u32, u32, u32)>) -> u32 {
  let mut period = 1;
  let mut time = 0;
  for (disc, npos, spos) in discs.iter() {
    while ((time + disc + spos) % npos) != 0 {
      time += period;
    }
    period *= npos;
  }

  time
}

pub fn day_15_v1(input: impl Into<String>) -> u32 {
  let mut discs: Vec<(u32, u32, u32)> = vec![];
  for line in input.into().lines() {
    discs.push(parse_disc(line));
  }

  spin_the_discs(discs)
}

pub fn day_15_v2(input: impl Into<String>) -> u32 {
  let mut discs: Vec<(u32, u32, u32)> = vec![];
  for line in input.into().lines() {
    discs.push(parse_disc(line));
  }
  discs.push(((discs.len() + 1) as u32, 11, 0));

  spin_the_discs(discs)
}

solvable!(day_15, day_15_v1, day_15_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "Disc #1 has 5 positions; at time=0, it is at position 4.\n\
      Disc #2 has 2 positions; at time=0, it is at position 1.";
    assert_eq!(day_15_v1(sample_one), 5);
  }
}
