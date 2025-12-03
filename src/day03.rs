use cached::proc_macro::cached;

use crate::selfprint::SelfPrint;

fn max_joltage(bank: &str) -> u64 {
    let mut max = 0;
    let bytes = bank.as_bytes();
    for i in 0..bank.len() - 1 {
        for j in i + 1..bank.len() {
            let joltage = (bytes[i] - b'0') as u64 * 10 + (bytes[j] - b'0') as u64;
            if joltage > max {
                max = joltage;
            }
        }
    }
    max
}

pub fn part1(input: String) {
    input
        .lines()
        .map(|line| max_joltage(line))
        .sum::<u64>()
        .print();
}

#[cached]
fn max_joltage_with_size(bank: String, size: usize) -> u64 {
    if size == 1 {
        return (bank.bytes().max().unwrap_or_default() - b'0') as u64;
    }

    let mut max = 0;
    let bytes = bank.as_bytes();
    for i in 0..=bank.len() - size {
        let joltage = (bytes[i] - b'0') as u64 * 10u64.pow(size as u32 - 1)
            + max_joltage_with_size(bank[i + 1..bank.len()].to_string(), size - 1);
        if joltage > max {
            max = joltage;
        }
    }
    max
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|line| max_joltage_with_size(line.to_string(), 12))
        .sum::<u64>()
        .print();
}
