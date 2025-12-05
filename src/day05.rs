use std::{collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    let (top, bottom) = input.split_once("\n\n").expect("No blank line found");
    let ranges = top
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").expect("No - found");
            start.parse::<u64>().unwrap_or_default()..=end.parse::<u64>().unwrap_or_default()
        })
        .collect::<HashSet<_>>();
    bottom
        .lines()
        .flat_map(|line| line.parse::<u64>().ok())
        .filter(|ingredient| ranges.iter().any(|r| r.contains(ingredient)))
        .count()
        .print();
}

pub fn part2(input: String) {
    let (top, _) = input.split_once("\n\n").expect("No blank line found");
    let ranges = top
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").expect("No - found");
            start.parse::<u64>().unwrap_or_default()..=end.parse::<u64>().unwrap_or_default()
        })
        .collect::<HashSet<_>>();

    let mut combined = Vec::<RangeInclusive<u64>>::new();
    let mut last_end = 0;
    for x in ranges.iter().sorted_by_key(|r| r.start()) {
        if *x.start() <= last_end {
            if let Some(y) = combined.pop() {
                let max = *x.end().max(y.end());
                combined.push(*y.start()..=max);
                last_end = max;
            }
        } else {
            combined.push(x.clone());
            last_end = *x.end();
        }
    }

    combined
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<u64>()
        .print();
}
