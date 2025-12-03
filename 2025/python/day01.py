import unittest


def part1(input: str) -> int:
    zero_count = 0
    dial = 50

    for line in input.strip().split("\n"):
        line = line.strip()
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

def part2(input: str) -> int:
    zero_count = 0
    dial = 50

    for line in input.strip().split("\n"):
        line = line.strip()
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
            dial += (amount % 100)

        dial %= 100

    return zero_count

def run():
    input = open("../inputs/01", "r").read().strip()

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")


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
