#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

#[derive(Debug)]
enum LineType {
	Horizontal { y: i32 },
	Vertical { x: i32 },
	Diagonal { m: f64, b: f64 }
}

#[derive(Debug)]
struct Line {
	point_a: Point,
	point_b: Point,
	line_type: LineType,
	length: f64,
}

impl Line {
	fn new(point_a: Point, point_b: Point) -> Line {
		use LineType::*;

		if point_a.x == point_b.x {
			let x = point_a.x;
			let length = (point_b.y - point_a.y) as f64;
			let length = length.abs() + 1f64;
			Line {
				point_a,
				point_b,
				line_type: Vertical { x },
				length,
			}
		} else if point_a.y == point_b.y {
			let y = point_a.y; 
			let length = (point_b.x - point_a.x) as f64;
			let length = length.abs() + 1f64;
			Line {
				point_a,
				point_b,
				line_type: Horizontal { y },
				length,
			}
		} else {
			let m = (point_a.y - point_b.y) as f64 / (point_a.x - point_b.x) as f64;
			let b = m * 0f64;
			let length = ((point_a.x + point_b.x).pow(2)) + ((point_a.y + point_b.y).pow(2));
			let length = (length as f64).sqrt() + 1f64;
			Line {
				point_a,
				point_b,
				line_type: Diagonal { m, b },
				length,
			}
		}
	}

	fn ordered_points(&self) -> (&Point, &Point) {
		match &self.line_type {
			LineType::Vertical { x: _ } => {
				if self.point_a.y <= self.point_b.y {
					(&self.point_a, &self.point_b)
				} else {
					(&self.point_b, &self.point_a)
				}
			},
			_ => {
				if self.point_a.x <= self.point_b.x {
					(&self.point_a, &self.point_b)
				} else {
					(&self.point_b, &self.point_a)
				}
			}
		}
	}

	fn intersections(&self, other: &Line) -> u32 {
		use LineType::*;

		match (&self.line_type, &other.line_type) {
			(Vertical { x: x_a }, Vertical { x: x_b }) => {
				if *x_a != *x_b {
					return 0;
				}
				let (bigger, smaller) = if self.length >= other.length {
					(self, other)
				} else {
					(other, self)
				};
				let (smaller_start, smaller_end) = smaller.ordered_points();
				let (bigger_start, bigger_end) = bigger.ordered_points();

				let intersects = if smaller_start.y > bigger_end.y || smaller_end.y < bigger_start.y {
					0
				} else if smaller_start.y < bigger_start.y {
					(smaller.length as i32) - (bigger_start.y - smaller_start.y)
				} else if smaller_end.y > bigger_end.y {
					(smaller.length as i32) - (bigger_end.y - smaller_end.y)
				} else {
					smaller.length as i32
				};

				if intersects > 0 {
					return intersects as u32;
				} else {
					return 0;
				}
			},
			(Horizontal { y: y_a }, Horizontal { y: y_b }) => {
				if *y_a != *y_b {
					return 0;
				}
				let (bigger, smaller) = if self.length >= other.length {
					(self, other)
				} else {
					(other, self)
				};
				let (smaller_start, smaller_end) = smaller.ordered_points();
				let (bigger_start, bigger_end) = bigger.ordered_points();

				let intersects = if smaller_start.x > bigger_end.x || smaller_end.x < bigger_start.x {
					0
				} else if smaller_start.x < bigger_start.x {
					(smaller.length as i32) - (bigger_start.x - smaller_start.x)
				} else if smaller_end.x > bigger_end.x {
					(smaller.length as i32) - (bigger_end.x - smaller_end.x)
				} else {
					smaller.length as i32
				};

				if intersects > 0 {
					return intersects as u32;
				} else {
					return 0;
				}
			},
			(Horizontal { y: hor_y }, Vertical { x: ver_x }) => {
				println!("comparing horizontal and vertical lines");
				let (hor_start, hor_end) = self.ordered_points();
				let (ver_start, ver_end) = other.ordered_points();

				if ver_start.y < *hor_y && ver_end.y > *hor_y && hor_start.x < *ver_x && hor_end.x > *ver_x {
					println!("bounds overlap, 1 intersection");
					return 1;
				} else {
					println!("bounds don't overlap, no intersection");
					return 0;
				}
			},
			(Vertical { x: ver_x }, Horizontal { y: hor_y }) => {
				// println!("comparing horizontal and vertical lines");
				let (ver_start, ver_end) = self.ordered_points();
				let (hor_start, hor_end) = other.ordered_points();

				if ver_start.y < *hor_y && ver_end.y > *hor_y && hor_start.x < *ver_x && hor_end.x > *ver_x {
					// println!("bounds overlap, 1 intersection");
					return 1;
				} else {
					// println!("bounds don't overlap, no intersection");
					return 0;
				}
			},
			_ => {
				println!("{:?}", self);
				println!("{:?}", other);
				unimplemented!();
			},
		}
	}
}

