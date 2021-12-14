#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, Clone)]
struct Grid {
    points: Vec<usize>,
    x_len: usize,
    y_len: usize,
}

impl Grid {
    fn print_grid(&self) {
        for y in 0..self.y_len + 1 {
            for x in 0..self.x_len + 1 {
                if self.points.contains(&((y * self.x_len) + x)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        print!("\n");
    }

    fn print_grid_size(&self, x_size: usize, y_size: usize) {
        for y in 0..y_size {
            for x in 0..x_size {
                if self.points.contains(&((y * self.x_len) + x)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        print!("\n");
    }

    fn insert_point(&mut self, point: usize) {
        if !self.points.contains(&point) {
            self.points.push(point);
        }
    }

    fn perform_fold(&mut self, fold: &Fold) {
        let mut added_points: Vec<usize> = vec![];
        match fold {
            Fold::X(axis) => {
                self.points.retain(|point| {
                    let point_y = *point / self.x_len;
                    let point_x = *point % self.x_len;

                    if point_x >= *axis {
                        let new_x = *axis - (point_x - *axis);
                        if new_x < *axis {
                            let new_pos = (point_y * self.x_len) + new_x;
                            added_points.push(new_pos);
                        }
                        return false; // remove this point
                    }
                    return true; // keep it
                });
            }
            Fold::Y(axis) => {
                self.points.retain(|point| {
                    let point_y = *point / self.x_len;
                    let point_x = *point % self.x_len;

                    if point_y >= *axis {
                        let new_y = *axis - (point_y - *axis);
                        if new_y < *axis {
                            let new_pos = (new_y * self.x_len) + point_x;
                            added_points.push(new_pos);
                        }
                        return false; // remove this point
                    }
                    return true; // keep it
                });
            }
        };

        for point in added_points {
            self.insert_point(point);
        }
    }
}

fn load_input(input: &str) -> (Grid, Vec<Fold>) {
    let mut points: Vec<(usize, usize)> = vec![];
    let mut folds: Vec<Fold> = vec![];

    let mut lines = input.lines();
    let mut line = lines.next().unwrap();

    while !line.is_empty() {
        let mut split = line.split(',');

        let x: usize = split.next().unwrap().parse().unwrap();
        let y: usize = split.next().unwrap().parse().unwrap();

        points.push((x, y));

        line = lines.next().unwrap();
    }

    for fold_line in lines {
        if !fold_line.is_empty() {
            let chars: Vec<_> = fold_line.chars().collect();

            assert_eq!(chars[0..11].iter().collect::<String>(), "fold along ");
            assert_eq!(chars[12], '=');

            let value: usize = chars[13..].iter().collect::<String>().parse().unwrap();

            folds.push(match chars[11] {
                'x' => Fold::X(value),
                'y' => Fold::Y(value),
                _ => panic!("bad axis"),
            });
        }
    }

    let (xs, ys): (Vec<_>, Vec<_>) = points.iter().cloned().unzip();

    let x_len: usize = *xs.iter().max().unwrap() + 1;
    let y_len: usize = *ys.iter().max().unwrap();

    let grid = Grid {
        x_len,
        y_len,
        points: points
            .iter()
            .map(|(x, y)| (y * x_len) + x)
            .collect::<Vec<_>>(),
    };

    (grid, folds)
}

fn part_one(mut grid: Grid, fold: &Fold) -> usize {
    grid.perform_fold(fold);

    grid.points.len()
}

fn part_two(mut grid: Grid, folds: &Vec<Fold>) {
    for fold in folds {
        grid.perform_fold(fold);
    }

    grid.print_grid_size(80, 40);
}

fn main() {
    let input = include_str!("day13.txt");
    let (grid, folds) = load_input(input);

    let part_one_answer = part_one(grid.clone(), &folds[0]);
    part_two(grid.clone(), &folds);

    println!("PART ONE ANSWER: {}", part_one_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let (grid, folds) = load_input(input);

        assert_eq!(part_one(grid, &folds[0]), 17);
    }

    #[test]
    fn test_full_part_one_sample() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let (mut grid, folds) = load_input(input);

        println!("Before");
        grid.print_grid();

        grid.perform_fold(&folds[0]);
        println!("First Fold");
        grid.print_grid();

        grid.perform_fold(&folds[1]);
        println!("Second Fold");
        grid.print_grid();

        assert_eq!(grid.points.len(), 16);
    }
}
