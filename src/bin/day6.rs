#[derive(Debug, Copy, Clone)]
struct LFish {
    timer: u32,
}

impl LFish {
    fn spawn() -> LFish {
        LFish { timer: 8 }
    }
}

fn part_one(mut fish: Vec<LFish>, days: u32) -> u32 {
    for _ in 0..days {
        let mut next_fish = Vec::new();
        for f in &mut fish {
            if f.timer == 0 {
                next_fish.push(LFish::spawn());
                f.timer = 6;
            } else {
                f.timer -= 1;
            }
        }

        fish.append(&mut next_fish);
    }

    fish.len() as u32
}

fn load_input() -> Vec<LFish> {
    let input = include_str!("day6.txt");

    input
        .lines()
        .flat_map(|line| {
            line.split(',')
                .filter(|fish_str| fish_str.len() > 0)
                .map(|fish_str| {
                    let timer: u32 = fish_str.parse().unwrap();
                    LFish { timer }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn main() {
    let initial_fish = load_input();

    let part_one_answer = part_one(initial_fish.clone(), 80);

    println!("PART ONE ANSWER {}", part_one_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let fish = vec![
            LFish { timer: 3 },
            LFish { timer: 4 },
            LFish { timer: 3 },
            LFish { timer: 1 },
            LFish { timer: 2 },
        ];

        assert_eq!(part_one(fish.clone(), 18), 26);
        assert_eq!(part_one(fish.clone(), 80), 5934);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(load_input(), 80), 352872);
    }
}