fn part_one(lines: &Vec<Line>) -> u32 {
	let non_diag_lines: Vec<_> = lines.iter().filter(|line| {
		match &line.line_type {
			LineType::Horizontal { y: _ } | LineType::Vertical { x: _ } => true,
			_ => false
		}
	}).collect();

	for line in &non_diag_lines {
		println!("{:?}", line);
	} 

	let mut intersections = 0;

	for i in 0..non_diag_lines.len() {
		let line = &non_diag_lines[i];
		for j in (i+1)..non_diag_lines.len() {
			let other_line = &non_diag_lines[j];

			let i = line.intersections(&other_line);
			println!("A: {:?}", line);
			println!("B: {:?}", other_line);
			println!("intersects {} times\n", i);

			intersections += i;
		}
	}

	intersections
}

// fn part_two(lines: &Vec<Line>) -> u32 {
// 
// }

fn load_input() -> Vec<Line> {
	let input = include_str!("day5.txt");

	input
		.lines()
		.filter(|line_str| line_str.len() > 0)
		.map(|line_str| {
			let components = line_str
				.split(" -> ")
				.flat_map(|split_str| split_str.split(','))
				.map(|num_str| num_str.parse::<i32>().unwrap())
				.collect::<Vec<_>>();

			Line::new(Point { x: components[0], y: components[1] }, Point { x: components[2], y: components[3] })
		})
		.collect()
}

fn main() {
	let input = load_input();

	let part_one_answer = part_one(&input);
	// let part_two_answer = part_two(&input);

	println!("PART ONE ANSWER {}", part_one_answer);
	// println!("PART TWO ANSWER {}", part_two_answer);
}

#[cfg(test)]
mod tests {
	use super::*; 

	#[test]
	fn test_part_one_sample() {
		let lines = vec![
			Line::new(Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
			Line::new(Point { x: 8, y: 0 }, Point { x: 0, y: 8 }),
			Line::new(Point { x: 9, y: 4 }, Point { x: 3, y: 4 }),
			Line::new(Point { x: 2, y: 2 }, Point { x: 2, y: 1 }),
			Line::new(Point { x: 7, y: 0 }, Point { x: 7, y: 4 }),
			Line::new(Point { x: 6, y: 4 }, Point { x: 2, y: 0 }),
			Line::new(Point { x: 0, y: 9 }, Point { x: 2, y: 9 }),
			Line::new(Point { x: 3, y: 4 }, Point { x: 1, y: 4 }),
			Line::new(Point { x: 0, y: 0 }, Point { x: 8, y: 8 }),
			Line::new(Point { x: 5, y: 5 }, Point { x: 8, y: 2 }),
		];

		assert_eq!(part_one(&lines), 5);
	}

	#[test]
	fn horizontal_lines() {
		let line_one = Line::new(
			Point { x: 0, y: 9 }, Point { x: 5, y: 9 }
		);
		let line_two = Line::new(
			Point { x: 0, y: 9 }, Point { x: 2, y: 9 }
		);

		assert_eq!(line_one.intersections(&line_two), 3);
	}
}