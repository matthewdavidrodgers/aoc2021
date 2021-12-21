#[derive(Debug, Clone)]
enum PairValue {
	Terminal(usize),
	Child(Box<Pair>),
}

#[derive(Debug, Clone)]
struct Pair {
	left: PairValue,
	right: PairValue,
}

impl Pair {
	#[allow(dead_code)]
	fn print_structure(&self) {
		print!("[");
		match &self.left {
			PairValue::Terminal(index) => print!("{}", *index),
			PairValue::Child(children) => children.print_structure(),
		}
		print!(",");
		match &self.right {
			PairValue::Terminal(index) => print!("{}", *index),
			PairValue::Child(children) => children.print_structure(),
		}
		print!("]");
	}

	fn to_string(&self, root: &PairRoot) -> String {
		let left = match &self.left {
			PairValue::Terminal(index) => format!("{}", root.data[*index]),
			PairValue::Child(children) => children.to_string(root),
		};
		let right = match &self.right {
			PairValue::Terminal(index) => format!("{}", root.data[*index]),
			PairValue::Child(children) => children.to_string(root),
		};

		format!("[{},{}]", left, right)
	}

	fn apply_index_update<F>(&mut self, update: &mut F)
		where F: FnMut(usize) -> usize {
		
		match &mut self.left {
			PairValue::Terminal(index) => *index = update(*index),
			PairValue::Child(children) => children.apply_index_update(update),
		}
		match &mut self.right {
			PairValue::Terminal(index) => *index = update(*index),
			PairValue::Child(children) => children.apply_index_update(update),
		}
	}

	fn reduce_explosion(&mut self, curr_depth: usize, data: &mut Vec<u32>) -> Option<usize> {
		match &mut self.left {
			PairValue::Child(children) if curr_depth == 3 => {
				let left_terminal_index = match &children.left {
					PairValue::Terminal(v) => *v,
					PairValue::Child(..) => panic!("Found children at a depth of 5"),
				};
				let right_terminal_index = match &children.right {
					PairValue::Terminal(v) => *v,
					PairValue::Child(..) => panic!("Found children at a depth of 5"),
				};

				if left_terminal_index != 0 {
					data[left_terminal_index - 1] += data[left_terminal_index];
				}
				if right_terminal_index < data.len() - 1 {
					data[right_terminal_index + 1] += data[right_terminal_index];
				}

				data[left_terminal_index] = 0;
				data.remove(right_terminal_index);

				self.left = PairValue::Terminal(left_terminal_index);

				return Some(left_terminal_index);
			},
			PairValue::Child(children) => {
				let child_reduction = children.reduce_explosion(curr_depth+1, data);
				if child_reduction.is_some() {
					return child_reduction;
				}
			},
			PairValue::Terminal(..) => {}
		}

		match &mut self.right {
			PairValue::Child(children) if curr_depth == 3 => {
				let left_terminal_index = match &children.left {
					PairValue::Terminal(v) => *v,
					PairValue::Child(..) => panic!("Found children at a depth of 5"),
				};
				let right_terminal_index = match &children.right {
					PairValue::Terminal(v) => *v,
					PairValue::Child(..) => panic!("Found children at a depth of 5"),
				};

				if left_terminal_index != 0 {
					data[left_terminal_index - 1] += data[left_terminal_index];
				}
				if right_terminal_index < data.len() - 1 {
					data[right_terminal_index + 1] += data[right_terminal_index];
				}

				data[left_terminal_index] = 0;
				data.remove(right_terminal_index);

				self.right = PairValue::Terminal(left_terminal_index);

				return Some(left_terminal_index);
			},
			PairValue::Child(children) => {
				let child_reduction = children.reduce_explosion(curr_depth+1, data);
				if child_reduction.is_some() {
					return child_reduction;
				}
			},
			PairValue::Terminal(..) => {}
		}

		None
	}

