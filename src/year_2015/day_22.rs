//! Advent of Code 2015: Day 22: Wizard Simulator 20XX

const WORST_MANA: usize = 100_000;

fn combat(
  mut user: (i16, i16),
  mut boss: (i8, i8),
  mut effects: Vec<i8>,
  player_turn: bool,
  mut best: usize,
  ver: u8,
  mana: usize,
) -> usize {
  if mana >= best || user.1 < 0 {
    return WORST_MANA;
  }
  if ver == 2 && player_turn {
    user.0 -= 1;
  }
  if user.0 <= 0 {
    return WORST_MANA;
  }
  let user_df = if effects[0] > 0 { 7 } else { 0 };
  if effects[1] > 0 {
    boss.0 -= 3;
  }
  if effects[2] > 0 {
    user.1 += 101;
  }
  effects = effects.iter().map(|e| std::cmp::max(0, e - 1)).collect();
  if boss.0 <= 0 {
    return std::cmp::min(best, mana);
  }

  if !player_turn {
    user.0 -= std::cmp::max(1, boss.1 - user_df) as i16;
    if user.0 > 0 {
      return combat(user, boss, effects, true, best, ver, mana);
    } else {
      return WORST_MANA;
    }
  }

  // Magic Missile
  let mut next_mana = combat(
    (user.0, user.1 - 53),
    (boss.0 - 4, boss.1),
    effects.clone(),
    false,
    best,
    ver,
    mana + 53,
  );
  best = std::cmp::min(best, next_mana);
  // Drain
  next_mana = combat(
    (user.0 + 2, user.1 - 73),
    (boss.0 - 2, boss.1),
    effects.clone(),
    false,
    best,
    ver,
    mana + 73,
  );
  best = std::cmp::min(best, next_mana);
  // Shield
  if effects[0] == 0 {
    next_mana = combat(
      (user.0, user.1 - 113),
      boss,
      vec![
        6,
        effects[1],
        effects[2],
      ],
      false,
      best,
      ver,
      mana + 113,
    );
    best = std::cmp::min(best, next_mana);
  }
  // Poison
  if effects[1] == 0 {
    next_mana = combat(
      (user.0, user.1 - 173),
      boss,
      vec![
        effects[0],
        6,
        effects[2],
      ],
      false,
      best,
      ver,
      mana + 173,
    );
    best = std::cmp::min(best, next_mana);
  }

  // Recharge
  if effects[2] == 0 {
    next_mana = combat(
      (user.0, user.1 - 229),
      boss,
      vec![
        effects[0],
        effects[1],
        5,
      ],
      false,
      best,
      ver,
      mana + 229,
    );
    best = std::cmp::min(best, next_mana);
  }

  best
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
  let boss = parse_input(&input.into());
  combat((50, 500), boss, vec![0, 0, 0], true, WORST_MANA, 1, 0)
}

pub fn day_22_v2(input: impl Into<String>) -> usize {
  let boss = parse_input(&input.into());
  combat((50, 500), boss, vec![0, 0, 0], true, WORST_MANA, 2, 0)
}

solvable!(day_22, day_22_v1, day_22_v2, usize);
