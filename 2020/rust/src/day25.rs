const MODULO: usize = 20201227;

#[must_use]
pub fn part1(input: &str) -> usize {
    let public_keys: Vec<usize> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    let loop_size: usize = find_loop_size(public_keys[0]);

    transform(public_keys[1], loop_size)
}

fn find_loop_size(public_key: usize) -> usize {
    let mut loop_size: usize = 0;
    let mut value: usize = 1;
    let subject_number: usize = 7;

    loop {
        if value == public_key {
            break;
        }

        value = transform_step(value, subject_number);
        loop_size += 1;
    }

    loop_size
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    (0..loop_size).fold(1, |acc, _iteration| transform_step(acc, subject_number))
}

fn transform_step(value: usize, subject_number: usize) -> usize {
    value * subject_number % MODULO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_example() {
        assert_eq!(transform(7, 8), 5764801);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("5764801\n17807724\n"), 14897079);
    }
}
