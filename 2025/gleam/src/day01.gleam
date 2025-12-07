import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

pub fn main() -> Nil {
  let assert Ok(input) = simplifile.read("../inputs/01")
  io.println("Part 1: " <> int.to_string(part1(input)))
  io.println("Part 2: " <> int.to_string(part2(input)))
}

pub fn part1(input: String) -> Int {
  let lines =
    input
    |> string.trim()
    |> string.split("\n")
    |> list.filter(fn(line) { line != "" })

  list.fold(lines, #(50, 0), fn(acc, raw_line) {
    let #(dial, zero_count) = acc
    let line = string.trim(raw_line)
    let direction = string.slice(line, 0, 1)
    let assert Ok(amount) =
      int.parse(string.slice(line, 1, string.length(line)))

    let updated_zero_count = case dial {
      0 -> zero_count + 1
      _ -> zero_count
    }
    let updated_dial = case direction {
      "L" -> dial + 100 - amount % 100
      _ -> dial + amount
    }

    #(updated_dial % 100, updated_zero_count)
  }).1
}

pub fn part2(input: String) -> Int {
  let lines =
    input
    |> string.trim()
    |> string.split("\n")
    |> list.filter(fn(line) { line != "" })

  list.fold(lines, #(50, 0), fn(acc, raw_line) {
    let #(dial, zero_count) = acc
    let line = string.trim(raw_line)
    let direction = string.slice(line, 0, 1)
    let assert Ok(amount) =
      int.parse(string.slice(line, 1, string.length(line)))

    let assert Ok(additional_zero_count) = case direction {
      "L" -> {
        case dial {
          0 -> int.divide(amount, 100)
          _ -> int.divide(100 - dial + amount, 100)
        }
      }
      _ -> int.divide(dial + amount, 100)
    }
    let updated_dial = case direction {
      "L" -> dial + 100 - amount % 100
      _ -> dial + amount
    }

    #(updated_dial % 100, additional_zero_count + zero_count)
  }).1
}
