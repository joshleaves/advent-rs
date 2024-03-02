use assert_cmd::Command;

#[test]
fn invalid_year_past() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("advent-rs")?;
  cmd.arg("--year").arg("2014");
  cmd.arg("--day").arg("01");
  cmd.assert().failure().stderr(predicates::str::contains(
    "advent-rs: Not implemented (Year 2014 Day 01v1)",
  ));

  Ok(())
}

#[test]
fn calls_a_proper_test() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("advent-rs")?;
  cmd.arg("--year").arg("2015");
  cmd.arg("--day").arg("01");
  cmd.arg("inputs/year_2015_day_01_input");
  cmd
    .assert()
    .success()
    .stdout(predicates::str::contains("Result: 138"));

  Ok(())
}
