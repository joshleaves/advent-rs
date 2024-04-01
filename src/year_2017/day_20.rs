use std::{collections::HashMap, ops::AddAssign};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
  x: i64,
  y: i64,
  z: i64,
}

impl Point {
  fn new(input: &str) -> Self {
    let mut input = input.trim().split_once('<').unwrap().1;
    input = input.strip_suffix('>').unwrap();
    let mut parts = input
      .split(',')
      .map(|number| number.parse::<i64>().unwrap());

    Point {
      x: parts.next().unwrap(),
      y: parts.next().unwrap(),
      z: parts.next().unwrap(),
    }
  }
}

impl AddAssign for Point {
  fn add_assign(&mut self, other: Point) {
    *self = Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    };
  }
}
struct Particle {
  id: u16,
  position: Point,
  velocity: Point,
  acceleration: Point,
}

impl Particle {
  fn step(&mut self) {
    self.velocity += self.acceleration;
    self.position += self.velocity;
  }

  fn distance(&self) -> i64 {
    self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
  }

  fn new(id: u16, input: &str) -> Self {
    let mut parts = input.split(", ");
    let position = Point::new(parts.next().unwrap());
    let velocity = Point::new(parts.next().unwrap());
    let acceleration = Point::new(parts.next().unwrap());

    Particle {
      id,
      position,
      velocity,
      acceleration,
    }
  }
}

pub fn day_20_v1(input: impl Into<String>) -> u16 {
  let mut particles: Vec<Particle> = input
    .into()
    .lines()
    .enumerate()
    .map(|(idx, line)| Particle::new(idx as u16, line))
    .collect();
  (0..1_000).for_each(|_| particles.iter_mut().for_each(|particle| particle.step()));

  particles
    .iter()
    .min_by_key(|particle| particle.distance())
    .unwrap()
    .id
}

pub fn day_20_v2(input: impl Into<String>) -> u16 {
  let mut particles: Vec<Particle> = input
    .into()
    .lines()
    .enumerate()
    .map(|(idx, line)| Particle::new(idx as u16, line))
    .collect();
  (0..1_000).for_each(|_| {
    let mut positions: HashMap<Point, u8> = HashMap::new();
    particles.iter_mut().for_each(|particle| {
      particle.step();
      *positions.entry(particle.position).or_default() += 1;
    });
    particles.retain(|particle| *positions.get(&particle.position).unwrap() == 1);
  });

  particles.len() as u16
}

solvable!(day_20, day_20_v1, day_20_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [&str; 4] = [
      "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\np=<4,0,0>, v=<0,0,0>, a=<-2,0,0>",
      "p=<4,0,0>, v=<1,0,0>, a=<-1,0,0>\np=<2,0,0>, v=<-2,0,0>, a=<-2,0,0>",
      "p=<4,0,0>, v=<0,0,0>, a=<-1,0,0>\np=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>",
      "p=<3,0,0>, v=<-1,0,0>, a=<-1,0,0>\np=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>",
    ];
    for sample in sample_one {
      assert_eq!(day_20_v1(sample), 0);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = [
      "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>",
      "p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>",
      "p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>",
      "p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>",
    ]
    .join("\n");

    assert_eq!(day_20_v2(sample_two), 1);
  }
}