	fn reduce_split(&mut self, data: &mut Vec<u32>) -> Option<usize> {
		match &mut self.left {
			PairValue::Terminal(v) if data[*v] >= 10 => {
				let orig = data[*v];
				let left = orig / 2;
				let right = if orig % 2 == 0 { orig / 2 } else { (orig / 2) + 1 };

				data[*v] = left;
				data.insert(*v + 1, right);

				let split_at = *v;

				self.left = PairValue::Child(Box::new(Pair {
					left: PairValue::Terminal(*v),
					right: PairValue::Terminal(*v + 1),
				}));

				return Some(split_at);
			},
			PairValue::Child(children) => {
				let child_reduction = children.reduce_split(data);
				if child_reduction.is_some() {
					return child_reduction;
				}
			},
			_ => {}
		}

		match &mut self.right {
			PairValue::Terminal(v) if data[*v] >= 10 => {
				let orig = data[*v];
				let left = orig / 2;
				let right = if orig % 2 == 0 { orig / 2 } else { (orig / 2) + 1 };

				data[*v] = left;
				data.insert(*v + 1, right);

				let split_at = *v;

				self.right = PairValue::Child(Box::new(Pair {
					left: PairValue::Terminal(*v),
					right: PairValue::Terminal(*v + 1),
				}));

				return Some(split_at);
			},
			PairValue::Child(children) => {
				let child_reduction = children.reduce_split(data);
				if child_reduction.is_some() {
					return child_reduction;
				}
			},
			_ => {}
		}

		None
	}

	fn magnitude(&self, root: &PairRoot) -> u32 {
		let left = match &self.left {
			PairValue::Terminal(v) => root.data[*v],
			PairValue::Child(children) => children.magnitude(root),
		};

		let right = match &self.right {
			PairValue::Terminal(v) => root.data[*v],
			PairValue::Child(children) => children.magnitude(root),
		};

		(left * 3) + (right * 2)
	}
}

#[derive(Debug, Clone)]
struct PairRoot {
	data: Vec<u32>,
	structure: Pair,
}

impl PairRoot {
	fn from_str(text: &str) -> PairRoot {
		let mut parser = PairParser::new(text);
		parser.parse()
	}

	#[allow(dead_code)]
	fn print_structure(&self) {
		println!("data: {:?}", self.data);
		print!("structure: ");
		self.structure.print_structure();
		print!("\n");
	}

	fn to_string(&self) -> String {
		self.structure.to_string(self)
	}

	fn add(&self, other: &PairRoot) -> PairRoot {
		let mut new_data = vec![];
		new_data.append(&mut self.data.clone());
		new_data.append(&mut other.data.clone());

		let left = self.structure.clone();

		let mut right = other.structure.clone();
		let increase_by = self.data.len();
		right.apply_index_update(&mut |index| index + increase_by);

		let new_structure = Pair {
			left: PairValue::Child(Box::new(left)),
			right: PairValue::Child(Box::new(right)),
		};

		let mut new_root = PairRoot {
			data: new_data,
			structure: new_structure,
		};
		new_root.reduce();

		new_root
	}

	fn reduce(&mut self) {
		loop {
			if let Some(exploded_at) = self.structure.reduce_explosion(0, &mut self.data) {
				self.structure.apply_index_update(&mut |i| {
					if i > exploded_at { i - 1 } else { i }
				});
			} else if let Some(split_at) = self.structure.reduce_split(&mut self.data) {
				let mut seen_first_of_dupe = false;
				self.structure.apply_index_update(&mut |i| {
					if i > split_at + 1 {
						return i + 1;
					}
					if i == split_at + 1 {
						if !seen_first_of_dupe {
							seen_first_of_dupe = true;
							return i;
						} else {
							return i + 1;
						}
					}
					return i;
				});
			} else {
				break;
			}
		}
	}

	fn magnitude(&self) -> u32 {
		self.structure.magnitude(self)
	}
}

struct PairParser {
	chars: Vec<char>,
	index: usize,
}

impl PairParser {
	fn new(text: &str) -> PairParser {
		PairParser { chars: text.chars().collect(), index: 0 }
	}

	fn advance(&mut self) -> char {
		let c = self.chars[self.index];
		self.index += 1;
		c
	}

	fn parse_pair(&mut self, data: &mut Vec<u32>) -> Pair {
		let left = match self.advance() {
			'[' => {
				let child = PairValue::Child(Box::new(self.parse_pair(data)));
				if self.advance() != ',' {
					panic!("Unexpected char after parsing left of pair");
				}
				child
			},
			c => {
				let mut chars = vec![c];
				loop {
					let next_c = self.advance();
					if next_c == ',' {
						break;
					}
					chars.push(next_c);
				}
				let term_str: String = chars.into_iter().collect();
				let term: u32 = term_str.parse().unwrap();
				data.push(term);
				let index = data.len() - 1;
				PairValue::Terminal(index)
			},
		};

		let right = match self.advance() {
			'[' => {
				let child = PairValue::Child(Box::new(self.parse_pair(data)));
				if self.advance() != ']' {
					panic!("Unexpected char after parsing left of pair");
				}
				child
			},
			c => {
				let mut chars = vec![c];
				loop {
					let next_c = self.advance();
					if next_c == ']' {
						break;
					}
					chars.push(next_c);
				}
				let term_str: String = chars.into_iter().collect();
				let term: u32 = term_str.parse().unwrap();
				data.push(term);
				let index = data.len() - 1;
				PairValue::Terminal(index)
			},
		};

		Pair { left, right }
	}

