fn _print_steps((mut pos_x, mut pos_y): (i64, i64), (mut vel_x, mut vel_y): (i64, i64)) {
    loop {
        println!("pos: ({}, {})\tvel: ({}, {}))", pos_x, pos_y, vel_x, vel_y);

        pos_x += vel_x;
        if vel_x != 0 { vel_x -= 1; }

        pos_y += vel_y;
        vel_y -= 1;

        if pos_y < -10 {
            break;
        }
    }
}

#[derive(Debug)]
struct Target {
    x: (i64, i64),
    y: (i64, i64),
}

fn load_input(input: &str) -> Target {
    let char_str: String = input.chars().skip(15).collect();

    let mut split = char_str.split(", y=");
    let mut xs = split.next().unwrap().split("..");
    let mut ys = split.next().unwrap().split("..");

    let xmin: i64 = xs.next().unwrap().parse().unwrap();
    let xmax: i64 = xs.next().unwrap().parse().unwrap();
    let ymax: i64 = ys.next().unwrap().parse().unwrap();
    let ymin: i64 = ys.next().unwrap().trim_end_matches("\n").parse().unwrap();

    Target { 
        x: (xmin, xmax),
        y: (ymin, ymax),
    }
}

fn tri_num(num: i64) -> i64 { (num * (num + 1)) / 2 }

fn derive_tri(num: i64) -> i64 {
    let squared = (8 * num) + 1;
    (((squared as f64).sqrt() as i64) + 1) / 2
}

fn determine_high_vel(target: &Target) -> (i64, i64) {
    // let mut x = target.x.0 / 4;
    let mut x = derive_tri(target.x.0);

    loop {
        let x_reached = tri_num(x);
        if x_reached >= target.x.0 && x_reached <= target.x.1 {
            break;
        }
        x += 1;
    }

    let mut y = if target.y.1 < 0 { target.y.1 * -1 } else { target.y.1 };
    y -= 1;

    (x, y)
}

fn part_one(target: &Target) -> i64 {
    let high_vel = determine_high_vel(target);
    tri_num(high_vel.1)
}

fn main() {
    let input = include_str!("day17.txt");
    let input = load_input(input);

    let part_one_answer = part_one(&input);

    println!("PART ONE ANSWER: {}", part_one_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let input = "target area: x=20..30, y=-10..-5";
        let input = load_input(input);

        assert_eq!(part_one(&input), 45);
    }
}

//      v, p
//  3 : 3, 0  2, 3  1, 5  0, 6
//  3 + 2 + 1 = 6
//  6 / 4 = 1
//
//  4 : 4, 0  3, 4  2, 7  1, 9  0,10
//  4 + 3 + 2 + 1 = 10
//  10 / 4 = 2
//
