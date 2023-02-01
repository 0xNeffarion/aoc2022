const OFFSET_LOWERCASE: u8 = 96;
const OFFSET_UPPERCASE: u8 = 64 - 26;

#[derive(Debug, Clone)]
struct Rucksack {
    number_compartments: usize,
    compartments: Vec<Vec<char>>,
}

impl Rucksack {
    fn new(number_compartments: usize, contents: &str) -> Self {
        let len = contents.len();

        let mut result: Vec<Vec<char>> = vec![];
        for i in 0..number_compartments {
            let start = (i * len) / number_compartments;
            let end = ((i + 1) * len) / number_compartments;

            result.push(contents[start..end].chars().collect());
        }

        Rucksack {
            number_compartments,
            compartments: result,
        }
    }

    fn sack(&self) -> Vec<char> {
        self.compartments.clone().into_iter().flatten().collect()
    }

    fn common_item(&self) -> Option<char> {
        for n1 in 0..self.number_compartments {
            let comp1 = &self.compartments[n1];

            for n2 in 1..self.number_compartments {
                let comp2 = &self.compartments[n2];
                if let Some(c) = comp1.iter().find(|&c| comp2.contains(c)) {
                    return Some(*c);
                }
            }
        }

        None
    }
}

fn group_badge(e1: &[char], e2: &[char], e3: &[char]) -> Option<char> {
    e1.iter()
        .find(|&a| e2.contains(a) && e3.contains(a))
        .copied()
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        return ((c as u8) - OFFSET_UPPERCASE) as u32;
    }

    (c as u8 - OFFSET_LOWERCASE) as u32
}

pub fn solve() {
    println!("Day 3");

    let rucksacks = rucksacks(2);

    part_one(&rucksacks);
    part_two(&rucksacks);
}

fn part_one(rucksacks: &[Rucksack]) {
    println!("Answer 1: ");

    let sum: u32 = rucksacks
        .iter()
        .map(|x| x.common_item())
        .filter(|x| x.is_some())
        .map(|x| priority(x.unwrap()))
        .sum();

    println!("Sum: {sum}")
}

fn part_two(rucksacks: &[Rucksack]) {
    println!("Answer 2: ");

    let mut sum = 0;
    for (i, _) in rucksacks.iter().enumerate().step_by(3) {
        let first = &rucksacks[i];
        let second = &rucksacks[i + 1];
        let third = &rucksacks[i + 2];

        let badge = group_badge(&first.sack(), &second.sack(), &third.sack());
        if let Some(x) = badge {
            sum += priority(x);
        }
    }

    println!("Sum: {sum}");
}

fn rucksacks(n: usize) -> Vec<Rucksack> {
    let input = include_str!("../../resources/day03.txt")
        .trim()
        .split('\n')
        .map(|x| Rucksack::new(n, x.trim()))
        .collect::<Vec<Rucksack>>();

    input
}
