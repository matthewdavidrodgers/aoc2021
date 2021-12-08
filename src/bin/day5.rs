use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_on(&self, line: &Line) -> bool {
        let (line_start, line_end) = line.ordered_points();

        match &line.line_type {
            LineType::Vertical { x } => {
                *x == self.x && line_start.y <= self.y && line_end.y >= self.y
            }
            LineType::Horizontal { y } => {
                *y == self.y && line_start.x <= self.x && line_end.x >= self.x
            }
            LineType::Diagonal { m, b } => {
                let line_y_at_ax = if let Slope::Positive = m {
                    b + self.x
                } else {
                    b - self.x
                };
                line_y_at_ax == self.y && line_start.x <= self.x && line_end.x >= self.x
            }
        }
    }
}

#[derive(Debug)]
enum Slope {
    Positive,
    Negative,
}

impl Slope {
    fn to_scalar(&self) -> i32 {
        match self {
            Slope::Positive => 1,
            Slope::Negative => -1,
        }
    }
}

#[derive(Debug)]
enum LineType {
    Horizontal { y: i32 },
    Vertical { x: i32 },
    Diagonal { m: Slope, b: i32 },
}

#[derive(Debug)]
struct Line {
    point_a: Point,
    point_b: Point,
    line_type: LineType,
    length: u32,
}

impl Line {
    fn new(point_a: Point, point_b: Point) -> Line {
        use LineType::*;

        if point_a.x == point_b.x {
            let x = point_a.x;
            let length = point_b.y - point_a.y;
            let length = (length.abs() + 1) as u32;
            Line {
                point_a,
                point_b,
                line_type: Vertical { x },
                length,
            }
        } else if point_a.y == point_b.y {
            let y = point_a.y;
            let length = point_b.x - point_a.x;
            let length = (length.abs() + 1) as u32;
            Line {
                point_a,
                point_b,
                line_type: Horizontal { y },
                length,
            }
        } else {
            let m = (point_a.y - point_b.y) / (point_a.x - point_b.x);
            let b = point_a.y - (m * point_a.x);
            let length = ((point_a.x + point_b.x).pow(2)) + ((point_a.y + point_b.y).pow(2));
            let length = (length as f64).sqrt() as u32;
            let slope = if (point_a.x >= point_b.x) == (point_a.y >= point_b.y) {
                Slope::Positive
            } else {
                Slope::Negative
            };
            Line {
                point_a,
                point_b,
                line_type: Diagonal { m: slope, b },
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
            }
            _ => {
                if self.point_a.x <= self.point_b.x {
                    (&self.point_a, &self.point_b)
                } else {
                    (&self.point_b, &self.point_a)
                }
            }
        }
    }

    fn intersections_same_line(a: &Line, b: &Line, line_type: &LineType) -> Vec<Point> {
        let mut intersections = Vec::new();
        let (bigger, smaller) = if a.length >= b.length { (a, b) } else { (b, a) };
        let (smaller_start, _) = smaller.ordered_points();
        let (bigger_start, _) = bigger.ordered_points();

        match line_type {
            LineType::Vertical { x } => {
                let mut point = if smaller_start.y < bigger_start.y {
                    bigger_start.clone()
                } else {
                    smaller_start.clone()
                };

                while point.is_on(a) && point.is_on(b) {
                    let next_point = Point {
                        x: *x,
                        y: point.y + 1,
                    };
                    intersections.push(point);
                    point = next_point;
                }
            }
            LineType::Horizontal { y } => {
                let mut point = if smaller_start.x < bigger_start.x {
                    bigger_start.clone()
                } else {
                    smaller_start.clone()
                };

                while point.is_on(a) && point.is_on(b) {
                    let next_point = Point {
                        x: point.x + 1,
                        y: *y,
                    };
                    intersections.push(point);
                    point = next_point;
                }
            }
            LineType::Diagonal { m, .. } => {
                let mut point = if smaller_start.x <= bigger_start.x {
                    bigger_start.clone()
                } else {
                    smaller_start.clone()
                };

                while point.is_on(a) && point.is_on(b) {
                    let next_point = Point {
                        x: point.x + 1,
                        y: point.y + m.to_scalar(),
                    };
                    intersections.push(point);
                    point = next_point;
                }
            }
        };

        intersections
    }

