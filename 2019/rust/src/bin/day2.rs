mod intcode;

fn run(original_intcode: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut intcode = original_intcode.clone();

    intcode[1] = noun;
    intcode[2] = verb;

    intcode::run(&mut intcode, &mut vec![]).intcode[0]
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("../inputs/02")?;
    let tape: Vec<i32> = input
        .trim()
        .split(",")
        .map(|str| str.parse().unwrap())
        .collect();

    let part1 = run(&tape, 12, 2);
    println!("part 1: {}", part1);

    if let Some((noun, verb)) = (0..100)
        .flat_map(move |a| (0..100).map(move |b| (a, b)))
        .find(|(noun, verb)| run(&tape, *noun, *verb) == 19690720)
    {
        let part2 = 100 * noun + verb;
        println!("part 2: {}", part2);
    }

    Ok(())
}
