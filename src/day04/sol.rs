use std::ops::Range;

struct Pair {
    start: u32,
    end: u32,
}

struct Assignment(Pair, Pair);

impl Pair {
    fn to_range(&self) -> Range<u32> {
        self.start..(self.end + 1)
    }
}

impl Assignment {
    fn overlaps_all(&self) -> bool {
        (self.0.start <= self.1.start && self.0.end >= self.1.end)
            || (self.1.start <= self.0.start && self.1.end >= self.0.end)
    }

    fn overlap_any(&self) -> bool {
        let r2 = self.1.to_range();

        for i in self.0.to_range() {
            if r2.contains(&i) {
                return true;
            }
        }

        false
    }
}

pub fn solve() {
    println!("Day 3");

    let assignments = pairs();
    part_one(&assignments);
    part_two(&assignments);
}

fn part_one(assignments: &[Assignment]) {
    let count = assignments.iter().filter(|&a| a.overlaps_all()).count();

    println!("Answer 1: {count}");
}

fn part_two(assignments: &[Assignment]) {
    let num = assignments.iter().filter(|x| x.overlap_any()).count();

    println!("Answer 2: {num}");
}

fn pairs() -> Vec<Assignment> {
    let mut result = vec![];
    let lines = include_str!("../../resources/day04.txt")
        .trim()
        .split('\n')
        .collect::<Vec<_>>();

    for s in lines {
        let exp = s.split(',').collect::<Vec<_>>();
        let first = exp[0];
        let second = exp[1];
        let range = first
            .trim()
            .split('-')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let pair1 = Pair {
            start: range[0],
            end: range[1],
        };
        let range = second
            .trim()
            .split('-')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let pair2 = Pair {
            start: range[0],
            end: range[1],
        };

        result.push(Assignment(pair1, pair2))
    }

    result
}
