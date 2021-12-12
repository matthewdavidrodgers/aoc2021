// use std::cmp::Ordering;

fn load_input(input: &str) -> [u32; 100] {
    let mut data: [u32; 100] = [0; 100];

    let mut lines = input.lines();

    for i in 0..10 {
        let line = lines.next().unwrap();
        let mut chars = line.chars();
        for j in 0..10 {
            let c = chars.next().unwrap();
            let c = c.to_digit(10).unwrap();
            data[(i * 10) + j] = c;
        }
    }

    data
}

fn pos_from(row: usize, col: usize) -> usize {
    (row * 10) + col
}

fn adj_pos(pos: usize) -> Vec<usize> {
    let mut positions = Vec::new();

    let row = pos / 10;
    let col = pos % 10;

    if row > 0 {
        positions.push(pos_from(row - 1, col));
    }
    if row < 9 {
        positions.push(pos_from(row + 1, col));
    }
    if col > 0 {
        positions.push(pos_from(row, col - 1));
    }
    if col < 9 {
        positions.push(pos_from(row, col + 1));
    }
    if row > 0 && col > 0 {
        positions.push(pos_from(row - 1, col - 1));
    }
    if row > 0 && col < 9 {
        positions.push(pos_from(row - 1, col + 1));
    }
    if row < 9 && col > 0 {
        positions.push(pos_from(row + 1, col - 1));
    }
    if row < 9 && col < 9 {
        positions.push(pos_from(row + 1, col + 1));
    }

    positions
}

fn perform_step(octopi: &mut [u32; 100]) -> u32 {
    let mut flashes = 0;

    let mut update_positions: Vec<_> = (0..100).collect();
    let mut first_pass = true;

    while update_positions.len() > 0 {
        let mut next_update_positions = Vec::new();
        for pos in update_positions {
            octopi[pos] = if octopi[pos] == 9 {
                flashes += 1;
                next_update_positions.append(&mut adj_pos(pos));
                0
            } else if octopi[pos] != 0 || first_pass {
                octopi[pos] + 1
            } else {
                octopi[pos]
            };
        }
        first_pass = false;
        update_positions = next_update_positions;
    }

    flashes
}

fn part_one(mut octopi: [u32; 100], steps: u32) -> u32 {
    let mut flashes = 0;

    for _ in 0..steps {
        flashes += perform_step(&mut octopi);
    }

    flashes
}

fn part_two(mut octopi: [u32; 100]) -> u32 {
    let mut step = 1;

    loop {
        if perform_step(&mut octopi) == 100 {
            return step;
        }
        step += 1;
    }
}

fn main() {
    let input = include_str!("day11.txt");
    let input = load_input(input);

    let part_one_answer = part_one(input.clone(), 100);
    let part_two_answer = part_two(input.clone());

    println!("PART ONE ANSWER: {}", part_one_answer);
    println!("PART TWO ANSWER: {}", part_two_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let input = load_input(input);

        assert_eq!(part_one(input.clone(), 10), 204);
        assert_eq!(part_one(input.clone(), 100), 1656);
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(adj_pos(pos_from(4, 8)).len(), 8);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("day11.txt");
        let input = load_input(input);

        assert_eq!(part_one(input, 100), 1679);
    }

    #[test]
    fn test_part_two_sample() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let input = load_input(input);

        assert_eq!(part_two(input.clone()), 195);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day11.txt");
        let input = load_input(input);

        assert_eq!(part_two(input), 519);
    }
}
