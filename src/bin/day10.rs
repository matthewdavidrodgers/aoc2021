#[derive(Debug, Clone, Copy, PartialEq)]
enum BoundType {
	Paren,
	Bracket,
	CurlyBrace,
	AngleBracket,
}

enum ChunkBound {
	Open(BoundType),
	Close(BoundType),
}

fn load_input(input: &str) -> Vec<Vec<ChunkBound>> {
	input.lines()
		.filter(|line| !line.is_empty())
		.map(|line| {
			line.chars()
				.map(|c| match c {
					'(' => ChunkBound::Open(BoundType::Paren),
					'[' => ChunkBound::Open(BoundType::Bracket),
					'{' => ChunkBound::Open(BoundType::CurlyBrace),
					'<' => ChunkBound::Open(BoundType::AngleBracket),
					')' => ChunkBound::Close(BoundType::Paren),
					']' => ChunkBound::Close(BoundType::Bracket),
					'}' => ChunkBound::Close(BoundType::CurlyBrace),
					'>' => ChunkBound::Close(BoundType::AngleBracket),
					_ => panic!("invalid char")
				})
				.collect::<Vec<_>>()

		})
		.collect()
}

enum ValidateResult {
	Valid,
	Incomplete(Vec<BoundType>),
	Corrupt(BoundType),
}

fn validate_line(line: &Vec<ChunkBound>) -> ValidateResult {
	let mut stack: Vec<BoundType> = Vec::new();

	for bound in line {
		match bound {
			ChunkBound::Open(bound_type) => {
				stack.push(*bound_type);
			},
			ChunkBound::Close(bound_type) => {
				match stack.pop() {
					None => return ValidateResult::Corrupt(*bound_type),
					Some(found_type) if found_type != *bound_type => {
						return ValidateResult::Corrupt(*bound_type);
					},
					_ => ()
				}
			}
		}
	}

	if stack.len() > 0 {
		ValidateResult::Incomplete(stack)
	} else {
		ValidateResult::Valid
	}
}

fn part_one(lines: &Vec<Vec<ChunkBound>>) -> u32 {
	let mut corrupt_score = 0;

	for line in lines {
		corrupt_score += match validate_line(line) {
			ValidateResult::Corrupt(bound_type) => match bound_type {
				BoundType::Paren => 3,
				BoundType::Bracket => 57,
				BoundType::CurlyBrace => 1197,
				BoundType::AngleBracket => 25137, 
			},
			_ => 0
		}
	}

	corrupt_score
}

fn part_two(lines: &Vec<Vec<ChunkBound>>) -> u64 {
	let mut incomplete_scores: Vec<u64> = Vec::new();

	for line in lines {
		match validate_line(line) {
			ValidateResult::Incomplete(rem_stack) => {
				let mut score = 0;

				for bound_type in rem_stack.iter().rev() {
					score *= 5;
					score += match bound_type {
						BoundType::Paren => 1,
						BoundType::Bracket => 2,
						BoundType::CurlyBrace => 3,
						BoundType::AngleBracket => 4,
					};
				}

				incomplete_scores.push(score);
			},
			_ => ()
		}
	}

	incomplete_scores.sort();

	incomplete_scores[incomplete_scores.len() / 2]
}

fn main() {
	let input = include_str!("day10.txt");
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
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
		let input = load_input(input);
			
		assert_eq!(part_one(&input), 26397);
	}

	#[test]
	fn test_part_one() {
		let input = include_str!("day10.txt");
		let input = load_input(input);

		assert_eq!(part_one(&input), 321237);
	}

	#[test]
	fn test_part_two_sample() {
		let input = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
		let input = load_input(input);
			
		assert_eq!(part_two(&input), 288957);
	}
}