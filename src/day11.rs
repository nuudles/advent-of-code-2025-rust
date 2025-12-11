use std::collections::{BTreeMap, BTreeSet};

use cached::proc_macro::cached;
use regex::Regex;

#[cached]
fn count_to_out(node: String, connections: BTreeMap<String, BTreeSet<String>>) -> u64 {
    if node == "out" {
        return 1;
    }
    connections[&node]
        .iter()
        .map(|n| count_to_out(n.to_string(), connections.clone()))
        .sum::<u64>()
}

pub fn part1(input: String) {
    let re = Regex::new("[a-z]{3}").expect("Invalid RegEx");
    let connections = input.lines().fold(
        BTreeMap::<String, BTreeSet<String>>::new(),
        |mut result, line| {
            let mut names = re.find_iter(line);
            let input = names.next().expect("Input not found").as_str();
            result.insert(
                input.to_string(),
                names
                    .map(|m| m.as_str().to_string())
                    .collect::<BTreeSet<_>>(),
            );
            result
        },
    );
    println!("{:?}", count_to_out("you".to_string(), connections));
}

#[cached]
fn count_to_out_with_did_pass(
    node: String,
    did_pass_fft: bool,
    did_pass_dac: bool,
    connections: BTreeMap<String, BTreeSet<String>>,
) -> u64 {
    if node == "out" {
        if did_pass_dac && did_pass_fft {
            return 1;
        } else {
            return 0;
        }
    }

    let fft = did_pass_fft || node == "fft";
    let dac = did_pass_dac || node == "dac";

    connections[&node]
        .iter()
        .map(|n| count_to_out_with_did_pass(n.to_string(), fft, dac, connections.clone()))
        .sum::<u64>()
}

pub fn part2(input: String) {
    let re = Regex::new("[a-z]{3}").expect("Invalid RegEx");
    let connections = input.lines().fold(
        BTreeMap::<String, BTreeSet<String>>::new(),
        |mut result, line| {
            let mut names = re.find_iter(line);
            let input = names.next().expect("Input not found").as_str();
            result.insert(
                input.to_string(),
                names
                    .map(|m| m.as_str().to_string())
                    .collect::<BTreeSet<_>>(),
            );
            result
        },
    );
    println!(
        "{}",
        count_to_out_with_did_pass("svr".to_string(), false, false, connections)
    );
}
