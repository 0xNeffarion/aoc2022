const ROCK: [char; 2] = ['A', 'X'];
const PAPER: [char; 2] = ['B', 'Y'];
const SCISSOR: [char; 2] = ['C', 'Z'];

const SCORE_ROCK: usize = 1;
const SCORE_PAPER: usize = 2;
const SCORE_SCISSOR: usize = 3;

enum Result {
    Win = 6,
    Lost = 0,
    Draw = 3,
}

const OUTCOME_LOSE: char = 'X';
const OUTCOME_WIN: char = 'Z';
const OUTCOME_DRAW: char = 'Y';

pub fn solve() {
    println!("Day 2");

    let strats = strategies();
    part_one(&strats);
    part_two(&strats);
}

fn strategies() -> Vec<(char, char)> {
    let input = include_str!("../../resources/day02.txt")
        .trim()
        .split('\n')
        .collect::<Vec<&str>>();

    let mut result = vec![];
    for line in input {
        let chars = line.chars().collect::<Vec<char>>();
        result.push((chars[0], chars[2]));
    }

    result
}

fn is_rock(c: &char) -> bool {
    ROCK.contains(c)
}

fn is_paper(c: &char) -> bool {
    PAPER.contains(c)
}

fn is_scissor(c: &char) -> bool {
    SCISSOR.contains(c)
}

fn game_outcome(input: &(char, char)) -> Result {
    let (opp, me) = input;

    if (is_rock(me) && is_rock(opp))
        || (is_paper(me) && is_paper(opp))
        || (is_scissor(me) && is_scissor(opp))
    {
        return Result::Draw;
    }

    if is_rock(me) && is_scissor(opp)
        || (is_scissor(me) && is_paper(opp))
        || (is_paper(me) && is_rock(opp))
    {
        return Result::Win;
    }

    Result::Lost
}

fn win(mv: &char) -> char {
    if is_scissor(mv) {
        return ROCK[0];
    } else if is_paper(mv) {
        return SCISSOR[0];
    }

    PAPER[0]
}

fn lose(mv: &char) -> char {
    if is_scissor(mv) {
        return PAPER[0];
    } else if is_paper(mv) {
        return ROCK[0];
    }

    SCISSOR[0]
}

fn move_score(mv: &char) -> usize {
    let move_score;

    if is_rock(mv) {
        move_score = SCORE_ROCK;
    } else if is_paper(mv) {
        move_score = SCORE_PAPER;
    } else {
        move_score = SCORE_SCISSOR;
    }

    move_score
}

fn predict_score(strats: &Vec<(char, char)>) -> usize {
    let mut total: usize = 0;
    for strat in strats {
        let outcome = game_outcome(strat);
        let game_score = move_score(&strat.1);
        total += game_score + (outcome as usize);
    }

    total
}

fn part_one(strats: &Vec<(char, char)>) {
    println!("Answer 1: ");
    let score = predict_score(strats);
    println!("Total score: {score}")
}

fn part_two(strats: &Vec<(char, char)>) {
    println!("Answer 2: ");

    let mut total = 0;
    for s in strats {
        let (opp, outcome) = s;

        match *outcome {
            OUTCOME_DRAW => {
                total += move_score(opp) + (Result::Draw as usize);
            }
            OUTCOME_LOSE => {
                let lose = lose(opp);
                total += move_score(&lose) + (Result::Lost as usize);
            }
            OUTCOME_WIN => {
                let win = win(opp);
                total += move_score(&win) + (Result::Win as usize);
            }
            _default => {}
        }
    }

    println!("Total score: {total}")
}
