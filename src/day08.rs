use std::collections::BTreeSet;

use itertools::Itertools;

use crate::parse_nums::parse_nums;

fn distance_squared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1) + (a.2 - b.2) * (a.2 - b.2)
}

pub fn part1(input: String) {
    let boxes = input
        .lines()
        .map(|line| {
            let mut nums = parse_nums::<i64>(line);
            (
                nums.next().unwrap_or_default(),
                nums.next().unwrap_or_default(),
                nums.next().unwrap_or_default(),
            )
        })
        .collect::<BTreeSet<_>>();

    let mut circuits = BTreeSet::<BTreeSet<(i64, i64, i64)>>::new();

    let mut i = 0;
    for v in boxes
        .iter()
        .combinations(2)
        .sorted_by_cached_key(|v| distance_squared(v[0], v[1]))
    {
        let (a, b) = (v[0], v[1]);

        let mut new_circuit = BTreeSet::new();
        new_circuit.insert(*a);
        new_circuit.insert(*b);

        let mut to_remove = BTreeSet::new();
        for circuit in circuits.iter().filter(|s| s.contains(a) || s.contains(b)) {
            to_remove.insert(circuit.clone());
            for j in circuit {
                new_circuit.insert(j.clone());
            }
        }
        for circuit in to_remove {
            circuits.remove(&circuit);
        }

        if new_circuit.len() == boxes.len() {
            println!("Part 2: {}", a.0 * b.0);
            break;
        }

        circuits.insert(new_circuit);

        i += 1;

        if i == 1000 {
            let product = circuits
                .iter()
                .sorted_by_key(|c| c.len())
                .rev()
                .take(3)
                .map(|c| c.len())
                .product::<usize>();
            println!("Part 1: {}", product);
        }
    }
}