    fn intersections(&self, other: &Line) -> Vec<Point> {
        use LineType::*;

        match (&self.line_type, &other.line_type) {
            (Vertical { x: a_val }, Vertical { x: b_val })
            | (Horizontal { y: a_val }, Horizontal { y: b_val }) => {
                if *a_val == *b_val {
                    Line::intersections_same_line(self, other, &self.line_type)
                } else {
                    vec![]
                }
            }
            (Diagonal { m: m_a, b: b_a }, Diagonal { m: m_b, b: b_b }) => match (m_a, m_b) {
                (Slope::Positive, Slope::Positive) | (Slope::Negative, Slope::Negative) => {
                    match b_a.cmp(&b_b) {
                        Ordering::Equal => {
                            Line::intersections_same_line(self, other, &self.line_type)
                        }
                        _ => vec![],
                    }
                }
                _ => {
                    let x = (*b_b - *b_a) / (m_a.to_scalar() - m_b.to_scalar());
                    let intersection = Point {
                        x,
                        y: (m_a.to_scalar() * x) + b_a,
                    };
                    if intersection.is_on(self) && intersection.is_on(other) {
                        vec![intersection]
                    } else {
                        vec![]
                    }
                }
            },
            (Horizontal { y }, Vertical { x }) | (Vertical { x }, Horizontal { y }) => {
                let intersection = Point { x: *x, y: *y };
                if intersection.is_on(self) && intersection.is_on(other) {
                    vec![intersection]
                } else {
                    vec![]
                }
            }
            (Diagonal { m, b }, Horizontal { y }) | (Horizontal { y }, Diagonal { m, b }) => {
                let intersection = Point {
                    x: (*y - *b) / m.to_scalar(),
                    y: *y,
                };
                if intersection.is_on(self) && intersection.is_on(other) {
                    vec![intersection]
                } else {
                    vec![]
                }
            }
            (Diagonal { m, b }, Vertical { x }) | (Vertical { x }, Diagonal { m, b }) => {
                let intersection = Point {
                    x: *x,
                    y: (m.to_scalar() * *x) + *b,
                };
                if intersection.is_on(self) && intersection.is_on(other) {
                    vec![intersection]
                } else {
                    vec![]
                }
            }
        }
    }
}

fn part_one(lines: &Vec<Line>) -> u32 {
    let non_diag_lines: Vec<_> = lines
        .iter()
        .filter(|line| match &line.line_type {
            LineType::Horizontal { y: _ } | LineType::Vertical { x: _ } => true,
            _ => false,
        })
        .collect();

    let mut intersections: HashMap<Point, u32> = HashMap::new();

    for i in 0..non_diag_lines.len() {
        let line = &non_diag_lines[i];
        for j in (i + 1)..non_diag_lines.len() {
            let other_line = &non_diag_lines[j];

            for intersection in line.intersections(&other_line) {
                let intersection_record = intersections.entry(intersection).or_insert(0);
                *intersection_record += 1;
            }
        }
    }

    intersections.len() as u32
}

fn part_two(lines: &Vec<Line>) -> u32 {
    let mut intersections: HashMap<Point, u32> = HashMap::new();

    for i in 0..lines.len() {
        let line = &lines[i];
        for j in (i + 1)..lines.len() {
            let other_line = &lines[j];

            for intersection in line.intersections(&other_line) {
                let intersection_record = intersections.entry(intersection).or_insert(0);
                *intersection_record += 1;
            }
        }
    }

    intersections.len() as u32
}

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

            Line::new(
                Point {
                    x: components[0],
                    y: components[1],
                },
                Point {
                    x: components[2],
                    y: components[3],
                },
            )
        })
        .collect()
}

fn main() {
    let input = load_input();

    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("PART ONE ANSWER {}", part_one_answer);
    println!("PART TWO ANSWER {}", part_two_answer);
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
    fn test_part_two_sample() {
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

        assert_eq!(part_two(&lines), 12);
    }

    #[test]
    fn horizontal_lines() {
        let line_one = Line::new(Point { x: 0, y: 9 }, Point { x: 5, y: 9 });
        let line_two = Line::new(Point { x: 0, y: 9 }, Point { x: 2, y: 9 });

        assert_eq!(
            line_one.intersections(&line_two),
            vec![
                Point { x: 0, y: 9 },
                Point { x: 1, y: 9 },
                Point { x: 2, y: 9 },
            ]
        );
    }

    #[test]
    fn test_is_on_line() {
        let hor_line = Line::new(Point { x: 0, y: 9 }, Point { x: 5, y: 9 });
        let point = Point { x: 1, y: 9 };

        assert_eq!(point.is_on(&hor_line), true);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&load_input()), 7297);
    }
}
