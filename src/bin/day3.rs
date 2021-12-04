fn part_one(report: &Vec<u16>) -> u32 {
    let entries = report.len() as u32;
    let mut ones_count = [0u32; 12];

    for value in report {
        for i in (0..12).rev() {
            let mask = 0x1 << i;
            if value & mask != 0 {
                ones_count[i] += 1
            }
        }
    }

    let mut gamma = 0u16;
    for i in 0..ones_count.len() {
        let count = ones_count[i];
        if count > (entries / 2) {
            let mask = 0x1 << i;
            gamma |= mask;
        }
    }

    let epsilon = gamma ^ 0xF_FF;

    (gamma as u32) * (epsilon as u32)
}

#[derive(PartialEq)]
enum PickPartition {
    Greater,
    Lesser,
}

fn partition_until(report: &Vec<u16>, pick_partition: PickPartition) -> u32 {
    let mut curr_position = 11;

    let mut curr_report = report.clone();

    // could infinitely loop? maybe?
    while curr_report.len() > 1 {
        let mut ones = Vec::new();
        let mut zeroes = Vec::new();

        for value in &curr_report {
            let mask = 0x1 << curr_position;
            if value & mask != 0 {
                ones.push(*value);
            } else {
                zeroes.push(*value);
            }
        }

        let greater;
        let lesser;
        if ones.len() >= curr_report.len() / 2 {
            greater = ones;
            lesser = zeroes;
        } else {
            greater = zeroes;
            lesser = ones;
        }

        curr_report = if pick_partition == PickPartition::Greater {
            greater
        } else {
            lesser
        };

        if curr_position == 0 {
            curr_position = 11;
        } else {
            curr_position -= 1;
        }
    }

    curr_report[0] as u32 // will panic if nothing is found, that's what i want
}

fn part_two(report: &Vec<u16>) -> u32 {
    let oxygen = partition_until(report, PickPartition::Greater);
    let co2 = partition_until(report, PickPartition::Lesser);

    oxygen * co2
}

fn load_input() -> Vec<u16> {
    let input = include_str!("day3.txt");

    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect()
}

fn main() {
    let report = load_input();

    let part_one_ans = part_one(&report);
    let part_two_ans = part_two(&report);

    println!("PART ONE ANSWER: {}", part_one_ans);
    println!("PART TWO ANSWER: {}", part_two_ans);
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&load_input), 2967914);
    }
}
