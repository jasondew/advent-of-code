#[must_use]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let timestamp: usize = lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&id| id != "x")
        .map(|id| id.parse().unwrap())
        .collect();

    let bus_ids_with_wait_time: Vec<(usize, usize)> = bus_ids
        .iter()
        .map(|&bus_id| (bus_id, bus_id - (timestamp % bus_id)))
        .collect();

    let (bus_id, min_wait_time) = bus_ids_with_wait_time
        .iter()
        .min_by_key(|(_bus_id, wait_time)| wait_time)
        .unwrap();

    bus_id * min_wait_time
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let bus_ids: Vec<(usize, usize)> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_index, id)| id != &"x")
        .map(|(index, id)| (index, id.parse().unwrap()))
        .collect();

    let (x, n) = solve_congruence(&bus_ids);

    n - x
}

fn solve_congruence(equations: &[(usize, usize)]) -> (usize, usize) {
    let cap_n: usize = equations.iter().map(|&(_a, n)| n).product();
    let x: usize = equations
        .iter()
        .map(|&(a, n)| {
            let y: usize = cap_n / n;
            let z: usize = mod_pow(y, phi(n) - 1, n);
            a * y * z
        })
        .sum();

    (x % cap_n, cap_n)
}

fn mod_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    let mut result = 1;

    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }

    result
}

fn phi(n: usize) -> usize {
    let x: Vec<usize> = vec![
        1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20, 12, 18,
        12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22, 46, 16, 42, 20,
        32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44, 24, 70, 24, 72,
        36, 40, 36, 60, 24, 78, 32, 54, 40, 82, 24, 64, 42, 56, 40, 88, 24, 72, 44, 60, 46, 72, 32,
        96, 42, 60, 40, 100, 32, 102, 48, 48, 52, 106, 36, 108, 40, 72, 48, 112, 36, 88, 56, 72,
        58, 96, 32, 110, 60, 80, 60, 100, 36, 126, 64, 84, 48, 130, 40, 108, 66, 72, 64, 136, 44,
        138, 48, 92, 70, 120, 48, 112, 72, 84, 72, 148, 40, 150, 72, 96, 60, 120, 48, 156, 78, 104,
        64, 132, 54, 162, 80, 80, 82, 166, 48, 156, 64, 108, 84, 172, 56, 120, 80, 116, 88, 178,
        48, 180, 72, 120, 88, 144, 60, 160, 92, 108, 72, 190, 64, 192, 96, 96, 84, 196, 60, 198,
        80, 132, 100, 168, 64, 160, 102, 132, 96, 180, 48, 210, 104, 140, 106, 168, 72, 180, 108,
        144, 80, 192, 72, 222, 96, 120, 112, 226, 72, 228, 88, 120, 112, 232, 72, 184, 116, 156,
        96, 238, 64, 240, 110, 162, 120, 168, 80, 216, 120, 164, 100, 250, 72, 220, 126, 128, 128,
        256, 84, 216, 96, 168, 130, 262, 80, 208, 108, 176, 132, 268, 72, 270, 128, 144, 136, 200,
        88, 276, 138, 180, 96, 280, 92, 282, 140, 144, 120, 240, 96, 272, 112, 192, 144, 292, 84,
        232, 144, 180, 148, 264, 80, 252, 150, 200, 144, 240, 96, 306, 120, 204, 120, 310, 96, 312,
        156, 144, 156, 316, 104, 280, 128, 212, 132, 288, 108, 240, 162, 216, 160, 276, 80, 330,
        164, 216, 166, 264, 96, 336, 156, 224, 128, 300, 108, 294, 168, 176, 172, 346, 112, 348,
        120, 216, 160, 352, 116, 280, 176, 192, 178, 358, 96, 342, 180, 220, 144, 288, 120, 366,
        176, 240, 144, 312, 120, 372, 160, 200, 184, 336, 108, 378, 144, 252, 190, 382, 128, 240,
        192, 252, 192, 388, 96, 352, 168, 260, 196, 312, 120, 396, 198, 216, 160, 400, 132, 360,
        200, 216, 168, 360, 128, 408, 160, 272, 204, 348, 132, 328, 192, 276, 180, 418, 96, 420,
        210, 276, 208, 320, 140, 360, 212, 240, 168, 430, 144, 432, 180, 224, 216, 396, 144, 438,
        160, 252, 192, 442, 144, 352, 222, 296, 192, 448, 120, 400, 224, 300, 226, 288, 144, 456,
        228, 288, 176, 460, 120, 462, 224, 240, 232, 466, 144, 396, 184, 312, 232, 420, 156, 360,
        192, 312, 238, 478, 128, 432, 240, 264, 220, 384, 162, 486, 240, 324, 168, 490, 160, 448,
        216, 240, 240, 420, 164, 498, 200, 332, 250, 502, 144, 400, 220, 312, 252, 508, 128, 432,
        256, 324, 256, 408, 168, 460, 216, 344, 192, 520, 168, 522, 260, 240, 262, 480, 160, 506,
        208, 348, 216, 480, 176, 424, 264, 356, 268, 420, 144, 540, 270, 360, 256, 432, 144, 546,
        272, 360, 200, 504, 176, 468, 276, 288, 276, 556, 180, 504, 192, 320, 280, 562, 184, 448,
        282, 324, 280, 568, 144, 570, 240, 380, 240, 440, 192, 576, 272, 384, 224, 492, 192, 520,
        288, 288, 292, 586, 168, 540, 232, 392, 288, 592, 180, 384, 296, 396, 264, 598, 160, 600,
        252, 396, 300, 440, 200, 606, 288, 336, 240, 552, 192, 612, 306, 320, 240, 616, 204, 618,
        240, 396, 310, 528, 192, 500, 312, 360, 312, 576, 144, 630, 312, 420, 316, 504, 208, 504,
        280, 420, 256, 640, 212, 642, 264, 336, 288, 646, 216, 580, 240, 360, 324, 652, 216, 520,
        320, 432, 276, 658, 160, 660, 330, 384, 328, 432, 216, 616, 332, 444, 264, 600, 192, 672,
        336, 360, 312, 676, 224, 576, 256, 452, 300, 682, 216, 544, 294, 456, 336, 624, 176, 690,
        344, 360, 346, 552, 224, 640, 348, 464, 240, 700, 216, 648, 320, 368, 352, 600, 232, 708,
        280, 468, 352, 660, 192, 480, 356, 476, 358, 718, 192, 612, 342, 480, 360, 560, 220, 726,
        288, 486, 288, 672, 240, 732, 366, 336, 352, 660, 240, 738, 288, 432, 312, 742, 240, 592,
        372, 492, 320, 636, 200, 750, 368, 500, 336, 600, 216, 756, 378, 440, 288, 760, 252, 648,
        380, 384, 382, 696, 256, 768, 240, 512, 384, 772, 252, 600, 384, 432, 388, 720, 192, 700,
        352, 504, 336, 624, 260, 786, 392, 524, 312, 672, 240, 720, 396, 416, 396, 796, 216, 736,
        320, 528, 400, 720, 264, 528, 360, 536, 400, 808, 216, 810, 336, 540, 360, 648, 256, 756,
        408, 432, 320, 820, 272, 822, 408, 400, 348, 826, 264, 828, 328, 552, 384, 672, 276, 664,
        360, 540, 418, 838, 192, 812, 420, 560, 420, 624, 276, 660, 416, 564, 320, 792, 280, 852,
        360, 432, 424, 856, 240, 858, 336, 480, 430, 862, 288, 688, 432, 544, 360, 780, 224, 792,
        432, 576, 396, 600, 288, 876, 438, 584, 320, 880, 252, 882, 384, 464, 442, 886, 288, 756,
        352, 540, 444, 828, 296, 712, 384, 528, 448, 840, 240, 832, 400, 504, 448, 720, 300, 906,
        452, 600, 288, 910, 288, 820, 456, 480, 456, 780, 288, 918, 352, 612, 460, 840, 240, 720,
        462, 612, 448, 928, 240, 756, 464, 620, 466, 640, 288, 936, 396, 624, 368, 940, 312, 880,
        464, 432, 420, 946, 312, 864, 360, 632, 384, 952, 312, 760, 476, 560, 478, 816, 256, 930,
        432, 636, 480, 768, 264, 966, 440, 576, 384, 970, 324, 828, 486, 480, 480, 976, 324, 880,
        336, 648, 490, 982, 320, 784, 448, 552, 432, 924, 240, 990, 480, 660, 420, 792, 328, 996,
        498, 648, 400,
    ];
    x[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("939\n7,13,x,x,59,x,31,19\n"), 295)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("939\n7,13,x,x,59,x,31,19\n"), 1068781)
    }

    #[test]
    fn solve_congruence_test() {
        assert_eq!(solve_congruence(&vec![(6, 7), (4, 5), (1, 3)]), (34, 105))
    }
}
