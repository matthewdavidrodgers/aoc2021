fn _print_steps((mut pos_x, mut pos_y): (i32, i32), (mut vel_x, mut vel_y): (i32, i32)) {
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

struct Target {
    x: (i32, i32),
    y: (i32, i32),
}

fn load_input(input: &str) -> Target {
    let chars: Vec<_> = input.chars().collect();

    let chars = &chars[16..];
    let split = chars.split(", y=");
    let xs = split.next().unwrap().split("..");
    let ys = split.next().unwrap().split("..");

    let xmin: i32 = xs.next().unwrap().parse().unwrap();
    let xmax: i32 = xs.next().unwrap().parse().unwrap();

    Target { 
        x: (xs.next().unwrap
    }
}

fn tri_num(num: i32) -> i32 { (num * (num + 1)) / 2 }

fn determine_high_vel(target: &Target) -> (i32, i32) {
    let mut x = target.x.0 / 4;

    loop {
        let x_reached = tri_num(x);
        if (x_reached >= target.x.0 && x_reached <= target.x.1) {
            break;
        }
        x += 1;
    }

    let mut y = if target.y.1 < 0 { target.y.1 * -1 } else { target.y.1 };
    y -= 1;

    (x, y)
}

fn part_one(target: &Target) -> (i32, i32) {
    determine_high_vel(target)
}

fn main() {
    let target = Target { x: (20, 30), y: (-5, -10) };
    println!("{:?}", part_one(&target));
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }

