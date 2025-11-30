use std::iter::zip;

use itertools::Itertools;
use pathfinding::num_traits::{abs, ToPrimitive};

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

pub fn part1(input: String) {
    let (left, right) = input
        .lines()
        .fold((vec![], vec![]), |(mut left, mut right), line| {
            let mut nums = parse_nums::<i64>(line);
            left.push(nums.next().expect("Left number not found"));
            right.push(nums.next().expect("Right number not found"));
            (left, right)
        });
    zip(left.iter().sorted(), right.iter().sorted())
        .map(|(l, r)| abs(l - r))
        .sum::<i64>()
        .print();
}

pub fn part2(input: String) {
    let (left, right) = input
        .lines()
        .fold((vec![], vec![]), |(mut left, mut right), line| {
            let mut nums = parse_nums::<u64>(line);
            left.push(nums.next().expect("Left number not found"));
            right.push(nums.next().expect("Right number not found"));
            (left, right)
        });
    let counts = right.iter().counts();
    left.iter()
        .map(|l| {
            *l * counts
                .get(l)
                .unwrap_or(&0)
                .to_u64()
                .expect("Could not convert to u64")
        })
        .sum::<u64>()
        .print();
}
