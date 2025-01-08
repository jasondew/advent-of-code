use std::collections::HashMap;
use std::ops::RangeInclusive;

type Disk = Vec<Option<usize>>;
type DiskMap = HashMap<usize, (Option<usize>, usize)>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (_disk_map, mut disk): (DiskMap, Disk) = parse(input);
    let mut free_index: usize = 0;
    let mut used_index: usize = disk.len() - 1;

    //    print_disk(&disk);

    while free_index < used_index {
        while disk[free_index].is_some() {
            free_index += 1;
        }
        //        println!(
        //            "disk[{}] := {:?} (from index {})",
        //            free_index, disk[used_index], used_index
        //        );

        disk[free_index] = disk[used_index];
        disk[used_index] = None;
        free_index += 1;
        while disk[used_index].is_none() {
            used_index -= 1;
        }
    }

    //    print_disk(&disk);

    disk.iter()
        .enumerate()
        .map(|(index, block)| match block {
            Some(id) => index * id,
            None => 0,
        })
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (disk_map, _disk): (DiskMap, Disk) = parse(input);
    let mut free_list: Vec<Option<RangeInclusive<usize>>> = Vec::new();
    let mut used_list: Vec<(usize, RangeInclusive<usize>)> = Vec::new();

    let mut vec: Vec<(usize, (Option<usize>, usize))> =
        disk_map.into_iter().collect();
    vec.sort_by_key(|(index, _block)| *index);

    for (index, block) in vec {
        match block {
            (Some(id), size) => used_list.push((id, index..=index + size - 1)),
            (None, size) => free_list.push(Some(index..=index + size - 1)),
        }
    }

    for (_id, used_range) in used_list.iter_mut().rev() {
        if let Some(Some(free_range)) = free_list.iter_mut().find(|range| {
            range.as_ref().map_or(0, |r| range_size(r))
                >= range_size(used_range)
        }) {
            if free_range.start() < used_range.start() {
                let new_start = free_range.start() + range_size(used_range);

                *used_range = *free_range.start()..=(new_start - 1);
                *free_range = new_start..=*free_range.end();
            }
        }
    }

    let mut uv: Vec<(usize, RangeInclusive<usize>)> =
        used_list.into_iter().collect();
    uv.sort_by_key(|(_id, range)| *range.start());
    let mut index: usize = 0;
    let mut total: usize = 0;

    for (id, range) in uv {
        index += range.start() - index;
        for offset in 0..range_size(&range) {
            total += (index + offset) * id;
        }
        index = *range.end() + 1;
    }

    total
}

fn range_size(range: &RangeInclusive<usize>) -> usize {
    if range.end() >= range.start() {
        range.end() - range.start() + 1
    } else {
        0
    }
}

#[allow(dead_code)]
fn print_disk(disk: &Disk) {
    println!(
        "{}",
        &disk
            .iter()
            .map(|block| match block {
                Some(id) => format!("{id}"),
                None => ".".to_owned(),
            })
            .collect::<String>()
    );
}

fn parse(input: &str) -> (DiskMap, Disk) {
    let mut disk_map = HashMap::new();
    let mut id: usize = 0;
    let mut index: usize = 0;
    let mut expecting_block: bool = true;

    for char in input.trim().chars() {
        let size: usize = parse_usize(char);

        if expecting_block {
            disk_map.insert(index, (Some(id), size));
            id += 1;
        } else if size > 0 {
            disk_map.insert(index, (None, size));
        }

        index += size;
        expecting_block = !expecting_block;
    }

    let mut output: Disk = Vec::new();
    let mut vec: Vec<(&usize, &(Option<usize>, usize))> =
        disk_map.iter().collect();
    vec.sort_by_key(|(index, _block)| **index);

    for (_index, block) in &vec {
        for _ in 0..block.1 {
            match block.0 {
                Some(id) => output.push(Some(id)),
                None => output.push(None),
            }
        }
    }

    (disk_map, output)
}

fn parse_usize(ch: char) -> usize {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        non_digit_char => {
            panic!("non-digit character seen: {non_digit_char:?}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "2333133121414131402"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 1928)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 2858)
    }
}
