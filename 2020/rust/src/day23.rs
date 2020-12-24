use std::collections::HashMap;
use std::str::FromStr;

type Cup = u32;

#[derive(Debug)]
struct Cups {
    data: Vec<Cup>,
    map: HashMap<Cup, usize>,
    current_index: usize,
    current_move: usize,
}

impl FromStr for Cups {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut data: Vec<Cup> = Vec::with_capacity(1_000_000);
        let mut map: HashMap<Cup, usize> = HashMap::with_capacity(1_000_000);

        for (index, ch) in input.trim().chars().enumerate() {
            let cup = ch.to_digit(10).unwrap() as Cup;
            data.push(cup);
            map.insert(cup, index);
        }

        Ok(Cups {
            data,
            map,
            current_index: 0,
            current_move: 0,
        })
    }
}

impl Cups {
    fn stepn(&mut self, count: usize) {
        for _index in 0..count {
            self.step();
        }
    }

    fn step(&mut self) {
        self.current_move += 1;

        let moving_cups: Vec<Cup> = self.cup_range(self.current_index + 1, 3);
        let destination_cup: Cup = self.final_destination(&moving_cups);
        let next_cup: Cup = self.cup_at(self.current_index + 4);

        //        self.print_step(&moving_cups, destination_cup);
        self.move_cups(&moving_cups, destination_cup);

        self.current_index = self.cup_index(next_cup);
    }

    fn final_destination(&self, moving_cups: &[Cup]) -> Cup {
        let mut destination_cup: Cup = self.previous_cup(self.cup_at(self.current_index));

        loop {
            if moving_cups.contains(&destination_cup) {
                destination_cup = self.previous_cup(destination_cup)
            } else {
                break;
            }
        }

        destination_cup
    }

    fn previous_cup(&self, cup: Cup) -> Cup {
        let mut previous_cup: Cup = cup - 1;

        if previous_cup <= 0 {
            previous_cup += self.data.len() as Cup;
        }

        previous_cup
    }

    fn cup_index(&self, target_cup: Cup) -> usize {
        self.map[&target_cup]
    }

    fn cup_at(&self, index: usize) -> Cup {
        let mut wrapped_index = index;

        if wrapped_index >= self.data.len() {
            wrapped_index -= self.data.len();
        }

        self.data[wrapped_index]
    }

    fn cup_range(&self, from: usize, length: usize) -> Vec<Cup> {
        (0..length)
            .map(|offset| self.cup_at(from + offset))
            .collect()
    }

    //          Case 1          or          Case 2
    //    1 2 (3) 4 5 6 7 8 9         1 2 (3) 4 5 6 7 8 9
    //      ^ <-- ^ ^ ^                       ^ ^ ^ --> ^
    //      |=========|                       |=========|
    // from 1  2  3 4 5                       3 4 5 6 7 8
    //  to  4  5  1 2 3                       6 7 8 3 4 5
    fn move_cups(&mut self, cups: &[Cup], destination_cup: Cup) {
        if self.cup_index(cups[0]) + 2 >= self.data.len() {
            self.shift(self.cup_index(cups[0]));
            //            dbg!(&self.data);
            //            dbg!(&self.map);
        }

        let group_start_index = self.cup_index(cups[0]);
        let group_end_index = group_start_index + 2;
        let destination_index = self.cup_index(destination_cup) + 1;

        let affected_cups: Vec<(Cup, usize)> = if destination_index < group_end_index {
            //            println!(
            //                "CASE 1: indexes {}..{} to index {}",
            //                group_start_index, group_end_index, destination_index
            //            );
            // case 1

            let x = (group_start_index..(group_end_index + 1))
                .chain(destination_index..group_start_index)
                .enumerate()
                .map(|(offset, index)| (self.cup_at(index), destination_index + offset))
                .collect();
            //            dbg!(&x);
            x
        } else {
            //            println!(
            //                "CASE 2: indexes {}..{} to index {}",
            //                group_start_index, group_end_index, destination_index
            //            );
            // case 2
            (group_start_index..destination_index)
                .enumerate()
                .map(|(offset, index)| {
                    let new_index = if index <= group_end_index {
                        let x = destination_index + offset - 3;
                        //                        println!(
                        //                            "[{}] (index <= group_end_index) index {} => {}",
                        //                            offset, index, x
                        //                        );
                        x
                    } else {
                        let x = group_start_index + (index - group_end_index - 1);
                        //                        println!(
                        //                            "[{}] (index > group_end_index) index {} => {}",
                        //                            offset, index, x
                        //                        );
                        x
                    };
                    (self.cup_at(index), new_index)
                })
                .collect()
        };

        //        println!("data={:?}", &self.data);
        //        println!("> move {:?} to after {:?}", &cups, destination_cup);
        for (cup, new_index) in affected_cups {
            let wrapped_index = new_index % self.data.len();
            self.data[wrapped_index] = cup;
            self.map.insert(cup, wrapped_index);
        }
        //        println!("data={:?}\n", &self.data);
    }

    fn shift(&mut self, offset: usize) {
        //        println!("SHIFTING!");
        let mut new_data: Vec<Cup> = Vec::with_capacity(self.data.len());
        let mut new_map: HashMap<Cup, usize> = HashMap::with_capacity(self.map.len());

        for &cup in self.data.iter().skip(offset) {
            new_data.push(cup);
        }

        for &cup in self.data.iter().take(offset) {
            new_data.push(cup);
        }

        for (index, &cup) in new_data.iter().enumerate() {
            new_map.insert(cup, index);
        }

        self.data = new_data;
        self.map = new_map;
    }

    fn add_cups_up_to(&mut self, n: Cup) {
        for cup in (*self.data.iter().max().unwrap() + 1)..n {
            self.data.push(cup);
            self.map.insert(cup, cup as usize);
        }
    }

    fn output(&mut self) -> String {
        let one_index: usize = self.cup_index(1);

        (1..9)
            .map(|index| self.data[(one_index + index) % self.data.len()] as u8)
            .map(|cup| (48 + cup) as char)
            .collect()
    }

    #[allow(dead_code)]
    fn print_step(&self, moving_cups: &[Cup], destination_cup: Cup) {
        println!("-- move {} --", self.current_move);
        print!("cups: ");
        for (index, cup) in self.data.iter().enumerate() {
            if index == self.current_index {
                print!("({}) ", cup);
            } else {
                print!("{} ", cup);
            }
        }
        println!();
        println!("pick up: {:?}", moving_cups);
        println!("destination: {}\n", destination_cup);
    }
}

#[must_use]
pub fn part1(input: &str) -> String {
    let mut cups: Cups = input.parse().unwrap();

    cups.stepn(100);

    cups.output()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut cups: Cups = input.parse().unwrap();

    cups.add_cups_up_to(1_000_000);

    cups.stepn(10_000_000);

    cups.cup_range(cups.cup_index(1), 2)
        .iter()
        .map(|cup| *cup as usize)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_short_example() {
        let mut cups: Cups = "389125467\n".parse().unwrap();

        cups.stepn(10);

        assert_eq!(cups.output(), "92658374");
    }

    #[test]
    fn part1_full_example() {
        assert_eq!(part1("389125467\n"), "67384529");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("389125467\n"), 149245887792)
    }
}
