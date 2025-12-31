use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

fn can_fit(line: &str, areas: &Vec<u64>) -> bool {
    let mut numbers = parse_nums::<u64>(line);
    let width = numbers.next().expect("Width not found");
    let height = numbers.next().expect("Height not found");

    let total_area = numbers
        .zip(areas)
        .map(|(count, &area)| count * area)
        .sum::<u64>();

    total_area < width * height
}

pub fn part1(input: String) {
    let mut areas = vec![];

    for section in input.split("\n\n").take(6) {
        let mut area = 0;
        for line in section.lines().skip(1) {
            for c in line.chars() {
                if c == '#' {
                    area += 1;
                }
            }
        }
        areas.push(area);
    }

    input
        .split("\n\n")
        .last()
        .expect("No bottom section found")
        .lines()
        .filter(|&l| can_fit(l, &areas))
        .count()
        .print();
}
