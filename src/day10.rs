use std::{
    collections::{BTreeMap, HashMap, HashSet},
    usize,
};

use itertools::{Itertools, iproduct};
use nalgebra::DMatrix;
use pathfinding::prelude::dijkstra;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

pub fn part1(input: String) {
    input
        .lines()
        .map(|line| {
            let mut separated = line.split(" ");
            let pattern = separated.next().expect("No pattern found");
            let desired = pattern
                .chars()
                .skip(1)
                .take(pattern.len() - 2)
                .map(|c| c == '#')
                .collect_vec();
            let mut schematics = separated
                .map(|s| parse_nums::<usize>(s).collect_vec())
                .collect_vec();
            let _ = schematics.remove(schematics.len() - 1);
            (desired, schematics)
        })
        .map(|(desired, schematics)| {
            dijkstra(
                &desired.iter().map(|_| false).collect_vec(),
                |v| {
                    schematics
                        .iter()
                        .map(|b| {
                            let mut next = v.clone();
                            for i in b {
                                next[*i] = !next[*i];
                            }
                            (next, 1)
                        })
                        .collect_vec()
                },
                |v| v == &desired,
            )
            .expect("No shortest path found")
            .1
        })
        .sum::<u64>()
        .print();
}

// #[cached]
// fn min_steps(joltage: Vec<usize>, schematics: Vec<Vec<usize>>) -> u64 {
//     if joltage.iter().all(|j| j == &0) {
//         return 0;
//     }

//     let mut min = u64::MAX;
//     'outer: for schematic in &schematics {
//         if *schematic == joltage {
//             return 1;
//         }

//         let max_multiplier = schematic
//             .iter()
//             .map(|i| joltage[*i])
//             .min()
//             .unwrap_or_default();
//         if max_multiplier == 0 {
//             continue 'outer;
//         }
//         for multiplier in (1..=max_multiplier).rev() {
//             let mut new_joltage = joltage.clone();
//             for &i in schematic {
//                 new_joltage[i] -= multiplier;
//             }
//             println!("{:?}", new_joltage);
//             let steps =
//                 min_steps(new_joltage, schematics.clone()).saturating_add(multiplier as u64);
//             if steps < u64::MAX && steps < min {
//                 min = steps;
//             }
//         }
//     }
//     min
// }

fn reduce(mut matrix: DMatrix<i64>) -> DMatrix<i64> {
    let (rows, cols) = matrix.shape();
    let mut pivot_row = 0;

    for col in 0..cols {
        if pivot_row >= rows {
            break;
        }

        // Find pivot
        let mut max_row = usize::MAX;
        for r in pivot_row..rows {
            if matrix[(r, col)] != 0
                && (max_row == usize::MAX || matrix[(r, col)].abs() < matrix[(max_row, col)].abs())
            {
                max_row = r;
            }
        }

        if max_row == usize::MAX {
            continue;
        }

        // Swap if needed to bring pivot to current row
        if max_row != pivot_row {
            matrix.swap_rows(pivot_row, max_row);
        }

        // If pivot is zero, move to next column
        if matrix[(pivot_row, col)] == 0 {
            continue;
        }

        // Make pivot positive
        if matrix[(pivot_row, col)] < 0 {
            for x in matrix.row_mut(pivot_row) {
                // Scale row
                *x *= -1;
            }
        }

        // Reduce to 1 using other rows
        if matrix[(pivot_row, col)] > 1 {
            let pivot_val = matrix[(pivot_row, col)];

            // Reduce any lower rows
            for r in (pivot_row + 1)..rows {
                let val = matrix[(r, col)];
                if val.abs() < pivot_val.abs() {
                    continue;
                }
                for c in col..cols {
                    if val < 0 {
                        matrix[(r, c)] -= matrix[(pivot_row, c)] * (val / pivot_val);
                    } else {
                        matrix[(r, c)] -= matrix[(pivot_row, c)] * (val / pivot_val);
                    }
                }
            }

            for r in (pivot_row + 1)..rows {
                let val = matrix[(r, col)];
                if val == 0 {
                    continue;
                }
                if val.abs() == 1 {
                    let factor = if val < 0 {
                        pivot_val - 1
                    } else {
                        -(pivot_val - 1)
                    };
                    for c in 0..cols {
                        matrix[(pivot_row, c)] += matrix[(r, c)] * factor;
                    }
                    break;
                } else if pivot_val % val == 1 {
                    let factor = if val < 0 {
                        -pivot_val / val
                    } else {
                        pivot_val / val
                    };
                    for c in 0..cols {
                        matrix[(pivot_row, c)] += matrix[(r, c)] * factor;
                    }
                }
            }
        }

        // Zero out other entries in the pivot column
        if matrix[(pivot_row, col)] == 1 {
            for r in 0..rows {
                if r != pivot_row {
                    let factor = matrix[(r, col)];
                    for c in 0..cols {
                        matrix[(r, c)] -= matrix[(pivot_row, c)] * factor;
                    }
                }
            }
        }

        pivot_row += 1;
    }

    matrix
}

