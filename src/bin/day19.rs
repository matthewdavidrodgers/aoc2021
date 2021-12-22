use std::collections::HashMap;

type Point = (i32, i32, i32);

fn load_input(input: &str) -> Vec<Vec<Point>> {
	let mut scanners = vec![];
	let mut current_scanner = vec![];

	let mut lines = input.lines();
	lines.next().unwrap();

	loop {
		if let Some(line) = lines.next() {
			if line.is_empty() {
				scanners.push(current_scanner);
				current_scanner = vec![];

				lines.next();
			} else {
				let mut split = line.split(',');
				current_scanner.push((
					split.next().unwrap().parse::<i32>().unwrap(),
					split.next().unwrap().parse::<i32>().unwrap(),
					split.next().unwrap().parse::<i32>().unwrap()
				));
			}
		} else {
			break;
		}
	}

	scanners.push(current_scanner);

	scanners
}

struct PointPair {
	a: Point,
	b: Point,
	dist: Point,
}


fn distance(a: &Point, b: &Point) -> Point {
	((a.0 - b.0).abs(), (a.1 - b.1).abs(), (a.2 - b.2).abs())
}

fn distances(scanner: &Vec<Point>) -> Vec<PointPair> {
	let mut dist = vec![];

	for x in 0..scanner.len() - 1 {
		for y in (x+1)..scanner.len() {
			let a = scanner[x];
			let b = scanner[y];

			dist.push(PointPair { a, b, dist: distance(&a, &b) });
		}
	}

	dist
}

fn build_point_map(scanner_a: &Vec<Point>, scanner_b: &Vec<Point>) -> HashMap<Point, Point> {
	let a_distances = distances(scanner_a);
	let b_distances = distances(scanner_b);

	let mut point_map: HashMap<Point, Point> = HashMap::new();

	for a_dist in &a_distances {
		match b_distances.iter().find(|d| d.dist == a_dist.dist) {
			Some(b_dist) => {
				match a_distances.iter().find(|d| {
					d.dist != a_dist.dist && (a_dist.a == d.a || a_dist.a == d.b)
				}) {
					Some(other_a_dist) => {
						match b_distances.iter().find(|d| d.dist == other_a_dist.dist) {
							Some(other_b_dist) => {
								if b_dist.a == other_b_dist.a || b_dist.a == other_b_dist.b {
									point_map.insert(a_dist.a, b_dist.a);
								} else if b_dist.b == other_b_dist.a || b_dist.b == other_b_dist.b {
									point_map.insert(a_dist.a, b_dist.b);
								} else {
									unreachable!();
								}
							},
							None => {},
						}
					},
					None => {},
				}

				match a_distances.iter().find(|d| {
					d.dist != a_dist.dist && (a_dist.b == d.a || a_dist.b == d.b)
				}) {
					Some(other_a_dist) => {
						match b_distances.iter().find(|d| d.dist == other_a_dist.dist) {
							Some(other_b_dist) => {
								if b_dist.a == other_b_dist.a || b_dist.a == other_b_dist.b {
									point_map.insert(a_dist.b, b_dist.a);
								} else if b_dist.b == other_b_dist.a || b_dist.b == other_b_dist.b {
									point_map.insert(a_dist.b, b_dist.b);
								} else {
									unreachable!();
								}
							},
							None => {},
						}
					},
					None => {},
				}
			},
			None => continue,
		}
	}

	point_map
}

fn main() {
	let input = include_str!("day19.txt");
	let input = load_input(input);
	
	println!("{:?}", build_point_map(&input[0], &input[1]));
}

#[cfg(test)]
mod tests {
	use super::*;

}