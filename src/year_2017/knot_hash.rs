use itertools::Itertools;

pub fn knot_hash(jumps: &mut Vec<u32>) -> Vec<u8> {
  jumps.append(&mut vec![17, 31, 73, 47, 23]);
  _knot_hash(256, jumps, 64)
    .chunks(16)
    .map(|chunk| chunk.iter().fold(0, |acc, &val| acc ^ val) as u8)
    .collect::<Vec<u8>>()
}

pub(crate) fn _knot_hash(size: u32, jumps: &[u32], rounds: u32) -> Vec<u32> {
  let mut numbers: Vec<_> = (0..size).collect_vec();
  let num_lens = numbers.len();
  let mut jumps_done = 0;
  let mut skip_size = 0;

  for _ in 0..rounds {
    for jump in jumps.iter() {
      let extract = numbers[0..*jump as usize]
        .iter()
        .cloned()
        .rev()
        .collect_vec();
      numbers.splice(0..*jump as usize, extract);
      numbers.rotate_left(((jump + skip_size) % num_lens as u32) as usize);
      jumps_done += (jump + skip_size) % num_lens as u32;
      skip_size += 1;
    }
  }
  numbers.rotate_right(jumps_done as usize % num_lens);

  numbers
}
