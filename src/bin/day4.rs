use std::fmt;

#[derive(Debug, Clone, Copy)]
struct BoardCell {
    number: u32,
    marked: bool,
}

impl BoardCell {
    fn new(number: u32) -> BoardCell {
        BoardCell {
            number,
            marked: false,
        }
    }
}

impl fmt::Display for BoardCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.marked {
            write!(f, "{:2} [X]", self.number)
        } else {
            write!(f, "{:2} [ ]", self.number)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    cells: [BoardCell; 25],
    score: Option<u32>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
			"{}\t{}\t{}\t{}\t{}\n\n{}\t{}\t{}\t{}\t{}\n\n{}\t{}\t{}\t{}\t{}\n\n{}\t{}\t{}\t{}\t{}\n\n{}\t{}\t{}\t{}\t{}\n\n",
			self.cells[0], self.cells[1], self.cells[2], self.cells[3], self.cells[4], 
			self.cells[5], self.cells[6], self.cells[7], self.cells[8], self.cells[9], 
			self.cells[10], self.cells[11], self.cells[12], self.cells[13], self.cells[14], 
			self.cells[15], self.cells[16], self.cells[17], self.cells[18], self.cells[19], 
			self.cells[20], self.cells[21], self.cells[22], self.cells[23], self.cells[24],
		)
    }
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [BoardCell::new(0); 25],
            score: None,
        }
    }

    fn new_from(nums: &[u32]) -> Board {
        let mut board = Board::new();
        for (i, num) in nums.iter().enumerate() {
            board.cells[i].number = *num;
        }
        board
    }

    fn mark_number(&mut self, number: u32) -> Option<u32> {
        for cell_index in 0..self.cells.len() {
            let cell = &mut self.cells[cell_index];
            if cell.number == number && !cell.marked {
                cell.marked = true;
                if self.is_completed_at(cell_index) {
                    let unmarked_cells_sum: u32 = self
                        .cells
                        .iter()
                        .filter(|cell| !cell.marked)
                        .map(|cell| cell.number)
                        .sum();
                    self.score = Some(number * unmarked_cells_sum);
                }
            }
        }
        self.score
    }

    fn is_completed_at(&self, index: usize) -> bool {
        let mut winnable_rows = Vec::new();

        let col_offset = index % 5;

        let hor_row = ((index - col_offset)..(index - col_offset + 5)).collect::<Vec<_>>();
        winnable_rows.push(hor_row);

        let ver_row = (0..5)
            .map(|i| ((i * 5) + col_offset) % 25)
            .collect::<Vec<_>>();
        winnable_rows.push(ver_row);

        let diag_downward_row = (0..5)
            .enumerate()
            .map(|(col, row)| (row * 5) + col)
            .collect::<Vec<_>>();

        let diag_upward_row = (0..5)
            .rev()
            .enumerate()
            .map(|(col, row)| (row * 5) + col)
            .collect::<Vec<_>>();

        if diag_downward_row.contains(&index) {
            winnable_rows.push(diag_downward_row);
        }
        if diag_upward_row.contains(&index) {
            winnable_rows.push(diag_upward_row);
        }

        winnable_rows.into_iter().any(|row| {
            if row.iter().all(|cell_index| self.cells[*cell_index].marked) {
                // println!("won for row: {:?}", row);
                return true;
            }
            return false;
        })
    }
}

