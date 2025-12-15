use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_line_segment_mut};
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

#[derive(PartialEq, Eq, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({:2}, {:2})", self.x, self.y)
    }
}

type Grid = Vec<Vec<Option<Tile>>>;

#[derive(Clone, PartialEq)]
enum Tile {
    Red,
    Green,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let tiles = parse(input);
    let mut max_area = 0usize;

    for tile in &tiles {
        let other_tiles = tiles.iter().filter(|&other| {
            tile.x != other.x && tile.y != other.y && other.y > tile.y
        });

        for other_tile in other_tiles {
            let width =
                (tile.x as i128 - other_tile.x as i128).unsigned_abs() + 1;
            let height =
                (tile.y as i128 - other_tile.y as i128).unsigned_abs() + 1;
            let area = (width * height) as usize;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let tiles = parse(input);
    let (mut grid, x_mapping, y_mapping) = compress(&tiles);
    add_edges(&mut grid, &tiles, &x_mapping, &y_mapping);
    flood_fill(&mut grid);

    let mut max_area = 0usize;

    for tile in &tiles {
        let grid_position = (
            *x_mapping.get(&tile.x).unwrap(),
            *y_mapping.get(&tile.y).unwrap(),
        );
        let other_tiles = tiles.iter().filter(|&other| {
            tile.x != other.x && tile.y != other.y && other.y > tile.y
        });

        for other_tile in other_tiles {
            let other_grid_position = (
                *x_mapping.get(&other_tile.x).unwrap(),
                *y_mapping.get(&other_tile.y).unwrap(),
            );
            if valid_rectangle(&grid, grid_position, other_grid_position) {
                let area = calculate_area(tile, other_tile);

                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

#[allow(clippy::needless_range_loop)]
fn valid_rectangle(
    grid: &[Vec<Option<Tile>>],
    (top_x, top_y): (usize, usize),
    (bottom_x, bottom_y): (usize, usize),
) -> bool {
    let x_range = if top_x < bottom_x {
        top_x..=bottom_x
    } else {
        bottom_x..=top_x
    };

    for x in x_range {
        for y in top_y..=bottom_y {
            if grid[y][x].is_none() {
                return false;
            }
        }
    }

    true
}

fn compress(
    tiles: &Vec<Position>,
) -> (Grid, HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut x_values: Vec<usize> = tiles.iter().map(|p| p.x).collect();
    x_values.sort_unstable();
    x_values.dedup();

    let mut y_values: Vec<usize> = tiles.iter().map(|p| p.y).collect();
    y_values.sort_unstable();
    y_values.dedup();

    let x_mapping: HashMap<usize, usize> =
        x_values.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_mapping: HashMap<usize, usize> =
        y_values.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    let mut grid = vec![vec![None; x_values.len()]; y_values.len()];

    for tile in tiles {
        grid[*y_mapping.get(&tile.y).unwrap()]
            [*x_mapping.get(&tile.x).unwrap()] = Some(Tile::Red);
    }

    (grid, x_mapping, y_mapping)
}

#[allow(clippy::needless_range_loop)]
fn add_edges(
    grid: &mut Grid,
    tiles: &[Position],
    x_mapping: &HashMap<usize, usize>,
    y_mapping: &HashMap<usize, usize>,
) {
    let mut rotated_tiles = tiles.to_vec();
    rotated_tiles.rotate_left(1);
    for (tile_a, tile_b) in tiles.iter().zip(rotated_tiles.iter()) {
        let xa = *x_mapping.get(&tile_a.x).unwrap();
        let ya = *y_mapping.get(&tile_a.y).unwrap();
        let xb = *x_mapping.get(&tile_b.x).unwrap();
        let yb = *y_mapping.get(&tile_b.y).unwrap();

        if xa == xb {
            let (start_y, end_y) = if ya < yb { (ya, yb) } else { (yb, ya) };
            for y in start_y + 1..end_y {
                grid[y][xa] = Some(Tile::Green);
            }
        } else if ya == yb {
            let (start_x, end_x) = if xa < xb { (xa, xb) } else { (xb, xa) };
            for x in start_x + 1..end_x {
                grid[ya][x] = Some(Tile::Green);
            }
        }
    }
}

fn flood_fill(grid: &mut Grid) {
    let mut start_point = Position { x: 0, y: 0 };
    let mut inside = false;
    for (x, tile) in grid[1].iter().enumerate() {
        if inside {
            if tile.is_none() {
                start_point = Position { x, y: 1 };
                break;
            }
        } else if tile.is_some() {
            inside = true;
        }
    }

    let mut stack = vec![start_point];
    while let Some(pos) = stack.pop() {
        if grid[pos.y][pos.x].is_none() {
            grid[pos.y][pos.x] = Some(Tile::Green);

            if pos.x > 0 {
                stack.push(Position {
                    x: pos.x - 1,
                    y: pos.y,
                });
            }
            if pos.x + 1 < grid[0].len() {
                stack.push(Position {
                    x: pos.x + 1,
                    y: pos.y,
                });
            }
            if pos.y > 0 {
                stack.push(Position {
                    x: pos.x,
                    y: pos.y - 1,
                });
            }
            if pos.y + 1 < grid.len() {
                stack.push(Position {
                    x: pos.x,
                    y: pos.y + 1,
                });
            }
        }
    }
}

fn calculate_area(top: &Position, bottom: &Position) -> usize {
    let width = (top.x as i128 - bottom.x as i128).unsigned_abs() + 1;
    let height = (top.y as i128 - bottom.y as i128).unsigned_abs() + 1;

    (width * height) as usize
}

#[allow(dead_code)]
fn print(tiles: &[Position]) {
    let max_x = tiles.iter().map(|p| p.x).max().unwrap() + 2;
    let max_y = tiles.iter().map(|p| p.y).max().unwrap() + 1;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if tiles.contains(&Position { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

/// Draw tiles as a PNG with red pixels at each tile position and green lines connecting them.
/// The image is automatically scaled to fit within 10000x10000 pixels.
///
/// # Panics
/// Panics if the image cannot be saved to the specified path.
#[allow(dead_code)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn draw_tiles_png(tiles: &[Position], filename: &str) {
    if tiles.is_empty() {
        return;
    }

    let max_x = tiles.iter().map(|p| p.x).max().unwrap();
    let max_y = tiles.iter().map(|p| p.y).max().unwrap();

    // Scale down to fit within 10000x10000 pixels
    let max_dimension = 10000.0;
    let scale = (max_dimension / max_x.max(max_y) as f64).min(1.0);

    let width = ((max_x + 1) as f64 * scale).ceil() as u32;
    let height = ((max_y + 1) as f64 * scale).ceil() as u32;

    println!("Creating {width}x{height} image (scale: {scale:.6})");

    let mut img: RgbImage = ImageBuffer::new(width, height);

    // Fill with white background
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    let red = Rgb([255, 0, 0]);
    let green = Rgb([0, 255, 0]);

    // Draw thick green lines between consecutive tiles
    let line_thickness = (scale * 16.0).max(8.0) as i32;
    for i in 0..tiles.len().saturating_sub(1) {
        let start = &tiles[i];
        let end = &tiles[i + 1];

        let x1 = (start.x as f64 * scale) as f32;
        let y1 = (start.y as f64 * scale) as f32;
        let x2 = (end.x as f64 * scale) as f32;
        let y2 = (end.y as f64 * scale) as f32;

        // Draw multiple parallel lines to create thickness
        for offset in -(line_thickness / 2)..=(line_thickness / 2) {
            let offset_f = offset as f32;
            draw_line_segment_mut(
                &mut img,
                (x1 + offset_f, y1),
                (x2 + offset_f, y2),
                green,
            );
            draw_line_segment_mut(
                &mut img,
                (x1, y1 + offset_f),
                (x2, y2 + offset_f),
                green,
            );
        }
    }

    // Draw thick red circles at each tile position
    let radius = (scale * 16.0).max(12.0) as i32;
    for tile in tiles {
        let x = (tile.x as f64 * scale) as i32;
        let y = (tile.y as f64 * scale) as i32;
        draw_filled_circle_mut(&mut img, (x, y), radius, red);
    }

    img.save(filename).unwrap();
}

fn parse(input: &str) -> Vec<Position> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (x_string, y_string) = line.split_once(',').unwrap();

            Position {
                x: x_string.parse().unwrap(),
                y: y_string.parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
    ";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(EXAMPLE_INPUT), 50);
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(EXAMPLE_INPUT), 24);
    }
}
