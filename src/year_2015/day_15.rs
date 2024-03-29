//! Advent of Code 2015: Day 15: Science for Hungry People

struct Ingredient {
  capacity: i32,
  durability: i32,
  flavor: i32,
  texture: i32,
  calories: i32,
}

impl Ingredient {
  fn new(input: &str) -> Self {
    let parts: Vec<_> = input.split_whitespace().collect();

    Self {
      capacity: parts[2].strip_suffix(',').unwrap().parse::<i32>().unwrap(),
      durability: parts[4].strip_suffix(',').unwrap().parse::<i32>().unwrap(),
      flavor: parts[6].strip_suffix(',').unwrap().parse::<i32>().unwrap(),
      texture: parts[8].strip_suffix(',').unwrap().parse::<i32>().unwrap(),
      calories: parts[10].trim_end().parse::<i32>().unwrap(),
    }
  }
}

fn score_recipe<STP>(ingredients: &[Ingredient], quantities: &[i32], cal: &STP) -> i32
where
  STP: Fn(i32) -> bool,
{
  let mut capacity = 0;
  let mut durability = 0;
  let mut flavor = 0;
  let mut texture = 0;
  let mut calories = 0;

  for i in 0..ingredients.len() {
    capacity += ingredients[i].capacity * quantities[i];
    durability += ingredients[i].durability * quantities[i];
    flavor += ingredients[i].flavor * quantities[i];
    texture += ingredients[i].texture * quantities[i];
    calories += ingredients[i].calories * quantities[i];
  }

  if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 || cal(calories) {
    return 0;
  }

  capacity * durability * flavor * texture
}

fn try_best_ingredient<STP>(
  ingredients: &Vec<Ingredient>,
  quantities: &mut [i32],
  depth: usize,
  cal: &STP,
) -> i32
where
  STP: Fn(i32) -> bool,
{
  if depth == quantities.len() {
    if quantities.iter().sum::<i32>() != 100 {
      return 0;
    }
    return score_recipe(ingredients, quantities, cal);
  }
  let spoons_used = quantities.iter().take(depth).sum::<i32>();
  let mut best_score = 0;
  for i in 0..=(100 - spoons_used) {
    quantities[depth] = i;
    let next_score = try_best_ingredient(ingredients, quantities, depth + 1, cal);
    best_score = std::cmp::max(best_score, next_score);
  }

  best_score
}

pub fn day_15_v1(input: impl Into<String>) -> i32 {
  let mut ingredients: Vec<Ingredient> = vec![];
  for line in input.into().lines() {
    ingredients.push(Ingredient::new(line));
  }
  let mut quantities = vec![1; ingredients.len()];

  try_best_ingredient(&ingredients, &mut quantities, 0, &|_c| false)
}

pub fn day_15_v2(input: impl Into<String>) -> i32 {
  let mut ingredients: Vec<Ingredient> = vec![];
  for line in input.into().lines() {
    ingredients.push(Ingredient::new(line));
  }
  let mut quantities = vec![0; ingredients.len()];

  try_best_ingredient(&ingredients, &mut quantities, 0, &|c| c != 500)
}

solvable!(day_15, day_15_v1, day_15_v2, i32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample: &str =
      "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
      Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    assert_eq!(day_15_v1(sample), 62_842_880);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample: &str =
      "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
      Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    assert_eq!(day_15_v2(sample), 57_600_000);
  }
}
