use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

type Rule = ((char, char), char);

fn load_input(input: &str) -> (Vec<char>, Vec<Rule>) {
    let mut lines = input.lines();

    let template: Vec<_> = lines.next().unwrap().chars().collect();

    lines.next().unwrap();

    let rules: Vec<_> = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            let chars: Vec<_> = line.chars().collect();

            let pair = (chars[0], chars[1]);
            let production = chars[6];

            (pair, production)
        })
        .collect();

    (template, rules)
}

struct PairingCharVec<'a> {
    vec: &'a Vec<char>,
    index: usize,
}

impl<'a> PairingCharVec<'a> {
    fn new(vec: &'a Vec<char>) -> PairingCharVec<'a> {
        PairingCharVec { vec, index: 0 }
    }
}

impl<'a> Iterator for PairingCharVec<'a> {
    type Item = (char, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() - 1 {
            let item = (self.vec[self.index], self.vec[self.index + 1]);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn perform_step(template: Vec<char>, rules: &Vec<Rule>) -> Vec<char> {
    let pairing_iter = PairingCharVec::new(&template);

    let mut mapped: Vec<_> = pairing_iter
        .flat_map(
            |(a, b)| match rules.iter().find(|((r_a, r_b), _)| *r_a == a && *r_b == b) {
                Some(((_, _), prod)) => {
                    vec![a, *prod]
                }
                None => {
                    vec![a]
                }
            },
        )
        .collect();

    mapped.push(template[template.len() - 1]);

    mapped
}

fn part_one(template: &Vec<char>, rules: &Vec<Rule>, steps: usize) -> usize {
    let mut polymer = template.clone();

    for step in 0..steps {
        polymer = perform_step(polymer, rules);
    }

    let mut counts = HashMap::new();
    for c in &polymer {
        let c_count = counts.entry(c).or_insert(0);
        *c_count += 1;
    }

    let mut max: Option<usize> = None;
    let mut min: Option<usize> = None;
    for (_, count) in counts {
        if let Some(max_size) = max {
            if count > max_size {
                max = Some(count);
            }
        } else {
            max = Some(count);
        }

        if let Some(min_size) = min {
            if count < min_size {
                min = Some(count);
            }
        } else {
            min = Some(count);
        }
    }

    max.unwrap() - min.unwrap()
}

struct MemoedRule {
    rule: Rule,
    after_steps: usize,
    produces: Vec<char>,
    counts: HashMap<char, usize>,
}

fn build_productions_to(rules: &Vec<Rule>, steps: usize) -> Vec<MemoedRule> {
    rules
        .iter()
        .map(|rule| {
            let mut produces = vec![rule.0 .0, rule.0 .1];
            for i in 0..steps {
                produces = perform_step(produces, rules);
            }
            let mut counts = HashMap::new();
            for c in &produces {
                let c_count = counts.entry(*c).or_insert(0);
                *c_count += 1;
            }

            println!("produced for rule {:?}", rule);

            MemoedRule {
                rule: *rule,
                after_steps: steps,
                produces,
                counts,
            }
        })
        .collect()
}

fn part_two(template: &Vec<char>, rules: &Vec<Rule>, steps: usize) -> usize {
    unimplemented!();
}

fn main() {
    let input = include_str!("day14.txt");
    let (template, rules) = load_input(input);

    let part_one_answer = part_one(&template, &rules, 10);
    let part_two_answer = part_one(&template, &rules, 40);

    println!("PART ONE ANSWER: {}", part_one_answer);
    println!("PART TWO ANSWER: {}", part_two_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let (template, rules) = load_input(input);

        assert_eq!(part_one(&template, &rules, 10), 1588);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("day14.txt");
        let (template, rules) = load_input(input);

        assert_eq!(part_one(&template, &rules, 10), 3587);
    }

    #[test]
    fn blah() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let (template, rules) = load_input(input);

        let memoed_rules = build_productions_to(&rules, 20);
        for r in memoed_rules {
            println!(
                "{:?} | N: {}\tB: {}\tC: {}\tH: {}",
                r.rule,
                r.counts.get(&'N').unwrap_or(&0),
                r.counts.get(&'B').unwrap_or(&0),
                r.counts.get(&'C').unwrap_or(&0),
                r.counts.get(&'H').unwrap_or(&0)
            );
        }
    }
}
