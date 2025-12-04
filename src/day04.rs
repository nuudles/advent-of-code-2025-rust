use std::collections::HashSet;

use crate::{point::Point, selfprint::SelfPrint};

fn is_accessible(p: &Point<i64>, rolls: &HashSet<Point<i64>>) -> bool {
    p.neighbors_with_diagonals()
        .iter()
        .filter(|n| rolls.contains(*n))
        .count()
        < 4
}

pub fn part1(input: String) {
    let mut rolls = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                rolls.insert(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    rolls
        .iter()
        .filter(|p| is_accessible(*p, &rolls))
        .count()
        .print();
}

pub fn part2(input: String) {
    let mut rolls = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                rolls.insert(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    let mut count = 0;
    loop {
        let mut next_rolls = rolls.clone();
        for p in rolls.iter().filter(|p| is_accessible(*p, &rolls)) {
            next_rolls.remove(p);
        }

        if next_rolls.len() < rolls.len() {
            count += rolls.len() - next_rolls.len();
            rolls = next_rolls;
        } else {
            break;
        }
    }
    println!("{}", count);
}
