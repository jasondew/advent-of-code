use aoc2019::intcode;

fn main() -> std::io::Result<()> {
    let master_machine = intcode::Machine::new_from_file("../inputs/09")?;

    let machine = &mut master_machine.clone();
    machine.input.push(1);
    machine.run();

    if let Some(output) = machine.output.pop() {
        println!("part 1: {:?}", output);
    }

    let machine = &mut master_machine.clone();
    machine.input.push(2);
    machine.run();

    if let Some(output) = machine.output.pop() {
        println!("part 2: {:?}", output);
    }

    Ok(())
}
