use aoc2019::intcode;

fn run(machine: &mut intcode::Machine, noun: intcode::Word, verb: intcode::Word) -> intcode::Word {
    machine.tape[1] = noun;
    machine.tape[2] = verb;

    machine.run();
    machine.tape[0]
}

fn main() -> std::io::Result<()> {
    let machine = intcode::Machine::new_from_file("../inputs/02")?;
    let part1 = run(&mut machine.clone(), 12, 2);
    println!("part 1: {}", part1);

    if let Some((noun, verb)) = (0..100)
        .flat_map(move |a| (0..100).map(move |b| (a, b)))
        .find(|(noun, verb)| run(&mut machine.clone(), *noun, *verb) == 19690720)
    {
        let part2 = 100 * noun + verb;
        println!("part 2: {}", part2);
    }

    Ok(())
}
