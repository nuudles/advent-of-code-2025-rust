use std::collections::{BTreeMap, BTreeSet};

use crate::point::Point;

pub fn part1(input: String) {
    let mut tachyons = BTreeSet::new();
    let mut splitters = BTreeSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                tachyons.insert(Point { x, y });
            } else if c == '^' {
                splitters.insert(Point { x, y });
            }
        }
    }

    let mut splits = 0;
    for _ in 0..input.lines().count() {
        let mut next_tachyons = BTreeSet::new();

        for tachyon in tachyons {
            let next = tachyon.down();
            if splitters.contains(&next) {
                next_tachyons.insert(next.left());
                next_tachyons.insert(next.right());
                splits += 1;
            } else {
                next_tachyons.insert(next);
            }
        }

        tachyons = next_tachyons;
    }
    println!("{}", splits);
}

/*
// Brute Force :D
pub fn part2(input: String) {
    let mut timelines = BTreeSet::new();
    let mut splitters = BTreeSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                let tachyons = BTreeSet::from([Point { x, y }]);
                timelines.insert(tachyons);
            } else if c == '^' {
                splitters.insert(Point { x, y });
            }
        }
    }

    for y in 0..input.lines().count() {
        println!("{}: {}", y, timelines.len());
        let mut next_timelines = BTreeSet::new();

        for timeline in timelines {
            for tachyon in timeline.iter().filter(|t| t.y == y) {
                let next = tachyon.down();
                if splitters.contains(&next) {
                    let mut left_timeline = timeline.clone();
                    left_timeline.insert(next.left());
                    let mut right_timeline = timeline.clone();
                    right_timeline.insert(next.right());
                    next_timelines.insert(left_timeline);
                    next_timelines.insert(right_timeline);
                } else {
                    let mut next_timeline = timeline.clone();
                    next_timeline.insert(next);
                    next_timelines.insert(next_timeline);
                }
            }
        }

        timelines = next_timelines;
    }
    println!("{}", timelines.len());
}
 */

pub fn part2(input: String) {
    let mut tachyons = BTreeMap::new();
    let mut splitters = BTreeSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                tachyons.insert(Point { x, y }, 1);
            } else if c == '^' {
                splitters.insert(Point { x, y });
            }
        }
    }

    for _ in 0..input.lines().count() {
        let mut next_tachyons = BTreeMap::new();

        for (tachyon, timelines) in tachyons {
            let next = tachyon.down();
            if splitters.contains(&next) {
                *next_tachyons.entry(next.left()).or_default() += timelines;
                *next_tachyons.entry(next.right()).or_default() += timelines;
            } else {
                *next_tachyons.entry(next).or_default() += timelines;
            }
        }

        tachyons = next_tachyons;
    }
    println!("{:?}", tachyons.values().sum::<u64>());
}