fn load_input() -> (Vec<Board>, Vec<u32>) {
    let input = include_str!("day4.txt");

    let mut lines = input.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str_num| str_num.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = Vec::new();
    let mut curr_nums = Vec::new();

    lines.next();

    for line in lines {
        if line == "" {
            boards.push(Board::new_from(&curr_nums));
            curr_nums.clear();
        } else {
            curr_nums.append(
                &mut line
                    .split_whitespace()
                    .map(|board_num| board_num.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
    }

    (boards, numbers)
}

fn part_one(mut boards: Vec<Board>, numbers: &Vec<u32>) -> u32 {
    for num in numbers {
        for board in &mut boards {
            if let Some(score) = board.mark_number(*num) {
                return score;
            }
        }
    }

    panic!("No board won");
}

fn part_two(mut boards: Vec<Board>, numbers: &Vec<u32>) -> u32 {
    let mut remaining = boards.len();

    for number in numbers {
        println!("marking number {}", number);
        // println!("remaining boards {}", remaining);
        println!("remaining unsolved boards:");
        for board in &mut boards {
            if board.score.is_some() {
                continue;
            }
            let did_win = board.mark_number(*number);
            if let Some(score) = did_win {
                if remaining == 1 {
                    // println!("won with board:\n{}", board);
                    return score;
                } else {
                    // println!("completed board:\n{}", board);
                    remaining -= 1;
                }
            } else {
                println!("{}", board);
            }
        }
    }

    panic!("No board won");
}

fn part_two_other(mut boards: Vec<Board>, numbers: &Vec<u32>) -> u32 {
    boards
        .iter_mut()
        .map(|board| {
            for (index, number) in numbers.iter().enumerate() {
                if let Some(score) = board.mark_number(*number) {
                    return Some((score, index + 1));
                }
            }
            None
        })
        .filter(|score| score.is_some())
        .fold((0, 0), |(score, most_turns), curr| {
            let (curr_score, turns) = curr.unwrap();
            if turns > most_turns {
                (curr_score, turns)
            } else {
                (score, most_turns)
            }
        })
        .0
}

// fn part_two(mut boards: Vec<Board>, numbers: &Vec<u32>) -> u32 {
// 	for num in numbers {
// 		let mut next_boards = Vec::new();
// 		let remaining = boards.len();
// 		for mut board in boards {
// 			let did_win = board.mark_number(*num);
// 			if let Some(score) = did_win {
// 				if remaining == 1 {
// 					return score;
// 				}
// 			} else {
// 				next_boards.push(board);
// 			}
// 		}
//
// 		boards = next_boards;
// 	}
//
// 	panic!("No board won");
// }

fn main() {
    let (boards, numbers) = load_input();

    let part_one_answer = part_one(boards.clone(), &numbers);
    let part_two_answer = part_two_other(boards.clone(), &numbers);

    println!("PART ONE ANSWER: {}", part_one_answer);
    println!("PART TWO ANSWER: {}", part_two_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_is_won() {
        let mut board = Board::new_from(&[
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ]);

        assert_eq!(board.is_completed_at(0), false);
        assert_eq!(board.is_completed_at(16), false);
        assert_eq!(board.is_completed_at(17), false);
        assert_eq!(board.is_completed_at(18), false);

        board.cells[0].marked = true;
        board.cells[1].marked = true;
        board.cells[2].marked = true;
        board.cells[3].marked = true;
        board.cells[4].marked = true;

        assert_eq!(board.is_completed_at(0), true);
        assert_eq!(board.is_completed_at(16), false);
        assert_eq!(board.is_completed_at(17), false);
        assert_eq!(board.is_completed_at(18), false);

        board.cells[7].marked = true;
        board.cells[12].marked = true;
        board.cells[17].marked = true;
        board.cells[22].marked = true;

        assert_eq!(board.is_completed_at(0), true);
        assert_eq!(board.is_completed_at(16), false);
        assert_eq!(board.is_completed_at(17), true);
        assert_eq!(board.is_completed_at(18), false);

        board.cells[8].marked = true;
        board.cells[16].marked = true;
        board.cells[20].marked = true;

        assert_eq!(board.is_completed_at(0), true);
        assert_eq!(board.is_completed_at(16), true);
        assert_eq!(board.is_completed_at(17), true);
        assert_eq!(board.is_completed_at(18), false);

        board.cells[6].marked = true;
        board.cells[18].marked = true;
        board.cells[24].marked = true;

        assert_eq!(board.is_completed_at(0), true);
        assert_eq!(board.is_completed_at(16), true);
        assert_eq!(board.is_completed_at(17), true);
        assert_eq!(board.is_completed_at(18), true);
    }

    #[test]
    fn test_part_two_sample() {
        let boards = vec![
            Board::new_from(&[
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19,
            ]),
            Board::new_from(&[
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6,
            ]),
            Board::new_from(&[
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ]),
        ];

        let nums = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        assert_eq!(part_two(boards, &nums), 1924);
    }

    #[test]
    fn test_part_one() {
        let (boards, numbers) = load_input();
        assert_eq!(part_one(boards, &numbers), 29440);
    }
}
