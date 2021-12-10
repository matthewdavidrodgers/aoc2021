fn load_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part_one(heights: &Vec<Vec<u32>>) -> u32 {
    let mut sum_risks = 0;

    for i in 0..heights.len() {
        for j in 0..heights[i].len() {
            let height = heights[i][j];
            if i > 0 && heights[i - 1][j] <= height {
                continue;
            }
            if i < heights.len() - 1 && heights[i + 1][j] <= height {
                continue;
            }
            if j > 0 && heights[i][j - 1] <= height {
                continue;
            }
            if j < heights[i].len() - 1 && heights[i][j + 1] <= height {
                continue;
            }

            sum_risks += 1 + height;
        }
    }

    sum_risks
}

struct Basin {
    positions: Vec<(usize, usize)>,
}

impl Basin {
    fn new() -> Basin {
        Basin {
            positions: Vec::new(),
        }
    }

    fn pos_is_adjacent(&self, (i, j): (usize, usize)) -> bool {
        for (b_i, b_j) in &self.positions {
            if i == *b_i && j + 1 == *b_j {
                return true;
            }
            if i == *b_i && j > 0 && j - 1 == *b_j {
                return true;
            }
            if i + 1 == *b_i && j == *b_j {
                return true;
            }
            if i > 0 && i - 1 == *b_i && j == *b_j {
                return true;
            }
        }

        false
    }

    fn merge(basins: Vec<Basin>) -> Basin {
        let mut basin = Basin::new();

        for mut b in basins {
            basin.positions.append(&mut b.positions);
        }

        basin
    }
}

fn part_two(heights: &Vec<Vec<u32>>) -> u32 {
    let mut non_nine_positions = Vec::new();

    for i in 0..heights.len() {
        for j in 0..heights[i].len() {
            if heights[i][j] != 9 {
                non_nine_positions.push((i, j));
            }
        }
    }

    let mut basins: Vec<Basin> = Vec::new();

    for (i, j) in &non_nine_positions {
        let mut matching_basins: Vec<Basin> = Vec::new();
        let mut remaining_basins: Vec<Basin> = Vec::new();
        for b in basins {
            if b.pos_is_adjacent((*i, *j)) {
                matching_basins.push(b)
            } else {
                remaining_basins.push(b);
            }
        }

        let mut merged_basin = match matching_basins.len() {
            0 => Basin::new(),
            _ => Basin::merge(matching_basins),
        };
        merged_basin.positions.push((*i, *j));

        remaining_basins.push(merged_basin);

        basins = remaining_basins;
    }

    let mut basin_sizes: Vec<_> = basins
        .into_iter()
        .map(|basin| basin.positions.len())
        .collect();
    basin_sizes.sort();

    basin_sizes[basin_sizes.len() - 1] as u32
        * basin_sizes[basin_sizes.len() - 2] as u32
        * basin_sizes[basin_sizes.len() - 3] as u32
}

fn main() {
    let input = include_str!("day9.txt");
    let input = load_input(input);

    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("PART ONE ANSWER: {}", part_one_answer);
    println!("PART TWO ANSWER: {}", part_two_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let input = "
2199943210
3987894921
9856789892
8767896789
9899965678";
        let input = load_input(input);

        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("day9.txt");
        let input = load_input(input);

        assert_eq!(part_one(&input), 436);
    }

    #[test]
    fn test_part_two_sample() {
        let input = "
2199943210
3987894921
9856789892
8767896789
9899965678";
        let input = load_input(input);

        assert_eq!(part_two(&input), 1134);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day9.txt");
        let input = load_input(input);

        assert_eq!(part_two(&input), 1317792);
    }
}
