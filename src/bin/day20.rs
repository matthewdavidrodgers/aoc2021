#[derive(Debug, Clone)]
struct Image {
	x_size: usize,
	y_size: usize,
	bits: Vec<Vec<bool>>,
}

impl Image {
	fn new(x_size: usize, y_size: usize) -> Image {
		Image {
			x_size,
			y_size,
			bits: vec![vec![false; x_size]; y_size],
		}
	}

	#[allow(dead_code)]
	fn print(&self) {
		for i in 0..self.y_size {
			for j in 0..self.x_size {
				if self.bits[i][j] {
					print!("X");
				} else {
					print!(".");
				}
			}
			print!("\n");
		}
		print!("\n");
	}

	fn extend(&mut self, by: usize) {
		for _ in 0..by {
			for bit_row in &mut self.bits {
				bit_row.insert(0, false);
				bit_row.push(false);
			}
		}
		self.x_size += by * 2;

		for _ in 0..by {
			self.bits.insert(0, vec![false; self.x_size]);
			self.bits.push(vec![false; self.x_size]);
		}
		self.y_size += by * 2;
	}

	fn map_by_algorithm(self, algorithm: &Vec<bool>) -> Image {
		let mut mapped = Image::new(self.x_size, self.y_size);

		for i in 0..self.y_size {
			for j in 0..self.x_size  {
				let index_bits = vec![
					if i > 0 && j > 0 { self.bits[i-1][j-1] } else { self.bits[i][j] },
					if i > 0 { self.bits[i-1][j] } else { self.bits[i][j] },
					if i > 0 && j < self.x_size - 1 { self.bits[i-1][j+1] } else { self.bits[i][j] },
					if j > 0 { self.bits[i][j-1] } else { self.bits[i][j] },
					self.bits[i][j],
					if j < self.x_size - 1 { self.bits[i][j+1] } else { self.bits[i][j] },
					if i < self.y_size - 1 && j > 0 { self.bits[i+1][j-1] } else { self.bits[i][j] },
					if i < self.y_size - 1 { self.bits[i+1][j] } else { self.bits[i][j] },
					if i < self.y_size - 1 && j < self.x_size - 1 { self.bits[i+1][j+1] } else { self.bits[i][j] },
				];

				let mut index: u16 = 0;
				for (bit_pos, bit) in index_bits.into_iter().enumerate() {
					if bit {
						let mask = 0x1 << 8 - bit_pos;
						index |= mask;
					}
				}

				mapped.bits[i][j] = algorithm[index as usize];
			}
		}

		mapped
	}

	fn lit_pixels(&self) -> usize {
		self.bits
			.iter()
			.flat_map(|bit_row| {
				bit_row
					.iter()
					.filter(|bit| **bit)
					.collect::<Vec<_>>()
			})
			.count()
	}
}

fn load_input(input: &str) -> (Vec<bool>, Image) {
	let mut lines = input.lines();
	let algorithm: Vec<_> = lines
		.next()
		.unwrap()
		.chars()
		.map(|c| match c {
			'#' => true,
			'.' => false,
			_ => panic!("Invalid char"),
		})
		.collect();

	lines.next().unwrap();

	let mut x_size = 0;

	let bits: Vec<_> = lines
		.filter_map(|line| {
			if line.is_empty() {
				return None;
			}

			x_size = line.len();
			Some(line
				.chars()
				.map(|c| match c {
					'#' => true,
					'.' => false,
					_ => panic!("Invalid char"),
				})
				.collect::<Vec<_>>())
		})
		.collect();

	let y_size = bits.len();

	(algorithm, Image { bits, x_size, y_size })
}

fn map_by_algorithm_times(mut image: Image, algorithm: &Vec<bool>, times: usize) -> Image {
	image.extend(times);

	for _ in 0..times {
		image = image.map_by_algorithm(algorithm);
	}

	image
}

fn part_one(mut image: Image, algorithm: &Vec<bool>) -> usize {
	image = map_by_algorithm_times(image, algorithm, 2);

	image.lit_pixels()
}

fn part_two(mut image: Image, algorithm: &Vec<bool>) -> usize {
	image = map_by_algorithm_times(image, algorithm, 50);

	image.lit_pixels()
}

fn main() {
	let input = include_str!("day20.txt");
	let (algorithm, image) = load_input(input);

	let part_one_answer = part_one(image.clone(), &algorithm);
	let part_two_answer = part_two(image, &algorithm);

	println!("PART ONE ANSWER: {}", part_one_answer);
	println!("PART TWO ANSWER: {}", part_two_answer);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one_sample() {
		let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
		let (algorithm, image) = load_input(input);

		assert_eq!(part_one(image, &algorithm), 35);
	}

	#[test]
	fn test_part_two_sample() {
		let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
		let (algorithm, image) = load_input(input);

		assert_eq!(part_two(image, &algorithm), 3351);
	}

	#[test]
	fn test_part_one() {
		let input = include_str!("day20.txt");
		let (algorithm, image) = load_input(input);

		assert_eq!(part_one(image, &algorithm), 5464);
	}
}