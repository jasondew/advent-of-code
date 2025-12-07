import gleeunit
import day01

const example_input = "
  L68
  L30
  R48
  L5
  R60
  L55
  L1
  L99
  R14
  L82
"

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn part1_test() {
  assert day01.part1(example_input) == 3
}

pub fn part2_test() {
  assert day01.part2(example_input) == 6
}
