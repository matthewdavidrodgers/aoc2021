#[derive(Debug)]
enum Mvmt {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn part_one(depths: &Vec<Mvmt>) -> i32 {
    let mut hor = 0;
    let mut ver = 0;

    for mvmt in depths {
        match mvmt {
            Mvmt::Forward(amt) => hor += amt,
            Mvmt::Up(amt) => ver -= amt,
            Mvmt::Down(amt) => ver += amt,
        }
    }

    hor * ver
}

fn part_two(depths: &Vec<Mvmt>) -> i32 {
    let mut hor = 0;
    let mut ver = 0;
    let mut aim = 0;

    for mvmt in depths {
        match mvmt {
            Mvmt::Forward(amt) => {
                hor += amt;
                ver += aim * amt;
            },
            Mvmt::Up(amt) => aim -= amt,
            Mvmt::Down(amt) => aim += amt,
        }
    }

    hor * ver
}

fn main() {
    let input = include_str!("day2.txt");

    let depths: Vec<_> = input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut split = line.split_whitespace();
            let dir = split.next().unwrap();
            let amt: i32 = split.next().unwrap().parse().unwrap();
            match dir {
                "forward" => Mvmt::Forward(amt),
                "up" => Mvmt::Up(amt),
                "down" => Mvmt::Down(amt),
                _ => panic!("invalid movement {}", dir),
            }
        })
        .collect();

    let part_one_ans = part_one(&depths);
    let part_two_ans = part_two(&depths);
    println!("PART ONE ANSWER: {}", part_one_ans);
    println!("PART TWO ANSWER: {}", part_two_ans);

    assert_eq!(part_one_ans, 2120749);
    assert_eq!(part_two_ans, 2138382217);
}
