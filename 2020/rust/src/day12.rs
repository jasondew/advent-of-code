#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Instruction {
    Shift(Direction, usize),
    Rotate(usize),
    Forward(usize),
}

#[derive(Debug)]
enum Version {
    One,
    Two,
}

#[derive(Debug)]
struct Ferry {
    version: Version,
    location: (isize, isize),
    bearing: usize,
    waypoint: (isize, isize),
}

impl Ferry {
    fn new(version: Version) -> Self {
        Self {
            version: version,
            location: (0, 0),
            bearing: 90,
            waypoint: (1, 10),
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            Shift(direction, value) => self.shift(direction, value),
            Rotate(value) => self.rotate(value),
            Forward(value) => self.forward(value),
        }
    }

    fn shift(&mut self, direction: &Direction, value: &usize) {
        use Direction::*;
        match self.version {
            Version::One => match direction {
                North => self.location.0 += *value as isize,
                South => self.location.0 -= *value as isize,
                East => self.location.1 += *value as isize,
                West => self.location.1 -= *value as isize,
            },
            Version::Two => match direction {
                North => self.waypoint.0 += *value as isize,
                South => self.waypoint.0 -= *value as isize,
                East => self.waypoint.1 += *value as isize,
                West => self.waypoint.1 -= *value as isize,
            },
        }
    }

    fn rotate(&mut self, value: &usize) {
        match self.version {
            Version::One => self.bearing = (self.bearing + value) % 360,
            Version::Two => match value {
                90 => {
                    let (y, x) = self.waypoint;
                    self.waypoint.0 = -1 * x;
                    self.waypoint.1 = y;
                }
                180 => {
                    self.waypoint.0 *= -1;
                    self.waypoint.1 *= -1;
                }
                270 => {
                    let (y, x) = self.waypoint;
                    self.waypoint.0 = x;
                    self.waypoint.1 = -1 * y;
                }
                _ => panic!("invalid bearing"),
            },
        }
    }

    fn forward(&mut self, value: &usize) {
        match self.version {
            Version::One => match self.bearing {
                0 => self.location.0 += *value as isize,
                90 => self.location.1 += *value as isize,
                180 => self.location.0 -= *value as isize,
                270 => self.location.1 -= *value as isize,
                _ => panic!("invalid bearing"),
            },
            Version::Two => {
                self.location.0 += self.waypoint.0 * *value as isize;
                self.location.1 += self.waypoint.1 * *value as isize;
            }
        }
    }

    fn distance(&self) -> usize {
        self.location.0.abs() as usize + self.location.1.abs() as usize
    }
}

impl std::str::FromStr for Instruction {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        use Instruction::*;
        match string.split_at(1) {
            ("N", value) => Ok(Shift(North, value.parse().unwrap())),
            ("S", value) => Ok(Shift(South, value.parse().unwrap())),
            ("E", value) => Ok(Shift(East, value.parse().unwrap())),
            ("W", value) => Ok(Shift(West, value.parse().unwrap())),
            ("L", value) => Ok(Rotate(
                360usize.checked_sub(value.parse().unwrap()).unwrap(),
            )),
            ("R", value) => Ok(Rotate(value.parse().unwrap())),
            ("F", value) => Ok(Forward(value.parse().unwrap())),
            _ => Err(format!("invalid instruction: {:?}", string)),
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut ferry = Ferry::new(Version::One);

    input
        .lines()
        .for_each(|line| ferry.execute(&line.parse().unwrap()));

    ferry.distance()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut ferry = Ferry::new(Version::Two);

    input
        .lines()
        .for_each(|line| ferry.execute(&line.parse().unwrap()));

    ferry.distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("F10\nN3\nF7\nR90\nF11\n"), 25)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("F10\nN3\nF7\nR90\nF11\n"), 286)
    }
}
