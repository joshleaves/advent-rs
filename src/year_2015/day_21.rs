//! Advent of Code 2015: Day 21: RPG Simulator 20XX

struct Equipment {
  cost: u8,
  damage: u8,
  armor: u8,
}

const WEAPONS: [Equipment; 5] = [
  Equipment {
    cost: 8,
    damage: 4,
    armor: 0,
  }, // name: "Dagger"
  Equipment {
    cost: 10,
    damage: 5,
    armor: 0,
  }, // name: "Shortsword"
  Equipment {
    cost: 25,
    damage: 6,
    armor: 0,
  }, // name: "Warhammer"
  Equipment {
    cost: 40,
    damage: 7,
    armor: 0,
  }, // name: "Longsword"
  Equipment {
    cost: 74,
    damage: 8,
    armor: 0,
  }, // name: "Greataxe"
];
const ARMORS: [Equipment; 6] = [
  Equipment {
    cost: 0,
    damage: 0,
    armor: 0,
  }, // name: "None"
  Equipment {
    cost: 13,
    damage: 0,
    armor: 1,
  }, // name: "Leather"
  Equipment {
    cost: 31,
    damage: 0,
    armor: 2,
  }, // name: "Chainmail"
  Equipment {
    cost: 53,
    damage: 0,
    armor: 3,
  }, // name: "Splintmail"
  Equipment {
    cost: 75,
    damage: 0,
    armor: 4,
  }, // name: "Bandedmail"
  Equipment {
    cost: 102,
    damage: 0,
    armor: 5,
  }, // name: "Platemail"
];
const ACCESSORIES: [Equipment; 8] = [
  Equipment {
    cost: 0,
    damage: 0,
    armor: 0,
  }, // name: "None"
  Equipment {
    cost: 0,
    damage: 0,
    armor: 0,
  }, // name: "None"
  Equipment {
    cost: 25,
    damage: 1,
    armor: 0,
  }, // name: "Damage +1"
  Equipment {
    cost: 50,
    damage: 2,
    armor: 0,
  }, // name: "Damage +2"
  Equipment {
    cost: 100,
    damage: 3,
    armor: 0,
  }, // name: "Damage +3"
  Equipment {
    cost: 20,
    damage: 0,
    armor: 1,
  }, // name: "Defense +1"
  Equipment {
    cost: 40,
    damage: 0,
    armor: 2,
  }, // name: "Defense +2"
  Equipment {
    cost: 80,
    damage: 0,
    armor: 3,
  }, // name: "Defense +3"
];

const PLAYER_HP: u8 = 100;

struct Character {
  hit_points: u8,
  damage: u8,
  armor: u8,
}

impl Character {
  fn from_equipment(equipment: [&Equipment; 4]) -> (usize, Self) {
    let hit_points = PLAYER_HP;
    let cost: usize = equipment.iter().map(|s| s.cost as usize).sum();
    let damage: u8 = equipment.iter().map(|s| s.damage).sum();
    let armor: u8 = equipment.iter().map(|s| s.armor).sum();

    (
      cost as usize,
      Character {
        hit_points,
        damage,
        armor,
      },
    )
  }

  fn from_string(boss_input: &str) -> Character {
    let mut hit_points: u8 = 0;
    let mut damage: u8 = 0;
    let mut armor: u8 = 0;
    for line in boss_input.lines() {
      let parts: Vec<&str> = line.split(": ").collect();
      let value = parts[1].parse::<u8>().unwrap();
      match parts[0] {
        "Hit Points" => hit_points = value,
        "Damage" => damage = value,
        "Armor" => armor = value,
        _ => {
          panic!("Invalid input: {}", line)
        }
      }
    }

    Character {
      hit_points,
      damage,
      armor,
    }
  }
}

fn simulate_battle(player: &Character, boss: &Character) -> bool {
  let player_damage: u8 = std::cmp::max(1, player.damage as i8 - boss.armor as i8) as u8;
  let boss_damage: u8 = std::cmp::max(1, boss.damage as i8 - player.armor as i8) as u8;
  let player_turns = (player.hit_points / boss_damage) + 1;
  let boss_turns = (boss.hit_points / player_damage) + 1;

  player_turns >= boss_turns
}

fn try_on_equipment<CMP>(boss: &Character, initial_best: usize, price_updater: CMP) -> usize
where
  CMP: Fn(bool, usize, usize) -> usize,
{
  let mut best_price: usize = initial_best;
  for weapon in WEAPONS.iter() {
    for armor in ARMORS.iter() {
      for acc_1 in ACCESSORIES[0..=6].iter() {
        for acc_2 in ACCESSORIES[1..=7].iter() {
          let (cost, player) = Character::from_equipment([
            weapon,
            armor,
            acc_1,
            acc_2,
          ]);
          let result = simulate_battle(&player, boss);
          best_price = price_updater(result, best_price, cost);
        }
      }
    }
  }

  best_price
}

pub fn day_21_v1<'a>(input: impl Into<String>) -> usize {
  let boss = Character::from_string(&input.into());
  let price_updater = |result: bool, best_price, next_price| {
    if result {
      std::cmp::min(best_price, next_price)
    } else {
      best_price
    }
  };

  try_on_equipment(&boss, 256, price_updater)
}

pub fn day_21_v2<'a>(input: impl Into<String>) -> usize {
  let boss = Character::from_string(&input.into());
  let price_updater = |result: bool, best_price, next_price| {
    if !result {
      std::cmp::max(best_price, next_price)
    } else {
      best_price
    }
  };

  try_on_equipment(&boss, 0, price_updater)
}

solvable!(day_21, day_21_v1, day_21_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn wins_the_fight_with_samples_v1() {
    let sample_boss = Character::from_string("Hit Points: 12\nDamage: 7\nArmor: 2");
    let sample_player = Character::from_string("Hit Points: 8\nDamage: 5\nArmor: 5");
    assert_eq!(simulate_battle(&sample_player, &sample_boss), true);
  }
}
