use crate::parse_nums::parse_nums;

pub fn part1(input: String) {
    let mut count = 0;
    let mut dial = 50;
    for line in input.lines() {
        let distance = parse_nums::<i64>(line).next().expect("Distance not found");
        if line.starts_with('L') {
            dial -= distance;
        } else {
            dial += distance;
        }

        while dial < 0 {
            dial += 100;
        }
        while dial > 99 {
            dial -= 100;
        }
        if dial == 0 {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn part2(input: String) {
    let mut count = 0;
    let mut dial = 50;
    for line in input.lines() {
        let distance = parse_nums::<i64>(line).next().expect("Distance not found");

        if line.starts_with('L') {
            let orbits = (distance - dial) / 100;
            count += orbits;
            if distance - 100 * orbits >= dial && dial > 0 {
                count += 1;
            }
            dial -= distance;
        } else {
            let orbits = (distance - (100 - dial)) / 100;
            count += orbits;
            if distance - 100 * orbits >= 100 - dial {
                count += 1;
            }
            dial += distance;
        }

        while dial < 0 {
            dial += 100;
        }
        while dial > 99 {
            dial -= 100;
        }
    }
    println!("{}", count);
}
