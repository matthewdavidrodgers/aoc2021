fn load_input(input: &str) -> (usize, usize) {
	let mut lines = input.lines();

	let player_one_chars: Vec<_> = lines.next().unwrap().chars().collect();
	let player_two_chars: Vec<_> = lines.next().unwrap().chars().collect();

	let player_one_pos = player_one_chars[28].to_digit(10).unwrap() as usize;
	let player_two_pos = player_two_chars[28].to_digit(10).unwrap() as usize;

	(player_one_pos, player_two_pos)
}

fn part_one(mut pos_a: usize, mut pos_b: usize) -> usize {
	let mut score_a = 0;
	let mut score_b = 0;

	let mut roles = 0;
	
	loop {
		let (curr_score, curr_pos) = if (roles / 3) % 2 == 0 {
			(&mut score_a, &mut pos_a)
		} else {
			(&mut score_b, &mut pos_b)
		};

		let mut curr_role = 0;
		for _ in 0..3 {
			roles += 1;
			let role_val = ((roles - 1) % 100) + 1;
			curr_role += role_val;
		}

		*curr_pos = ((*curr_pos + curr_role - 1) % 10) + 1;
		*curr_score += *curr_pos;

		if *curr_score >= 1000 {
			let (losing_score, _) = if (roles / 3) % 2 == 0 {
				(score_a, pos_a)
			} else {
				(score_b, pos_b)
			};

			return losing_score * roles;
		}
	}
}

fn main() {
	let input = include_str!("day21.txt");
	let (pos_a, pos_b) = load_input(input);

	let part_one_answer = part_one(pos_a, pos_b);

	println!("PART ONE ANSWER: {}", part_one_answer);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one_sample() {
		let input = "Player 1 starting position: 4
Player 2 starting position: 8
";
		let (pos_a, pos_b) = load_input(input);

		assert_eq!(part_one(pos_a, pos_b), 739785);

	}

	#[test]
	fn test_part_one() {
		let input = include_str!("day21.txt");
		let (pos_a, pos_b) = load_input(input);

		assert_eq!(part_one(pos_a, pos_b), 556206);
	}
}