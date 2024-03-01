use std::collections::HashMap;

enum Operation<'a> {
  Assign(&'a str),
  Not(&'a str),
  And(&'a str, &'a str),
  Or(&'a str, &'a str),
  LeftShift(&'a str, &'a str),
  RightShift(&'a str, &'a str),
}

struct LogicalGate<'a> {
  operation: Operation<'a>,
  value: Option<u32>,
}

fn value_of<'a>(gates: &mut HashMap<&'a str, LogicalGate<'a>>, wire: &'a str) -> Option<u32> {
  if let Ok(value) = wire.parse::<u32>() {
    return Some(value);
  } else {
    return find_value_of_wire(gates, wire);
  }
}

fn find_value_of_wire<'a>(
  gates: &mut HashMap<&'a str, LogicalGate<'a>>,
  wire: &'a str,
) -> Option<u32> {
  let Some(gate) = gates.get(wire) else {
    panic!("Invalid wire: {}", wire);
  };
  if gate.value.is_some() {
    return gate.value;
  }
  let wire_value = match gate.operation {
    Operation::Assign(value) => value_of(gates, value)?,
    Operation::Not(value) => !value_of(gates, value)?,
    Operation::And(lhs, rhs) => value_of(gates, lhs)? & value_of(gates, rhs)?,
    Operation::Or(lhs, rhs) => value_of(gates, lhs)? | value_of(gates, rhs)?,
    Operation::LeftShift(lhs, rhs) => value_of(gates, lhs)? << value_of(gates, rhs)?,
    Operation::RightShift(lhs, rhs) => value_of(gates, lhs)? >> value_of(gates, rhs)?,
  };
  gates.get_mut(wire)?.value = Some(wire_value);
  Some(wire_value)
}

fn create_gate<'a>(gates: &mut HashMap<&'a str, LogicalGate<'a>>, line: &'a str) {
  let words = line.split(' ').collect::<Vec<&str>>();
  let (wire, operation) = match words.len() {
    // "123 -> x"
    3 => (words[2], Operation::Assign(words[0])),
    // "NOT e -> f".
    4 => (words[3], Operation::Not(words[1])),
    5 => match words[1] {
      "AND" => (words[4], Operation::And(words[0], words[2])),
      "OR" => (words[4], Operation::Or(words[0], words[2])),
      "LSHIFT" => (words[4], Operation::LeftShift(words[0], words[2])),
      "RSHIFT" => (words[4], Operation::RightShift(words[0], words[2])),
      _ => panic!("Invalid line: {}", line),
    },
    _ => panic!("Invalid line: {}", line),
  };
  let gate = LogicalGate {
    operation,
    value: None,
  };
  gates.insert(wire, gate);
}

pub fn day_07_v1(input: impl Into<String>) -> u32 {
  let input_str = input.into();
  let mut gates: HashMap<&str, LogicalGate> = HashMap::new();
  for line in input_str.lines() {
    create_gate(&mut gates, line);
  }
  let Some(result) = value_of(&mut gates, "a") else {
    panic!("Invalid result");
  };
  return result;
}

pub fn day_07_v2(input: impl Into<String>) -> u32 {
  let input_str = input.into();
  let mut gates: HashMap<&str, LogicalGate> = HashMap::new();
  for line in input_str.lines() {
    create_gate(&mut gates, line);
  }
  let value_of_a = day_07_v1(&input_str);
  let value_a = format!("{}", value_of_a);
  let gate = LogicalGate {
    operation: Operation::Assign(value_a.as_str()),
    value: Some(value_of_a),
  };
  gates.remove("b");
  gates.insert("b", gate);
  let Some(result) = value_of(&mut gates, "a") else {
    panic!("Invalid result");
  };
  return result;
}

solvable!(day_07, day_07_v1, day_07_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "123 -> x\n\
      456 -> b\n\
      x AND g -> a\n\
      x OR b -> e\n\
      x LSHIFT 2 -> f\n\
      b RSHIFT 2 -> g\n\
      NOT x -> h\n\
      NOT b -> i";
    assert_eq!(day_07_v1(sample_one), 114);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_one = "123 -> x\n\
      456 -> b\n\
      x AND g -> a\n\
      x OR b -> e\n\
      x LSHIFT 2 -> f\n\
      b RSHIFT 2 -> g\n\
      NOT x -> h\n\
      NOT b -> i";
    assert_eq!(day_07_v2(sample_one), 24);
  }
}
