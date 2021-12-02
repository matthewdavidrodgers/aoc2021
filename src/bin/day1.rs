struct SummingRangedVec<'a> {
    vec: &'a Vec<i32>,
    index: usize,
}

impl<'a> SummingRangedVec<'a> {
    fn new(vec: &'a Vec<i32>) -> SummingRangedVec<'a> {
        SummingRangedVec { vec, index: 0 }
    }
}

impl<'a> Iterator for SummingRangedVec<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() - 2 {
            let sum = self.vec[self.index] + self.vec[self.index + 1] + self.vec[self.index + 2];
            self.index += 1;
            Some(sum)
        } else {
            None
        }
    }
}

fn part_one(depths: &Vec<i32>) -> u32 {
    let mut increases = 0;
    let mut depths_iter = depths.iter();
    let mut prev = depths_iter.next().unwrap();

    for curr in depths_iter {
        if curr > prev {
            increases += 1;
        }
        prev = curr;
    }

    increases
}

fn part_two(depths: &Vec<i32>) -> u32 {
    let mut increases = 0;
    let mut depths_iter = SummingRangedVec::new(depths);
    let mut prev = depths_iter.next().unwrap();

    for curr in depths_iter {
        if curr > prev {
            increases += 1;
        }
        prev = curr;
    }

    increases
}

fn main() {
    let input = include_str!("day1.txt");

    let depths: Vec<_> = input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let part_one_ans = part_one(&depths);
    let part_two_ans = part_two(&depths);
    println!("PART 1: {} increases", part_one_ans);
    println!("PART 2: {} increases", part_two_ans);

    assert_eq!(part_one_ans, 1451);
    assert_eq!(part_two_ans, 1395);
}
