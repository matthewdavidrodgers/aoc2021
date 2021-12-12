use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Revisitable {
    Always,
    Once,
    Never,
}

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    revisitable: Revisitable,
}

impl Cave {
    fn from_name(name: &str) -> Cave {
        Cave {
            name: String::from(name),
            revisitable: match name {
                "start" | "end" => Revisitable::Never,
                other => {
                    if other.chars().all(|c| c.is_uppercase()) {
                        Revisitable::Always
                    } else {
                        Revisitable::Once
                    }
                }
            },
        }
    }
}

#[derive(Debug)]
struct CaveSystem {
    caves: Vec<Cave>,
    start_index: usize,
    end_index: usize,
    paths_map: HashMap<String, Vec<usize>>,
}

impl CaveSystem {
    fn take_path(&self, curr_cave: usize, mut prev_visited: Vec<usize>) -> Vec<Vec<usize>> {
        if curr_cave == self.end_index {
            prev_visited.push(curr_cave);
            return vec![prev_visited];
        }

        self.paths_map
            .get(&self.caves[curr_cave].name)
            .unwrap()
            .iter()
            .filter(|cave_index| {
                let cave_is_revisitable = match self.caves[**cave_index].revisitable {
                    Revisitable::Always => true,
                    _ => false,
                };
                let cave_has_been_visited = prev_visited.contains(*cave_index);
                !cave_has_been_visited || cave_is_revisitable
            })
            .flat_map(|cave_index| {
                let mut next_prev_visited = prev_visited.clone();
                next_prev_visited.push(curr_cave);
                self.take_path(*cave_index, next_prev_visited)
            })
            .collect()
    }

    fn take_path_with_single_revisit(
        &self,
        curr_cave: usize,
        mut prev_visited: Vec<usize>,
    ) -> Vec<Vec<usize>> {
        if curr_cave == self.end_index {
            prev_visited.push(curr_cave);
            return vec![prev_visited];
        }

        let have_double_visited = self
            .caves
            .iter()
            .enumerate()
            .filter(|(_, cave)| match cave.revisitable {
                Revisitable::Once => true,
                _ => false,
            })
            .any(
                |(index, _)| match prev_visited.iter().filter(|i| **i == index).count() {
                    0 => false,
                    1 if curr_cave != index => false,
                    _ => true,
                },
            );

        self.paths_map
            .get(&self.caves[curr_cave].name)
            .unwrap()
            .iter()
            .filter(|cave_index| {
                let cave_is_revisitable = match self.caves[**cave_index].revisitable {
                    Revisitable::Always => true,
                    Revisitable::Once if !have_double_visited => true,
                    _ => false,
                };
                let cave_has_been_visited = prev_visited.contains(*cave_index);
                !cave_has_been_visited || cave_is_revisitable
            })
            .flat_map(|cave_index| {
                let mut next_prev_visited = prev_visited.clone();
                next_prev_visited.push(curr_cave);
                self.take_path_with_single_revisit(*cave_index, next_prev_visited)
            })
            .collect()
    }
}

fn load_input(input: &str) -> CaveSystem {
    let mut uniq_caves: Vec<Cave> = Vec::new();
    let pairs: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split: Vec<_> = line.split('-').collect();
            let name_a = split[0];
            let name_b = split[1];

            let cave_a = Cave::from_name(name_a);
            let cave_b = Cave::from_name(name_b);

            if uniq_caves.iter().all(|cave| cave.name != cave_a.name) {
                uniq_caves.push(cave_a.clone());
            }
            if uniq_caves.iter().all(|cave| cave.name != cave_b.name) {
                uniq_caves.push(cave_b.clone());
            }

            (cave_a, cave_b)
        })
        .collect();

    let mut paths_map = HashMap::new();

    for (cave_a, cave_b) in pairs {
        let cave_a_pos = uniq_caves
            .iter()
            .position(|c| c.name == cave_a.name)
            .unwrap();
        let cave_b_pos = uniq_caves
            .iter()
            .position(|c| c.name == cave_b.name)
            .unwrap();

        let cave_a_entry = paths_map.entry(cave_a.name).or_insert(Vec::new());
        cave_a_entry.push(cave_b_pos);

        let cave_b_entry = paths_map.entry(cave_b.name).or_insert(Vec::new());
        cave_b_entry.push(cave_a_pos);
    }

    let start_index = uniq_caves
        .iter()
        .position(|cave| cave.name == "start")
        .unwrap();
    let end_index = uniq_caves
        .iter()
        .position(|cave| cave.name == "end")
        .unwrap();

    CaveSystem {
        caves: uniq_caves,
        start_index,
        end_index,
        paths_map,
    }
}

fn part_one(cave_sys: &CaveSystem) -> u32 {
    let paths = cave_sys.take_path(cave_sys.start_index, vec![]);

    paths
        .iter()
        .filter(|path| path.len() > 0 && path[path.len() - 1] == cave_sys.end_index)
        .count() as _
}

fn part_two(cave_sys: &CaveSystem) -> u32 {
    let paths = cave_sys.take_path_with_single_revisit(cave_sys.start_index, vec![]);

    paths
        .iter()
        .filter(|path| path.len() > 0 && path[path.len() - 1] == cave_sys.end_index)
        .count() as _
}

fn main() {
    let input = include_str!("day12.txt");
    let input = load_input(input);

    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("PART ONE ANSWER {}", part_one_answer);
    println!("PART TWO ANSWER {}", part_two_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample_a() {
        let input = "
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let input = load_input(input);

        assert_eq!(part_one(&input), 10);
    }

    #[test]
    fn test_part_one_sample_b() {
        let input = "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let input = load_input(input);

        assert_eq!(part_one(&input), 19);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("day12.txt");
        let input = load_input(input);

        assert_eq!(part_one(&input), 5178);
    }

    #[test]
    fn test_part_two_sample_a() {
        let input = "
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let input = load_input(input);

        assert_eq!(part_two(&input), 36);
    }

    #[test]
    fn test_part_two_sample_b() {
        let input = "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let input = load_input(input);

        assert_eq!(part_two(&input), 103);
    }

    #[test]
    fn test_part_two_sample_c() {
        let input = "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let input = load_input(input);

        assert_eq!(part_two(&input), 3509);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day12.txt");
        let input = load_input(input);

        assert_eq!(part_two(&input), 130094);
    }
}
