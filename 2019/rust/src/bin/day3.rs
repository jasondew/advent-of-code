use std::fs;
use std::io;

#[derive(Debug)]
struct Vector {
    direction: String,
    magnitude: u16,
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("../inputs/03")?;
    if let Some([path_one, path_two]) = input
        .trim()
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|string| {
                    let (direction, magnitude_string) = string.split_at(1);
                    Vector {
                        direction: direction.to_string(),
                        magnitude: magnitude_string.parse().unwrap(),
                    }
                })
                .collect::<Vec<Vector>>()
        })
        .collect::<Vec<Vec<Vector>>>()
        .get(0..2)
    {
        println!("{:?}", path_one);
        println!("{:?}", path_two);

        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "bad input"))
    }
}
