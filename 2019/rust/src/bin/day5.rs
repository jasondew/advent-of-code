use aoc2019::intcode;

fn main() -> std::io::Result<()> {
    let machine = &mut intcode::Machine::new_from_file("../inputs/05")?;
    machine.input.push(1);
    machine.run();

    if let Some(output) = machine.output.pop() {
        println!("part 1: {:?}", output);
    }

    let machine = &mut intcode::Machine::new_from_file("../inputs/05")?;
    machine.input.push(5);
    machine.run();

    if let Some(output) = machine.output.pop() {
        println!("part 2: {:?}", output);
    }

    Ok(())
}
