fn op<F>(tape: &mut Vec<i32>, ip: usize, f: F)
where
    F: Fn(i32, i32) -> i32,
{
    let a = tape[tape[ip + 1] as usize];
    let b = tape[tape[ip + 2] as usize];
    let to = tape[ip + 3] as usize;

    tape[to] = f(a, b);
}

fn run(original_tape: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut tape = original_tape.clone();
    let mut ip: usize = 0;

    tape[1] = noun;
    tape[2] = verb;

    loop {
        match tape[ip] {
            1 => op(&mut tape, ip, |a, b| a + b),
            2 => op(&mut tape, ip, |a, b| a * b),
            99 => break tape[0],
            _ => panic!("got invalid instruction: {:?}", tape[ip]),
        }
        ip += 4
    }
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
