use std::collections::VecDeque;

use itertools::Itertools;

use crate::parse_nums::parse_nums;

pub fn part1(input: String) {
    let count = input.lines().count();
    let numbers = input
        .lines()
        .take(count - 1)
        .map(|l| parse_nums::<u64>(l).collect_vec())
        .collect_vec();

    let mut total = 0;
    for (index, symbol) in input
        .lines()
        .last()
        .expect("Symbol line not found")
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .enumerate()
    {
        let is_sum = symbol == "+";
        let mut result = if is_sum { 0 } else { 1 };

        for vec in &numbers {
            if is_sum {
                result += vec[index];
            } else {
                result *= vec[index];
            }
        }

        total += result;
    }
    println!("{}", total);
}

struct Multizip<T>(Vec<T>);

impl<T> Iterator for Multizip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

pub fn part2(input: String) {
    let count = input.lines().count();

    let mut numbers = VecDeque::new();
    let mut current = vec![];
    for all in Multizip(
        input
            .lines()
            .take(count - 1)
            .map(|s| s.bytes().rev())
            .collect_vec(),
    ) {
        let mut blanks = 0;
        let mut value = 0;
        for b in all {
            if b == b' ' {
                blanks += 1;
                continue;
            }
            value = value * 10 + (b - b'0') as u64;
        }
        if blanks == count - 1 {
            numbers.push_front(current);
            current = vec![];
        } else {
            current.push(value);
        }
    }
    if current.len() > 0 {
        numbers.push_front(current);
    }

    let mut total = 0;
    for (symbol, vec) in input
        .lines()
        .last()
        .expect("Symbol line not found")
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .zip(numbers)
    {
        let is_sum = symbol == "+";
        let mut result = if is_sum { 0 } else { 1 };

        for value in vec {
            if is_sum {
                result += value;
            } else {
                result *= value;
            }
        }

        total += result;
    }
    println!("{}", total);
}
