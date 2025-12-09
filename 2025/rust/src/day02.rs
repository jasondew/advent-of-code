use std::collections::HashSet;

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut total = 0;

    for range in input.split(',') {
        let (start, end) = range.trim().split_once('-').unwrap();
        for repeat in find_single_repeats(start, end) {
            total += repeat.parse::<usize>().unwrap();
        }
    }

    total
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut total = 0;

    for range in input.split(',') {
        let (start, end) = range.trim().split_once('-').unwrap();
        for repeat in find_all_repeats(start, end) {
            total += repeat.parse::<usize>().unwrap();
        }
    }

    total
}

fn find_single_repeats(start: &str, end: &str) -> Vec<String> {
    let mut repeats = Vec::<String>::new();
    let size = start.len();
    if start.len() == end.len() {
        if size > 1 {
            let start_value: usize = start.parse().unwrap();
            let end_value: usize = end.parse().unwrap();

            let (left_start, _right_start) = start.split_at(size / 2);
            let (left_end, _right_end) = end.split_at(size / 2);

            let left_start_value: usize = left_start.parse().unwrap();
            let left_end_value: usize = left_end.parse().unwrap();

            for left_value in left_start_value..=left_end_value {
                let repeat = left_value.to_string().repeat(2);
                let repeat_value: usize = repeat.parse().unwrap();

                if repeat_value >= start_value && repeat_value <= end_value {
                    repeats.push(repeat);
                }
            }
        }
    } else {
        assert!(end.len() - start.len() == 1);
        let left_end = "9".repeat(size);
        let right_start = format!("1{}", "0".repeat(size));

        repeats = find_single_repeats(start, &left_end);
        repeats.append(&mut find_single_repeats(&right_start, end));
    }

    repeats
}

fn find_all_repeats(start: &str, end: &str) -> Vec<String> {
    let mut repeats = HashSet::<String>::new();
    let size = start.len();

    if start.len() == end.len() {
        if size > 1 {
            let start_value: usize = start.parse().unwrap();
            let end_value: usize = end.parse().unwrap();

            for repeat_size in 1..=(size / 2) {
                let left_start = &start[0..repeat_size];
                let left_end = &end[0..repeat_size];

                let left_start_value: usize = left_start.parse().unwrap();
                let left_end_value: usize = left_end.parse().unwrap();

                for left_value in left_start_value..=left_end_value {
                    for repeat_count in 1..=(size / left_start.len()) {
                        let repeat =
                            left_value.to_string().repeat(repeat_count);
                        let repeat_value: usize = repeat.parse().unwrap();

                        if repeat_value >= start_value
                            && repeat_value <= end_value
                        {
                            repeats.insert(repeat);
                        }
                    }
                }
            }
        }
    } else {
        assert!(end.len() - start.len() == 1);
        let left_end = "9".repeat(size);
        let right_start = format!("1{}", "0".repeat(size));

        repeats = find_all_repeats(start, &left_end).into_iter().collect();
        for repeat in find_all_repeats(&right_start, end) {
            repeats.insert(repeat);
        }
    }

    let mut sorted_repeats: Vec<String> = repeats.iter().cloned().collect();
    sorted_repeats.sort_unstable();
    sorted_repeats
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_same_elements {
        ($left:expr, $right:expr) => {{
            let mut left = $left;
            let mut right = $right;
            left.sort();
            right.sort();
            assert_eq!(left, right);
        }};
    }

    const EXAMPLE_INPUT: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
    ";

    #[test]
    fn test_find_single_repeats() {
        assert_eq!(find_single_repeats("1", "9"), Vec::<String>::new());
        assert_eq!(find_single_repeats("11", "22"), vec!["11", "22"]);
        assert_eq!(find_single_repeats("95", "115"), vec!["99"]);
    }

    #[test]
    fn test_find_all_repeats() {
        assert_same_elements!(find_all_repeats("1", "9"), Vec::<String>::new());
        assert_same_elements!(find_all_repeats("11", "22"), vec!["11", "22"]);
        assert_same_elements!(find_all_repeats("95", "115"), vec!["99", "111"]);
        assert_same_elements!(
            find_all_repeats("998", "1012"),
            vec!["999", "1010"]
        );
        assert_same_elements!(
            find_all_repeats("1188511880", "1188511890"),
            vec!["1188511885"]
        );
        assert_same_elements!(
            find_all_repeats("222220", "222224"),
            vec!["222222"]
        );
        assert_same_elements!(
            find_all_repeats("1698522", "1698528"),
            Vec::<String>::new()
        );
        assert_same_elements!(
            find_all_repeats("446443", "446449"),
            vec!["446446"]
        );
        assert_same_elements!(
            find_all_repeats("38593856", "38593862"),
            vec!["38593859"]
        );
        assert_same_elements!(
            find_all_repeats("565653", "565659"),
            vec!["565656"]
        );
        assert_same_elements!(
            find_all_repeats("824824821", "824824827"),
            vec!["824824824"]
        );
        assert_same_elements!(
            find_all_repeats("2121212118", "2121212124"),
            vec!["2121212121"]
        );
    }

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(EXAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(EXAMPLE_INPUT), 4174379265);
    }
}
