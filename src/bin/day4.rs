#[derive(Debug, Clone, Copy)]
struct BoardCell {
	number: u32,
	marked: bool,
}

impl BoardCell {
	fn new(number: u32) -> BoardCell {
		BoardCell { number, marked: false }
	}
}

#[derive(Debug, Clone, Copy)]
struct Board {
	cells: [BoardCell; 25],
}

impl Board {
	fn new() -> Board {
		Board { cells: [BoardCell::new(0); 25] }
	}

	fn new_from(nums: &[u32]) -> Board {
		let mut board = Board::new();
		for (i, num) in nums.iter().enumerate() {
			board.cells[i].number = *num;
		}
		board
	}

	fn mark_number(&mut self, number: u32) -> bool {
		for cell_index in 0..self.cells.len() {
			let cell = &mut self.cells[cell_index];
			if cell.number == number && !cell.marked {
				cell.marked = true;
				return self.is_completed_at(cell_index);
			}
		}
		false
	}

	fn is_completed_at(&self, index: usize) -> bool {
		let mut winnable_rows = Vec::new();

		let col_offset = index / 5;
		let hor_row = (col_offset..(col_offset+5)).collect::<Vec<_>>();
		winnable_rows.push(hor_row);

		let row_offset = index % 5;
		let ver_row = (0..5).map(|i| ((i * 5) + row_offset) % 25).collect::<Vec<_>>();
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
			row.into_iter().all(|cell_index| self.cells[cell_index].marked)
		})
	}
}

fn load_input() -> (Vec<Board>, Vec<u32>) {
	let input = include_str!("day4.txt");

	let mut lines = input.lines();

	let numbers = lines.next().unwrap()
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
			curr_nums.append(&mut line
				.split_whitespace()
				.map(|board_num| board_num.parse::<u32>().unwrap())
				.collect::<Vec<_>>());
		}
	}

	(boards, numbers)
}

fn part_one(boards: &Vec<Board>, numbers: &Vec<u32>) -> u32 {
	let mut boards = (*boards).clone();

	for num in numbers {
		for board in &mut boards {
			if board.mark_number(*num) {
				let unmarked_cells_sum: u32 = board.cells.iter()
					.filter(|cell| !cell.marked)
					.map(|cell| cell.number)
					.sum();
				return num * unmarked_cells_sum;
			}
		}
	}

	panic!("No board won");
}

fn main() {
	let (boards, numbers) = load_input();

	let part_one_answer = part_one(&boards, &numbers);

	println!("PART ONE ANSWER: {}", part_one_answer);
}

#[cfg(tests)]
mod tests {
	use super::*;

	#[test]
	fn board_is_won() {
		let mut board = Board::new_from([
			22, 13, 17, 11,  0,
			 8,  2, 23,  4, 24,
			21,  9, 14, 16,  7,
			 6, 10,  3, 18,  5,
			 1, 12, 20, 15, 19
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

		board.cells[6].marked = true;
		board.cells[16].marked = true;
		board.cells[20].marked = true;

		assert_eq!(board.is_completed_at(0), true);
		assert_eq!(board.is_completed_at(16), true);
		assert_eq!(board.is_completed_at(17), true);
		assert_eq!(board.is_completed_at(18), false);

		board.cells[18].marked = true;
		board.cells[24].marked = true;

		assert_eq!(board.is_completed_at(0), true);
		assert_eq!(board.is_completed_at(16), true);
		assert_eq!(board.is_completed_at(17), true);
		assert_eq!(board.is_completed_at(18), true);
	}

	#[test]
	fn test_part_one() {
		let (boards, numbers) = load_input();
		assert_eq!(part_one(&boards, &numbers), 29440);
	}
}