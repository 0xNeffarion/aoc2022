pub fn solve() {
    println!("Day 1");
    part_one();
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

fn part_one() {
    let elfs = calories();
    let max = elfs
        .iter()
        .enumerate()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();

    println!("\nMax calories: {} | Elf: {}", *max.1, max.0)
}

fn part_two() {
    //let elfs = calories();

    // TODO
}
