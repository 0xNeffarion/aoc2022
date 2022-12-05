pub fn solve() {
    println!("Day 1");
    let mut calories = calories()
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, u32)>>();

    calories.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));

    part_one(&calories);
    part_two(&calories);
}

fn calories() -> Vec<u32> {
    let input = include_str!("../../resources/day01.txt").trim().split("\n");
    let mut index = 0;
    let mut elfs: Vec<u32> = vec![];
    for line in input {
        if line.trim().is_empty() {
            index += 1;
            continue;
        }

        let value = line.parse::<u32>().unwrap();
        if elfs.len() > index {
            elfs[index] = elfs[index] + value;
        } else {
            elfs.push(value);
        }
    }

    elfs
}

fn part_one(calories: &Vec<(usize, u32)>) {
    let len = calories.len();
    println!(
        "Answer 1: Elf: {} | Calories: {}\n",
        calories[len - 1].0,
        calories[len - 1].1
    )
}

fn part_two(calories: &Vec<(usize, u32)>) {
    let top_elves = 3;
    let len = calories.len();

    println!("Answer 2: ");
    println!("Top {} elves", top_elves);
    let mut sum = 0;
    for i in 0..top_elves {
        let value = calories[len - i - 1].1;
        let elf = calories[len - i - 1].0;
        sum += value;
        println!("Elf: {elf} | Value: {value}")
    }

    println!("Sum: {sum}")
}
