use std::collections::HashMap;

enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

struct DisplayDigit {
    number: u32,
    segments: Vec<Segment>,
}

#[derive(Debug)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
struct DigitRender {
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
                .map(|sig| DigitRender {
                    segments: sig.chars().map(match_char).collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>();

            let output_vals = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|val| DigitRender {
                    segments: val.chars().map(match_char).collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>();

            Entry {
                sig_patterns,
                output_vals,
            }
        })
        .collect()
}

fn load_display() -> HashMap<u32, Vec<Segment>> {
    use Segment::*;

    let display_map = HashMap::new();

    display_map.insert(0, vec![A, B, C, E, F, G]);
    display_map.insert(1, vec![C, F]);
    display_map.insert(2, vec![A, C, D, E, G]);
    display_map.insert(3, vec![A, C, D, F, G]);
    display_map.insert(4, vec![B, C, D, F]);
    display_map.insert(5, vec![A, B, D, F, G]);
    display_map.insert(6, vec![A, B, D, E, F, G]);
    display_map.insert(7, vec![A, C, F]);
    display_map.insert(8, vec![A, B, C, D, E, F, G]);
    display_map.insert(9, vec![A, B, C, D, F, G]);

    display_map
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

struct DecodingSignal<'a> {
    number: Option<u32>,
    sig_segments: &'a Vec<Segment>,
}

fn build_sig_to_disp_map(entry: &Entry, display: &HashMap<u32, Vec<Segment>>) -> HashMap<Segment, Segment> {
    let mut signals: Vec<_> = entry.sig_patterns.iter().map(|seg| {
        DecodingSignal { number: None, sig_segments: &seg }
    }).collect();

    let mut decode_map = HashMap::new();   

    let all_displays = vec![
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::D,
        Segment::E,
        Segment::F,
        Segment::G,
    ];

    decode_map.insert(Segment::A, all_displays.clone());
    decode_map.insert(Segment::B, all_displays.clone());
    decode_map.insert(Segment::C, all_displays.clone());
    decode_map.insert(Segment::D, all_displays.clone());
    decode_map.insert(Segment::E, all_displays.clone());
    decode_map.insert(Segment::F, all_displays.clone());
    decode_map.insert(Segment::G, all_displays.clone());

    for signal in &mut signals {
        match signal.segments.len() {
            2 => signal.number = Some(1),
            3 => signal.number = Some(7),
            4 => signal.number = Some(4),
            7 => signal.number = some(8),
        }
    }

    for signal in &mut signals {
        if let Some(num) = signal.number {
            let display_segments = display.get(num).unwrap();
            let mark_segments = all_displays
                .iter()
                .filter(|s| !display_segments.contains(s))
                .collect::<Vec<_>>();
            signal.segments = signal.segments.iter().filter(|seg| {
                !mark_segments.contains(seg)
            });
            for (sig, signals) in display {
                
            }
        }
    }
}

fn part_two(entries: &Vec<Entry>) -> u32 {
    let display_digits = load_display();
    
    0
}

fn main() {
    let input = load_input(include_str!("day8.txt"));

    let part_one_answer = part_one(&input);

    println!("PART ONE ANSWER: {}", part_one_answer);
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
}


// acedgfb  /       (8, abcdefg / )
// cdfbe    / ag    (5, abdfg / ce)
// gcdfa    / be    (2, acdeg / bf)
// fbcad    / eg    (3, acdfg / be)
// dab      / cefg  (7, acf     / bdeg)
// cefabd   / g
// cdfgeb   / a     (6, abdefg  / c)
// eafb     / cdg   (4, bcdf    / ag)
// cagedb   / f
// ab       / cdefg (1, cf      / abdeg)
//
//      a   b   c   d   e   f   g
//  a | x | x | O | x | x | x | x |
//  b | x | x | x | x | x | O | x |
//  c | x | x | x | x | x | x | O |
//  d | O | x | x | x | x | x | x |
//  e | x | O | x | x | x | x | x |
//  f | x | x | x | O | x | x | x |
//  g | x | x | x | x | O | x | x |
//
//
// for each known number (signal -> display)
// - for every signal, mark non-display
// - for every display, mark non-signal
// - find 6 - only len 6 that omits val in 7 that's not in 1
// - find 2 - only len 5 that omits f (known)
// - find 5 - only len 5 that has b (known)
// - find 3 - only other len 5
// - everything should be marked
//
//
//
//
//
//
//
//
//
//
// 
