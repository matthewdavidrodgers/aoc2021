use colored::*;

struct Grid {
    positions: Vec<Vec<u32>>,
    x_len: usize,
    y_len: usize,
}

impl Grid {
    fn adj_next_pos(&self, pos: Pos) -> Vec<Pos> {
        let mut positions = vec![];

        if pos.x < self.x_len - 1 {
            positions.push(Pos {
                x: pos.x + 1,
                y: pos.y,
            });
        }
        if pos.x > 0 {
            positions.push(Pos {
                x: pos.x - 1,
                y: pos.y,
            });
        }
        if pos.y < self.y_len - 1 {
            positions.push(Pos {
                x: pos.x,
                y: pos.y + 1,
            });
        }
        if pos.y > 0 {
            positions.push(Pos {
                x: pos.x,
                y: pos.y - 1,
            });
        }

        positions
    }

    fn find_paths(&self, curr: Pos, curr_path: Vec<Pos>, rem_moves: usize) -> Vec<Vec<Pos>> {
        if rem_moves == 0 {
            return vec![curr_path];
        }

        let first_in_path = curr_path.get(0);

        let adj_positions: Vec<_> = self
            .adj_next_pos(curr)
            .into_iter()
            .filter(|adj_pos| {
                if curr_path.contains(&adj_pos) {
                    return false;
                }
                if let Some(first) = first_in_path {
                    return first.x <= adj_pos.x || first.y <= adj_pos.y;
                }
                return true;
            })
            .collect();

        if adj_positions.len() == 0 {
            return vec![curr_path];
        }

        adj_positions
            .iter()
            .flat_map(|adj_pos| {
                println!("trying {:?} for path {:?}", adj_pos, curr_path);
                let mut curr_path = curr_path.clone();
                curr_path.push(*adj_pos);
                self.find_paths(*adj_pos, curr_path, rem_moves - 1)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

struct Move {
    to: Pos,
    cost: u32,
}

fn chose_path_with_lookahead(
    grid: &Grid,
    curr_pos: Pos,
    target_pos: Pos,
    lookahead: usize,
) -> Option<Move> {
    if curr_pos == target_pos {
        return None;
    }

    let peeking_paths = grid.find_paths(curr_pos, vec![curr_pos], lookahead);
    if peeking_paths.len() == 0 {
        return None;
    }

    let mut cheapest_path: Option<(&Vec<Pos>, u32)> = None;
    for path in &peeking_paths {
        let sum = path.iter().map(|pos| grid.positions[pos.y][pos.x]).sum();

        if let Some((_, min_sum)) = cheapest_path {
            if min_sum > sum {
                cheapest_path = Some((&path, sum));
            }
        } else {
            cheapest_path = Some((&path, sum));
        }
    }

    let (path, _) = cheapest_path.unwrap();
    let move_pos = path[0];

    Some(Move {
        to: move_pos,
        cost: grid.positions[move_pos.y][move_pos.x],
    })
}

fn _part_one(grid: &Grid) -> u32 {
    let mut cost = 0;
    let mut pos = Pos { x: 0, y: 0 };

    while let Some(next_pos) = lookahead_and_choose_move(grid, pos) {
        cost += grid[next_pos.y][next_pos.x];
        pos = next_pos;
    }

    cost
}

fn part_one(grid: &Grid) -> u32 {
    let mut min_cost: Option<u32> = None;

    let mut taken_positions = vec![];
    for lookahead in 3..9 {
        let mut cost = 0;

        let mut pos = Pos { x: 0, y: 0 };
        let target = Pos {
            x: grid.x_len - 1,
            y: grid.y_len - 1,
        };

        while let Some(pos_move) = chose_path_with_lookahead(grid, pos, target, lookahead) {
            println!("TOOK PATH: {:?}", pos_move.to);
            taken_positions.push(pos_move.to);
            pos = pos_move.to;
            cost += pos_move.cost;
        }

        if let Some(min) = min_cost {
            if min > cost {
                min_cost = Some(cost);
            }
        } else {
            min_cost = Some(cost);
        }

        for y in 0..grid.y_len {
            for x in 0..grid.x_len {
                if taken_positions.contains(&Pos { x, y }) {
                    print!("{}", grid.positions[y][x].to_string().red());
                } else {
                    print!("{}", grid.positions[y][x]);
                }
            }
            print!("\n");
        }
        print!("\n");
    }

    min_cost.unwrap()
}

fn load_input(input: &str) -> Grid {
    let positions = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let x_len = positions[0].len();
    let y_len = positions.len();

    Grid {
        positions,
        x_len,
        y_len,
    }
}

fn main() {
    let input = include_str!("day15.txt");
    let input = load_input(input);

    let part_one_answer = part_one(&input);

    println!("PART ONE ANSWER: {}", part_one_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let input = load_input(input);

        assert_eq!(part_one(&input), 40);
    }
}
