import gleam/io
import simplifile

pub fn main() -> Nil {
  let assert Ok(input) = simplifile.read("../inputs/01")
  io.println(input)
}

pub fn part1(input: String) -> Int {
  string.lines(input)
}