fn reduce2(mut matrix: DMatrix<i64>) -> DMatrix<i64> {
    let (rows, cols) = matrix.shape();
    let mut pivot_row = 0;

    for col in 0..cols {
        if pivot_row >= rows {
            break;
        }

        // Find pivot (largest absolute value for numerical stability)
        let mut max_row = pivot_row;
        for r in (pivot_row + 1)..rows {
            if matrix[(r, col)].abs() > matrix[(max_row, col)].abs() {
                max_row = r;
            }
        }

        // Swap if needed to bring pivot to current row
        if max_row != pivot_row {
            matrix.swap_rows(pivot_row, max_row);
        }

        // If pivot is near zero, move to next column
        if matrix[(pivot_row, col)] == 0 {
            continue;
        }

        // Make pivot positive
        let pivot_val = matrix[(pivot_row, col)];
        for x in matrix.row_mut(pivot_row) {
            if *x % pivot_val != 0 {
                println!("INDIVISIBLE!");
            }

            // Scale row
            *x /= pivot_val;
        }

        // Zero out other entries in the pivot column
        for r in 0..rows {
            if r != pivot_row {
                let factor = matrix[(r, col)];
                for c in 0..cols {
                    matrix[(r, c)] -= matrix[(pivot_row, c)] * factor;
                }
            }
        }
        pivot_row += 1;
    }
    matrix
}

fn find_minimum_dependencies<I>(
    candidates: I,
    dependencies: &BTreeMap<usize, i64>,
    values: &HashMap<usize, (i64, Vec<(usize, i64)>, i64)>,
) -> BTreeMap<usize, i64>
where
    I: Iterator<Item = Vec<i64>>,
{
    let (mut min, mut min_dependencies) = (i64::MAX, BTreeMap::<usize, i64>::new());
    'inner: for candidate in candidates {
        let mut new_dependencies = dependencies.clone();
        for (k, v) in dependencies.keys().sorted().zip(candidate) {
            new_dependencies.insert(*k, v);
        }

        for (&index, (m, d, value)) in values.iter().sorted_by_key(|(i, _)| **i).rev() {
            let x = *value
                - d.iter()
                    .map(|(d_i, d_m)| *d_m * new_dependencies[d_i] as i64)
                    .sum::<i64>();
            if x >= 0 && x % *m == 0 {
                new_dependencies.insert(index, x / *m);
            } else {
                continue 'inner;
            }
        }
        let total = new_dependencies.values().sum::<i64>();
        if total < min {
            min = total;
            min_dependencies = new_dependencies;
        }
    }
    min_dependencies
}