	fn parse(&mut self) -> PairRoot {
		if self.advance() != '[' {
			panic!("Unexpected char at start of pair");
		}

		let mut data = vec![];

		let pair = self.parse_pair(&mut data);

		PairRoot { data, structure: pair }
	}
}

fn load_input(input: &str) -> Vec<PairRoot> {
	input
		.lines()
		.filter_map(|line| {
			if line.is_empty() { return None; }
			Some(PairRoot::from_str(&line))
		})
		.collect()
}

fn sum_list(numbers: &Vec<PairRoot>) -> PairRoot {
	let mut iter = numbers.iter();
	let first = (*iter.next().unwrap()).clone();

	iter.fold(first, |curr_total, num| {
		curr_total.add(num)
	})
}

fn part_one(numbers: &Vec<PairRoot>) -> u32 {
	let total = sum_list(numbers);

	total.magnitude()
}

fn part_two(numbers: &Vec<PairRoot>) -> u32 {
	let mut max_mag: Option<u32> = None;

	for i in 0..numbers.len() {
		for j in 0..numbers.len() {
			if i != j {
				let a = &numbers[i];
				let b = &numbers[j];

				let magnitude = a.add(b).magnitude();
				if let Some(max) = max_mag {
					if max < magnitude {
						max_mag = Some(magnitude);
					}
				} else {
					max_mag = Some(magnitude);
				}
			}
		}
	}

	max_mag.unwrap()
}

fn main() {
	let input = include_str!("day18.txt");
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
	fn test_data_parse() {
		let root = PairRoot::from_str("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");
		assert_eq!(root.data, vec![0,5,8,1,7,9,6,4,1,2,1,4,2]);

		let root = PairRoot::from_str("[[[5,[2,8]],4],[5,[[9,9],0]]]");
		assert_eq!(root.data, vec![5,2,8,4,5,9,9,0]);

		let root = PairRoot::from_str("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]");
		assert_eq!(root.data, vec![6,6,2,5,6,7,6,4,7]);

		let root = PairRoot::from_str("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]");
		assert_eq!(root.data, vec![6,0,7,0,9,4,9,9,0]);
	}

	#[test]
	fn test_concatenative_add() {
		let a = PairRoot::from_str("[1,2]");
		let b = PairRoot::from_str("[[3,4],5]");
		let c = a.add(&b);

		assert_eq!(c.to_string(), "[[1,2],[[3,4],5]]");
		assert_eq!(c.data, vec![1,2,3,4,5]);
	}

	#[test]
	fn test_reduction() {
		let a = PairRoot::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]");
		let b = PairRoot::from_str("[1,1]");
		let c = a.add(&b);
		assert_eq!(c.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
	}

	#[test]
	fn test_add_and_reduce() {
		let input = "
[1,1]
[2,2]
[3,3]
[4,4]
";
		let numbers = load_input(input);
		let total = sum_list(&numbers);

		assert_eq!(total.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");

		let total = total.add(&PairRoot::from_str("[5,5]"));

		assert_eq!(total.to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]");

		let total = total.add(&PairRoot::from_str("[6,6]"));

		assert_eq!(total.to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
	}

	#[test]
	fn test_add_big_nums() {
		let a = PairRoot::from_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
		let b = PairRoot::from_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
		let c = a.add(&b);

		assert_eq!(c.to_string(), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");

		let a = PairRoot::from_str("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
		let b = PairRoot::from_str("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]");
		let c = a.add(&b);

		assert_eq!(c.to_string(), "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");
	}

	#[test]
	fn test_larger_add_and_reduce() {
		let input = "
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
";
		let numbers = load_input(input);

		let total = sum_list(&numbers);

		assert_eq!(total.to_string(), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
	}

	#[test]
	fn test_part_one_sample() {
		let input = "
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";
		let numbers = load_input(input);

		assert_eq!(part_one(&numbers), 4140);
	}

	#[test]
	fn test_part_two_sample() {
		let input = "
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";
		let numbers = load_input(input);

		assert_eq!(part_two(&numbers), 3993);
	}
}