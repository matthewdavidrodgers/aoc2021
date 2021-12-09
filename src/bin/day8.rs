use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DigitRender {
    segment_str: String,
    segments: Vec<Segment>,
}

#[derive(Debug)]
struct Entry {
    sig_patterns: Vec<DigitRender>,
    output_vals: Vec<DigitRender>,
}

fn load_input(input: &str) -> Vec<Entry> {
    let match_char = |c| match c {
        'a' => Segment::A,
        'b' => Segment::B,
        'c' => Segment::C,
        'd' => Segment::D,
        'e' => Segment::E,
        'f' => Segment::F,
        'g' => Segment::G,
        _ => panic!("bad char"),
    };

    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut split = line.split('|');
            let sig_patterns = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|sig| {
                    let mut chars: Vec<_> = sig.chars().collect();
                    chars.sort_by(|a, b| a.cmp(b));

                    let segment_str = String::from_iter(chars.iter());

                    DigitRender {
                        segment_str,
                        segments: chars.into_iter().map(match_char).collect::<Vec<_>>(),
                    }
                })
                .collect::<Vec<_>>();

            let output_vals = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|val| {
                    let mut chars: Vec<_> = val.chars().collect();
                    chars.sort_by(|a, b| a.cmp(b));

                    let segment_str = String::from_iter(chars.iter());

                    DigitRender {
                        segment_str,
                        segments: chars.into_iter().map(match_char).collect::<Vec<_>>(),
                    }
                })
                .collect::<Vec<_>>();

            Entry {
                sig_patterns,
                output_vals,
            }
        })
        .collect()
}

fn part_one(entries: &Vec<Entry>) -> u32 {
    entries
        .iter()
        .flat_map(|entry| &entry.output_vals)
        .filter(|out_val| match out_val.segments.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count() as u32
}

fn build_sig_map(entry: &Entry) -> HashMap<String, u32> {
    let mut sig_map = HashMap::new();

    let mut sig_seven = None;
    for signal in &entry.sig_patterns {
        match signal.segments.len() {
            2 => {
                sig_map.insert(signal.segment_str.clone(), 1);
            }
            3 => {
                sig_seven = Some(signal);
                sig_map.insert(signal.segment_str.clone(), 7);
            }
            4 => {
                sig_map.insert(signal.segment_str.clone(), 4);
            }
            7 => {
                sig_map.insert(signal.segment_str.clone(), 8);
            }
            _ => (),
        }
    }
    let mut signals = entry.sig_patterns.clone();

    let sig_seven = sig_seven.unwrap();
    signals.retain(|sig| !sig_map.contains_key(&sig.segment_str));

    let i = signals
        .iter()
        .position(|sig| {
            sig.segments.len() == 6
                && sig_seven
                    .segments
                    .iter()
                    .any(|sig_seg| !sig.segments.contains(sig_seg))
        })
        .unwrap();
    let sig_six = signals.swap_remove(i);
    sig_map.insert(sig_six.segment_str.clone(), 6);

    let i = signals
        .iter()
        .position(|sig| {
            sig.segments.len() == 5
                && sig
                    .segments
                    .iter()
                    .all(|sig_seg| sig_six.segments.contains(sig_seg))
        })
        .unwrap();
    let sig_five = signals.swap_remove(i);
    sig_map.insert(sig_five.segment_str.clone(), 5);

    let i = signals
        .iter()
        .position(|sig| {
            sig.segments.len() == 6
                && sig_five
                    .segments
                    .iter()
                    .all(|sig_seg| sig.segments.contains(sig_seg))
        })
        .unwrap();
    let sig_nine = signals.swap_remove(i);
    sig_map.insert(sig_nine.segment_str.clone(), 9);

    let i = signals
        .iter()
        .position(|sig| {
            sig.segments.len() == 5
                && sig_seven
                    .segments
                    .iter()
                    .all(|sig_seg| sig.segments.contains(sig_seg))
        })
        .unwrap();
    let sig_three = signals.swap_remove(i);
    sig_map.insert(sig_three.segment_str.clone(), 3);

    let i = signals
        .iter()
        .position(|sig| sig.segments.len() == 5)
        .unwrap();
    let sig_two = signals.swap_remove(i);
    sig_map.insert(sig_two.segment_str.clone(), 2);

    let sig_zero = signals.swap_remove(0);
    sig_map.insert(sig_zero.segment_str.clone(), 0);

    sig_map
}

fn part_two(entries: &Vec<Entry>) -> u32 {
    let mut sum = 0;

    for entry in entries {
        let map = build_sig_map(entry);
        let mut entry_sum = 0;
        for (i, output_val) in entry.output_vals.iter().enumerate() {
            if let Some(num) = map.get(&output_val.segment_str) {
                let pow = entry.output_vals.len() - i - 1;
                entry_sum += num * 10u32.pow(pow as u32);
            } else {
                panic!("Could not resolve number");
            }
        }
        sum += entry_sum;
    }

    sum
}

fn main() {
    let input = load_input(include_str!("day8.txt"));

    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("PART ONE ANSWER: {}", part_one_answer);
    println!("PART TWO ANSWER: {}", part_two_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let input = load_input(input);

        assert_eq!(part_one(&input), 26);
    }

    #[test]
    fn test_part_one() {
        let input = load_input(include_str!("day8.txt"));
        assert_eq!(part_one(&input), 387);
    }

    #[test]
    fn test_build_sig_map() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let input = load_input(input);

        let map = build_sig_map(&input[0]);
        assert_eq!(map.get("abcdefg"), Some(&8));
        assert_eq!(map.get("bcdef"), Some(&5));
        assert_eq!(map.get("acdfg"), Some(&2));
        assert_eq!(map.get("abcdf"), Some(&3));
        assert_eq!(map.get("abd"), Some(&7));
        assert_eq!(map.get("abcdef"), Some(&9));
        assert_eq!(map.get("bcdefg"), Some(&6));
        assert_eq!(map.get("abef"), Some(&4));
        assert_eq!(map.get("abcdeg"), Some(&0));
        assert_eq!(map.get("ab"), Some(&1));
    }

    #[test]
    fn test_part_two_single_sample() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let input = load_input(input);

        assert_eq!(part_two(&input), 5353);
    }

    #[test]
    fn test_part_two_sample() {
        let input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let input = load_input(input);

        assert_eq!(part_two(&input), 61229);
    }
}
