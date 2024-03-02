//! Advent of Code 2015: Day 22: Wizard Simulator 20XX

const WORST_MANA: usize = 100_000;

fn combat(
  mut user_hp: i16,
  mut user_mp: i16,
  mut boss_hp: i8,
  boss_dmg: i8,
  mut effects: Vec<i8>,
  player_turn: bool,
  mut best_mana: usize,
  version: u8,
  mana: usize,
) -> usize {
  if mana >= best_mana || user_mp < 0 {
    return WORST_MANA;
  }
  if version == 2 && player_turn {
    user_hp -= 1;
  }
  if user_hp <= 0 {
    return WORST_MANA;
  }
  let user_df = if effects[0] > 0 { 7 } else { 0 };
  if effects[1] > 0 {
    boss_hp -= 3;
  }
  if effects[2] > 0 {
    user_mp += 101;
  }
  effects = effects.iter().map(|e| std::cmp::max(0, e - 1)).collect();
  if boss_hp <= 0 {
    return std::cmp::min(best_mana, mana);
  }

  if !player_turn {
    user_hp -= std::cmp::max(1, boss_dmg - user_df) as i16;
    if user_hp > 0 {
      return combat(
        user_hp,
        user_mp,
        boss_hp,
        boss_dmg,
        effects,
        true,
        best_mana,
        version,
        mana,
      );
    } else {
      return WORST_MANA;
    }
  }

  // Magic Missile
  let mut next_mana = combat(
    user_hp,
    user_mp - 53,
    boss_hp - 4,
    boss_dmg,
    effects.clone(),
    false,
    best_mana,
    version,
    mana + 53,
  );
  best_mana = std::cmp::min(best_mana, next_mana);
  // Drain
  next_mana = combat(
    user_hp + 2,
    user_mp - 73,
    boss_hp - 2,
    boss_dmg,
    effects.clone(),
    false,
    best_mana,
    version,
    mana + 73,
  );
  best_mana = std::cmp::min(best_mana, next_mana);
  // Shield
  if effects[0] == 0 {
    next_mana = combat(
      user_hp,
      user_mp - 113,
      boss_hp,
      boss_dmg,
      vec![
        6,
        effects[1],
        effects[2],
      ],
      false,
      best_mana,
      version,
      mana + 113,
    );
    best_mana = std::cmp::min(best_mana, next_mana);
  }
  // Poison
  if effects[1] == 0 {
    next_mana = combat(
      user_hp,
      user_mp - 173,
      boss_hp,
      boss_dmg,
      vec![
        effects[0],
        6,
        effects[2],
      ],
      false,
      best_mana,
      version,
      mana + 173,
    );
    best_mana = std::cmp::min(best_mana, next_mana);
  }

  // Recharge
  if effects[2] == 0 {
    next_mana = combat(
      user_hp,
      user_mp - 229,
      boss_hp,
      boss_dmg,
      vec![
        effects[0],
        effects[1],
        5,
      ],
      false,
      best_mana,
      version,
      mana + 229,
    );
    best_mana = std::cmp::min(best_mana, next_mana);
  }

  best_mana
}

fn parse_input(boss_input: &str) -> (i8, i8) {
  let mut hit_points: i8 = 0;
  let mut damage: i8 = 0;
  for line in boss_input.lines() {
    let parts: Vec<&str> = line.split(": ").collect();
    let value = parts[1].parse::<i8>().unwrap();
    match parts[0] {
      "Hit Points" => hit_points = value,
      "Damage" => damage = value,
      _ => {
        panic!("Invalid input: {}", line)
      }
    }
  }

  (hit_points, damage)
}

pub fn day_22_v1(input: impl Into<String>) -> usize {
  let (boss_hp, boss_dmg) = parse_input(&input.into());
  combat(
    50,
    500,
    boss_hp,
    boss_dmg,
    vec![0, 0, 0],
    true,
    WORST_MANA,
    1,
    0,
  )
}

pub fn day_22_v2(input: impl Into<String>) -> usize {
  let (boss_hp, boss_dmg) = parse_input(&input.into());
  combat(
    50,
    500,
    boss_hp,
    boss_dmg,
    vec![0, 0, 0],
    true,
    WORST_MANA,
    2,
    0,
  )
}

solvable!(day_22, day_22_v1, day_22_v2, usize);
