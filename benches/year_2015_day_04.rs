use advent_rs::year_2015::day_04;
use core::hash;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

use md5::Digest;
use md5::Md5;

struct Odometer {
  digits: [u8; 16],
  len: usize,
}

impl Odometer {
  fn new() -> Self {
    Self {
      digits: [b'0'; 16],
      len: 1,
    }
  }

  fn as_bytes(&self) -> &[u8] {
    let start = 16 - self.len;
    &self.digits[start..16]
  }

  fn next_bytes(&mut self) -> Option<&[u8]> {
    self.next()?;
    Some(self.as_bytes())
  }

  fn as_number(&self) -> u32 {
    let start = 16 - self.len;
    let mut value = 0;
    for d in &self.digits[start..16] {
      value = value * 10 + (d - b'0') as u32;
    }
    value
  }
}
impl Iterator for Odometer {
  type Item = ();

  fn next(&mut self) -> Option<Self::Item> {
    let mut i = 15;

    loop {
      self.digits[i] += 1;

      if self.digits[i] <= b'9' {
        // Pas de débordement → terminé
        return Some(());
      }

      // Débordement : on remet ce digit à 0 et on propage
      self.digits[i] = b'0';

      // On a débordé le digit le plus à gauche des actifs
      if i == 16 - self.len {
        // On ajoute un nouveau digit de poids fort
        if self.len == 10 {
          // 10 chiffres = 0..=4_294_967_295, on s'arrête là
          return None;
        }

        self.len += 1;
        let msd = 16 - self.len;
        self.digits[msd] = b'1'; // ex: 9 → 10, 99 → 100

        return Some(());
      }

      // Sinon on continue à propager à gauche
      i -= 1;
    }
  }
}

// const ZERO: u8 = b'0';    // 48
// const NINE: u8 = b'9';    // 57
// fn counter_to_number(counter: [u8; 9], length: usize) -> u32 {
//   let mut value: u32 = 0;
//   let mut index: usize = 0;
//   let mut mult: u32 = 1;
//   while index < length {
//     value = value + (counter[8 - index] - ZERO) as u32 * mult;
//     index = index + 1;
//     mult = mult * 10;
//   }
//   return value;
// }

pub fn find_hash_fast(input: &str, stop_value: u8) -> u32 {
  let mut md5 = Md5::new();
  md5.update(input);
  // let mut counter: [u8; 9] = [ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO];
  // let mut cnt_len = 1;
  let mut odo = Odometer::new();

  loop {
    let mut hasher = md5.clone();
    // hasher.update(&counter[(9 - cnt_len)..]);
    hasher.update(odo.next_bytes().unwrap());
    let hash = hasher.finalize();
    if hash[0] == 0 && hash[1] == 0 && hash[2] < stop_value {
      // return counter_to_number(counter, cnt_len)
      return odo.as_number();
    }
    // let mut carry = 0;
    // loop {
    //   let idx = (9 - 1) - carry;
    //   counter[idx] += 1;
    //   if counter[idx] <= NINE {
    //     break
    //   }
    //   counter[idx] = ZERO;
    //   carry = carry + 1;
    //   if carry == cnt_len {
    //     cnt_len = cnt_len + 1;
    //   }

    //   if cnt_len > 7 {
    //     return 0
    //   }
    // }
    // odo.next().unwrap();
  }
}

pub fn day_04_v1_fast(input: impl Into<String>) -> u32 {
  find_hash_fast(input.into().trim_end(), 16)
}

pub fn day_04_v2_fast(input: impl Into<String>) -> u32 {
  find_hash_fast(input.into().trim_end(), 1)
}

fn bench_year_2015_day_04_v1(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2015/day_04_input");
  assert_eq!(day_04::day_04_v1(input), 346_386);
  assert_eq!(day_04_v1_fast(input), 346_386);

  let mut group = c.benchmark_group("year_2015::day_04_v1");
  group.warm_up_time(Duration::from_millis(100));
  group.bench_with_input(BenchmarkId::new("Base", input.len()), input, |b, input| {
    b.iter(|| day_04::day_04_v1(input))
  });

  group.bench_with_input(BenchmarkId::new("Fast", input.len()), input, |b, input| {
    b.iter(|| day_04_v1_fast(input))
  });
  group.finish();
}

fn bench_year_2015_day_04_v2(c: &mut Criterion) {
  let input = include_str!("../inputs/year_2015/day_04_input");
  assert_eq!(day_04::day_04_v2(input), 9_958_218);
  assert_eq!(day_04_v2_fast(input), 9_958_218);

  let mut group = c.benchmark_group("year_2015::day_04_v2");
  group.warm_up_time(Duration::from_millis(100));

  group.bench_with_input(BenchmarkId::new("Base", input.len()), input, |b, input| {
    b.iter(|| day_04::day_04_v2(input))
  });
  group.bench_with_input(BenchmarkId::new("Fast", input.len()), input, |b, input| {
    b.iter(|| day_04_v2_fast(input))
  });
  group.finish();
}

criterion_group!(
  bench_year_2015_day_04,
  bench_year_2015_day_04_v1,
  bench_year_2015_day_04_v2
);
criterion_main!(bench_year_2015_day_04);
