use std::collections::HashSet;

type Point = (isize, isize);
type Algorithm = Vec<char>;

#[derive(Debug)]
struct Image {
    light_pixels: HashSet<Point>,
    dark_pixels: HashSet<Point>,
    min: isize,
    max: isize,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let (algorithm, mut image) = parse(input);

    for step in 0..2 {
        enhance(&mut image, &algorithm, step)
    }

    image.light_pixels.len()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (algorithm, mut image) = parse(input);

    for step in 0..50 {
        enhance(&mut image, &algorithm, step)
    }

    image.light_pixels.len()
}

fn enhance(image: &mut Image, algorithm: &Algorithm, step: usize) {
    let on_char: char;
    let source_pixels: &mut HashSet<Point>;
    let target_pixels: &mut HashSet<Point>;
    let invert: bool;

    if step % 2 == 0 {
        on_char = '.';
        invert = false;
        source_pixels = &mut image.light_pixels;
        target_pixels = &mut image.dark_pixels;
    } else {
        on_char = '#';
        invert = true;
        source_pixels = &mut image.dark_pixels;
        target_pixels = &mut image.light_pixels;
    }

    for y in image.min..=image.max {
        for x in image.min..=image.max {
            let mut binary_string = binary_string(source_pixels, x, y);
            if invert {
                binary_string = binary_string
                    .chars()
                    .map(|ch| if ch == '1' { '0' } else { '1' })
                    .collect();
            }
            let discriminant = usize::from_str_radix(&binary_string, 2).unwrap();

            if algorithm[discriminant] == on_char {
                target_pixels.insert((x, y));
            }
        }
    }

    source_pixels.clear();
    image.min -= 1;
    image.max += 1;
}

fn binary_string(pixels: &HashSet<Point>, x: isize, y: isize) -> String {
    [
        get_digit(pixels, x - 1, y - 1),
        get_digit(pixels, x, y - 1),
        get_digit(pixels, x + 1, y - 1),
        get_digit(pixels, x - 1, y),
        get_digit(pixels, x, y),
        get_digit(pixels, x + 1, y),
        get_digit(pixels, x - 1, y + 1),
        get_digit(pixels, x, y + 1),
        get_digit(pixels, x + 1, y + 1),
    ]
    .iter()
    .collect()
}

fn get_digit(pixels: &HashSet<Point>, x: isize, y: isize) -> char {
    if pixels.contains(&(x, y)) {
        '1'
    } else {
        '0'
    }
}

fn parse(input: &str) -> (Algorithm, Image) {
    let (algorithm_string, image_string) = input.split_once("\n\n").unwrap();
    let algorithm = algorithm_string.trim_end().chars().collect();
    let mut light_pixels: HashSet<Point> = HashSet::new();

    for (y, row) in image_string.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                light_pixels.insert((x as isize, y as isize));
            }
        }
    }

    (
        algorithm,
        Image {
            light_pixels,
            dark_pixels: HashSet::new(),
            min: -2,
            max: (image_string.lines().count() + 1) as isize,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
#.#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#...

#..#.
#....
##..#
..#..
..###\n"
    }

    #[test]
    fn example() {
        assert_eq!(part1(input()), 24);
        assert_eq!(part2(input()), 3352);
    }
}
