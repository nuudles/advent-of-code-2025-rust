use itertools::Itertools;
use regex::Regex;

use crate::selfprint::SelfPrint;

fn is_invalid(id: u64) -> bool {
    let length = id.checked_ilog10().unwrap_or_default() + 1;
    if length % 2 == 1 {
        return false;
    }
    let half = 10u64.pow(length / 2);
    id / half == id % half
}

pub fn part1(input: String) {
    Regex::new(r"\d+")
        .expect("Invalid Regex")
        .find_iter(input.lines().next().expect("No line found"))
        .flat_map(|m| m.as_str().parse().ok())
        .tuples()
        .flat_map(|(start, end)| (start..=end).filter(|n| is_invalid(*n)))
        .sum::<u64>()
        .print();
}

fn is_silly(id: u64) -> bool {
    let length = id.checked_ilog10().unwrap_or_default() + 1;
    'outer: for i in 1..=length / 2 {
        if length % i != 0 {
            continue;
        }
        let divisor = 10u64.pow(i);
        let needle = id % divisor;
        let mut haystack = id / divisor;

        for _ in 0..(length / i) - 1 {
            if haystack % divisor != needle {
                continue 'outer;
            }
            haystack /= divisor;
        }
        return true;
    }
    false
}

pub fn part2(input: String) {
    Regex::new(r"\d+")
        .expect("Invalid Regex")
        .find_iter(input.lines().next().expect("No line found"))
        .flat_map(|m| m.as_str().parse().ok())
        .tuples()
        .flat_map(|(start, end)| (start..=end).filter(|n| is_silly(*n)))
        .sum::<u64>()
        .print();
}
