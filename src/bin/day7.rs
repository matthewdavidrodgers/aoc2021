fn part_one(positions: &Vec<i32>) -> i32 {
    let min_best = positions.iter().min().unwrap();
    let max_best = positions.iter().max().unwrap();

    let mut best_movements: Option<i32> = None;

    for i in *min_best..*max_best {
        let mut movements = 0;
        positions.iter().for_each(|pos| {
            movements += if i > *pos { i - *pos } else { *pos - i }
        });
        if let Some(best) = best_movements {
            best_movements = if movements < best { Some(movements) } else { Some(best) };
        } else {
            best_movements = Some(movements);
        }
    }

    best_movements.unwrap() as i32
}

fn triangular_number(num: i32) -> i32 {
    (num * (num + 1)) / 2
}

fn part_two(positions: &Vec<i32>) -> i32 {
    let min_best = positions.iter().min().unwrap();
    let max_best = positions.iter().max().unwrap();

    let mut best_movements: Option<i32> = None;

    for i in *min_best..*max_best {
        let mut movements = 0;
        positions.iter().for_each(|pos| {
            movements += if i > *pos { triangular_number(i - *pos) } else { triangular_number(*pos - i) }
        });
        if let Some(best) = best_movements {
            best_movements = if movements < best { Some(movements) } else { Some(best) };
        } else {
            best_movements = Some(movements);
        }
    }

    best_movements.unwrap() as i32
}

fn load_input() -> Vec<i32> {
    let input = include_str!("day7.txt");

    input
        .lines()
        .filter(|line| line.len() > 0)
        .flat_map(|line| {
            line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn main() {
    let positions = load_input();

    let part_one_answer = part_one(&positions);
    let part_two_answer = part_two(&positions);

    println!("PART ONE ANSWER: {}", part_one_answer);
    println!("PART TWO ANSWER: {}", part_two_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        assert_eq!(part_one(&vec![16,1,2,0,4,2,7,1,2,14]), 37);
    }
}
