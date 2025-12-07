import unittest


def part1(puzzle_input: str) -> int:
    zero_count = 0
    dial = 50

    for raw_line in puzzle_input.strip().splitlines():
        line = raw_line.strip()
        direction = line[0]
        amount = int(line[1:])

        if direction == "L":
            dial -= amount
        else:
            dial += amount

        dial %= 100

        if dial == 0:
            zero_count += 1

    return zero_count


def part2(puzzle_input: str) -> int:
    zero_count = 0
    dial = 50

    for raw_line in puzzle_input.strip().splitlines():
        line = raw_line.strip()
        direction = line[0]
        amount = int(line[1:])

        if direction == "L":
            if dial == 0:
                zero_count += amount // 100
            else:
                zero_count += ((100 - dial) + amount) // 100
            dial += 100 - (amount % 100)
        else:
            zero_count += (dial + amount) // 100
            dial += amount % 100

        dial %= 100

    return zero_count


def run() -> None:
    with open("../inputs/01", "r") as file:
        puzzle_input = file.read().strip()

        print(f"Part 1: {part1(puzzle_input)}")
        print(f"Part 2: {part2(puzzle_input)}")


class Tests(unittest.TestCase):
    EXAMPLE_INPUT = """
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
    """

    def test_part1_example(self):
        self.assertEqual(part1(self.EXAMPLE_INPUT), 3)

    def test_part2_example(self):
        self.assertEqual(part2(self.EXAMPLE_INPUT), 6)


if __name__ == "__main__":
    import sys

    if "test" in sys.argv:
        sys.argv.remove("test")
        unittest.main(argv=sys.argv, exit=False)
    else:
        run()