fn solve_equations(
    variables: HashSet<usize>,
    equations: Vec<(HashSet<usize>, u64)>,
) -> HashMap<usize, u64> {
    let mut solved = HashMap::new();

    let mut coefficients = Vec::<i64>::new();
    for equation in &equations {
        for i in 0..variables.len() {
            if equation.0.contains(&i) {
                coefficients.push(1);
            } else {
                coefficients.push(0);
            }
        }
        coefficients.push(equation.1 as i64);
    }

    let matrix = DMatrix::from_vec(variables.len() + 1, equations.len(), coefficients).transpose();
    let reduced = reduce(matrix);
    let mut max_value = 0;

    let mut values = HashMap::<usize, (i64, Vec<(usize, i64)>, i64)>::new();
    let mut dependencies = BTreeMap::new();
    for row in reduced.row_iter() {
        let mut subject = usize::MAX;
        for (i, &col) in row.iter().enumerate() {
            if subject == usize::MAX && col != 0 {
                subject = i;
                values.entry(subject).or_default().0 = col;
            } else if subject != usize::MAX && i == row.len() - 1 {
                values.entry(subject).or_default().2 = col;
                if col > max_value {
                    max_value = col;
                }
            } else if subject != usize::MAX && col != 0 {
                values.entry(subject).or_default().1.push((i, col));
                dependencies.insert(i, 0i64);
            }
        }
    }

    for (&index, &(m, _, value)) in values.iter().filter(|(_, (_, d, _))| d.len() == 0) {
        solved.insert(index, (value / m) as u64);
    }

    let mut max_dependency_value = vec![];
    for _ in &dependencies {
        max_dependency_value.push(max_value);
    }

    let keys = dependencies
        .keys()
        .sorted()
        .enumerate()
        .map(|(i, k)| (*k, i))
        .collect::<HashMap<_, _>>();
    for (_, (_, d, v)) in values.iter() {
        if d.iter().any(|(_, m)| *m < 0) {
            continue;
        }
        for &(d_i, d_m) in d {
            let index = keys[&d_i];
            let max = (*v / d_m).abs();
            if max < max_dependency_value[index] {
                max_dependency_value[index] = max;
            }
        }
    }

    match dependencies.len() {
        0 => (),
        1 => {
            let min_dependencies = find_minimum_dependencies(
                (0..=max_dependency_value[0]).map(|x| vec![x]),
                &dependencies,
                &values,
            );
            for (i, v) in min_dependencies {
                solved.insert(i, v as u64);
            }
        }
        2 => {
            let min_dependencies = find_minimum_dependencies(
                iproduct!(0..=max_dependency_value[0], 0..=max_dependency_value[1])
                    .map(|(i, j)| vec![i, j]),
                &dependencies,
                &values,
            );
            for (i, v) in min_dependencies {
                solved.insert(i, v as u64);
            }
        }
        3 => {
            let min_dependencies = find_minimum_dependencies(
                iproduct!(
                    0..=max_dependency_value[0],
                    0..=max_dependency_value[1],
                    0..=max_dependency_value[2]
                )
                .map(|(i, j, k)| vec![i, j, k]),
                &dependencies,
                &values,
            );
            for (i, v) in min_dependencies {
                solved.insert(i, v as u64);
            }
        }
        4 => {
            let min_dependencies: BTreeMap<usize, i64> = find_minimum_dependencies(
                iproduct!(
                    0..=max_dependency_value[0],
                    0..=max_dependency_value[1],
                    0..=max_dependency_value[2],
                    0..=max_dependency_value[3]
                )
                .map(|(i, j, k, l)| vec![i, j, k, l]),
                &dependencies,
                &values,
            );
            for (i, v) in min_dependencies {
                solved.insert(i, v as u64);
            }
        }
        5 => {
            let min_dependencies: BTreeMap<usize, i64> = find_minimum_dependencies(
                iproduct!(
                    0..=max_dependency_value[0],
                    0..=max_dependency_value[1],
                    0..=max_dependency_value[2],
                    0..=max_dependency_value[3],
                    0..=max_dependency_value[4]
                )
                .map(|(i, j, k, l, m)| vec![i, j, k, l, m]),
                &dependencies,
                &values,
            );
            for (i, v) in min_dependencies {
                solved.insert(i, v as u64);
            }
        }
        _ => panic!("Can't do more than 5"),
    }

    solved
}

fn min_presses(joltage: Vec<usize>, schematics: Vec<Vec<usize>>) -> u64 {
    let mut dependencies = Vec::new();
    for _ in &joltage {
        dependencies.push(vec![]);
    }
    // let mut max = 0;
    for (i, schematic) in schematics.iter().enumerate() {
        for &j in schematic {
            dependencies[j].push(i);
            // if j > max {
            //     max = j;
            // }
        }
    }

    let solved = solve_equations(
        dependencies.iter().flatten().copied().collect(),
        dependencies
            .iter()
            .zip(joltage)
            .map(|(d, j)| (d.iter().copied().collect::<HashSet<usize>>(), j as u64))
            .collect(),
    );

    solved.values().sum::<u64>()
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|line| {
            let mut separated = line.split(" ");
            let _ = separated.next().expect("No pattern found");
            let mut schematics = separated
                .map(|s| parse_nums::<usize>(s).collect_vec())
                .collect_vec();
            let joltage = schematics.remove(schematics.len() - 1);
            (joltage, schematics)
        })
        .map(|(joltage, schematics)| min_presses(joltage, schematics))
        .sum::<u64>()
        .print();
}
