use std::{
    cmp::{max, min},
    collections::{BTreeMap, HashSet},
    ops::RangeInclusive,
};

use cached::proc_macro::cached;
use itertools::Itertools;
use pathfinding::num_traits::{abs, signum};

use crate::{parse_nums::parse_nums, point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    parse_nums::<i64>(&input)
        .tuples()
        .map(|(x, y)| Point { x, y })
        .combinations(2)
        .map(|v| abs(v[0].x - v[1].x + 1) * abs(v[0].y - v[1].y + 1))
        .max()
        .expect("No max area found")
        .print();
}

#[cached(key = "String", convert = r#"{ format!("{}{}", x, y) }"#)]
fn is_inside(
    x: i64,
    y: i64,
    red_tiles: &Vec<Point<i64>>,
    horizontal_borders: &BTreeMap<i64, HashSet<RangeInclusive<i64>>>,
    vertical_borders: &BTreeMap<i64, HashSet<RangeInclusive<i64>>>,
) -> bool {
    let mut crossings = 0;

    if horizontal_borders
        .iter()
        .any(|(vy, s)| vy == &y && s.iter().any(|r| r.contains(&x)))
    {
        return true;
    }
    if vertical_borders
        .iter()
        .any(|(vx, s)| vx == &x && s.iter().any(|r| r.contains(&y)))
    {
        return true;
    }

    for (p1, p2) in red_tiles.iter().tuple_windows() {
        if (p1.y > y) != (p2.y > y)
            && (x as f64)
                < (p2.x - p1.x) as f64 * (y - p1.y) as f64 / (p2.y - p1.y) as f64 + p1.x as f64
        {
            crossings += 1;
        }
    }

    crossings % 2 == 1
}

pub fn part2(input: String) {
    let mut red_tiles = parse_nums::<i64>(&input)
        .tuples()
        .map(|(x, y)| Point { x, y })
        .collect_vec();
    red_tiles.push(red_tiles[0].clone());

    let mut horizontal_borders = BTreeMap::<i64, HashSet<RangeInclusive<i64>>>::new();
    let mut vertical_borders = BTreeMap::<i64, HashSet<RangeInclusive<i64>>>::new();

    for (a, b) in red_tiles.iter().tuple_windows() {
        let direction = Point {
            x: signum(b.x - a.x),
            y: signum(b.y - a.y),
        };
        if direction.x == 0 {
            vertical_borders
                .entry(a.x)
                .or_default()
                .insert(min(a.y, b.y)..=max(a.y, b.y));
        } else {
            horizontal_borders
                .entry(a.y)
                .or_default()
                .insert(min(a.x, b.x)..=max(a.x, b.x));
        }
    }

    let mut max_area = 0;

    /*
    for y in 0..=8 {
        for x in 0..=12 {
            let p = Point { x, y };
            if red_tiles.contains(&p) {
                print!("#");
            } else if horizontal_borders
                .get(&Point { x: 1, y: 0 })
                .unwrap()
                .iter()
                .any(|(ry, x_range)| *ry == y && x_range.contains(&x))
            {
                print!(">");
            } else if horizontal_borders
                .get(&Point { x: -1, y: 0 })
                .unwrap()
                .iter()
                .any(|(ry, x_range)| *ry == y && x_range.contains(&x))
            {
                print!("<");
            } else if vertical_borders
                .get(&Point { x: 0, y: 1 })
                .unwrap()
                .iter()
                .any(|(rx, y_range)| *rx == x && y_range.contains(&y))
            {
                print!("v");
            } else if vertical_borders
                .get(&Point { x: 0, y: -1 })
                .unwrap()
                .iter()
                .any(|(rx, y_range)| *rx == x && y_range.contains(&y))
            {
                print!("^");
            } else {
                print!(".");
            }
        }
        println!();
    }
     */

    'inner: for v in red_tiles
        .iter()
        .combinations(2)
        .sorted_by_key(|v| {
            (max(v[0].x, v[1].x) - min(v[0].x, v[1].x) + 1)
                * (max(v[0].y, v[1].y) - min(v[0].y, v[1].y) + 1)
        })
        .rev()
    {
        let first = v[0];
        let second = v[1];

        let tl = Point {
            x: min(first.x, second.x),
            y: min(first.y, second.y),
        };
        let tr = Point {
            x: max(first.x, second.x),
            y: min(first.y, second.y),
        };
        let br = Point {
            x: max(first.x, second.x),
            y: max(first.y, second.y),
        };
        let bl = Point {
            x: min(first.x, second.x),
            y: max(first.y, second.y),
        };
        let area = (br.x - tl.x + 1) * (br.y - tl.y + 1);
        if area <= max_area {
            continue;
        }

        // Check the borders to see if every point is in the polygon
        for x in tl.x..=tr.x {
            if !is_inside(x, tl.y, &red_tiles, &horizontal_borders, &vertical_borders)
                || !is_inside(x, bl.y, &red_tiles, &horizontal_borders, &vertical_borders)
            {
                continue 'inner;
            }
        }
        for y in tl.y..=bl.y {
            if !is_inside(tl.x, y, &red_tiles, &horizontal_borders, &vertical_borders)
                || !is_inside(tr.x, y, &red_tiles, &horizontal_borders, &vertical_borders)
            {
                continue 'inner;
            }
        }

        // println!("===");

        // Check to see if inside
        /*
        let mut x = tl.x;
        let mut y = tl.y;

        let x_range = tl.x..=br.x;
        let y_range = tl.y..=br.y;

        // Go right
        let mut borders = horizontal_borders[&y]
            .iter()
            .filter(|r| r.start() >= x_range.start() || r.end() <= x_range.end())
            .map(|r| (r.clone(), y..=y))
            .chain(vertical_borders.iter().flat_map(|(&vx, s)| {
                if vx <= *x_range.start() || vx >= *x_range.end() {
                    return vec![];
                }
                s.iter()
                    .filter(|r| r.contains(&y))
                    .map(|r| (vx..=vx, r.clone()))
                    .collect_vec()
            }))
            .sorted_by(|a, b| a.0.start().cmp(b.0.start()).then(b.0.end().cmp(a.0.end())));

        while x < br.x {
            if let Some((xr, yr)) = borders.next() {
                if xr.end() > xr.start() && xr.end() > &x {
                    // Horizontal border
                    x = min(*xr.end(), br.x);
                } else if yr.end() > yr.start() {
                    // Vertical border
                    if yr.end() > &y {
                        // Going down, we're outside now, bail!
                        continue 'inner;
                    }
                } else {
                    // println!("Going right: {:?},{:?} {:?} {:?}", x, y, xr, yr);
                    // continue 'inner;
                }
            } else {
                if red_tiles.contains(&tr) {
                    continue 'inner;
                } else {
                    x = br.x;
                }
            }
        }

        // Go down
        let mut borders = vertical_borders[&x]
            .iter()
            .filter(|r| r.start() >= y_range.start() || r.end() <= y_range.end())
            .map(|r| (x..=x, r.clone()))
            .chain(horizontal_borders.iter().flat_map(|(&vy, s)| {
                if vy <= *y_range.start() || vy >= *y_range.end() {
                    return vec![];
                }
                s.iter()
                    .filter(|r| r.contains(&x))
                    .map(|r| (r.clone(), vy..=vy))
                    .collect_vec()
            }))
            .sorted_by(|a, b| a.1.start().cmp(b.1.start()).then(a.1.end().cmp(b.1.end())));

        while y < br.y {
            if let Some((xr, yr)) = borders.next() {
                if yr.end() > yr.start() && yr.end() > &y {
                    // Vertical border
                    y = min(*yr.end(), br.y);
                } else if xr.end() > xr.start() {
                    // Horizontal border
                    if xr.start() < &x {
                        // Going left, we're outside now, bail!
                        continue 'inner;
                    }
                } else {
                    // continue 'inner;
                }
            } else {
                if red_tiles.contains(&br) {
                    continue 'inner;
                } else {
                    y = br.y;
                }
            }
        }

        // Go left
        let mut borders = horizontal_borders[&y]
            .iter()
            .filter(|r| r.start() >= x_range.start() || r.end() <= x_range.end())
            .map(|r| (r.clone(), y..=y))
            .chain(vertical_borders.iter().flat_map(|(&vx, s)| {
                if vx <= *x_range.start() || vx >= *x_range.end() {
                    return vec![];
                }
                s.iter()
                    .filter(|r| r.contains(&y))
                    .map(|r| (vx..=vx, r.clone()))
                    .collect_vec()
            }))
            .sorted_by(|a, b| b.0.end().cmp(a.0.end()).then(a.0.start().cmp(b.0.start())));

        while x > tl.x {
            if let Some((xr, yr)) = borders.next() {
                if xr.end() > xr.start() && xr.start() < &x {
                    // Horizontal border
                    x = max(*xr.start(), tl.x);
                } else if yr.end() > yr.start() {
                    // Vertical border
                    if yr.start() < &y {
                        // Going up, we're outside now, bail!
                        continue 'inner;
                    }
                } else {
                    // println!("Going left: {:?},{:?} {:?} {:?}", x, y, xr, yr);
                    // continue 'inner;
                }
            } else {
                if red_tiles.contains(&bl) {
                    continue 'inner;
                } else {
                    x = tl.x;
                }
            }
        }

        // Go up
        let mut borders = vertical_borders[&x]
            .iter()
            .filter(|r| r.start() >= y_range.start() || r.end() <= y_range.end())
            .map(|r| (x..=x, r.clone()))
            .chain(horizontal_borders.iter().flat_map(|(&vy, s)| {
                if vy <= *y_range.start() || vy >= *y_range.end() {
                    return vec![];
                }
                s.iter()
                    .filter(|r| r.contains(&x))
                    .map(|r| (r.clone(), vy..=vy))
                    .collect_vec()
            }))
            .sorted_by(|a, b| b.1.end().cmp(a.1.end()).then(a.1.start().cmp(b.1.start())));

        while y > tl.y {
            if let Some((xr, yr)) = borders.next() {
                if yr.end() > yr.start() && yr.start() < &y {
                    // Vertical border
                    y = max(*yr.start(), tl.y);
                } else if xr.end() > xr.start() {
                    // Horizontal border
                    if xr.end() > &x {
                        // Going right, we're outside now, bail!
                        continue 'inner;
                    }
                } else {
                    // println!(
                    //     "Going up: {:?} {:?} {:?},{:?} {:?} {:?}",
                    //     tl, br, x, y, xr, yr
                    // );
                    // continue 'inner;
                }
            } else {
                // y = br.y;
                if red_tiles.contains(&tr) {
                    continue 'inner;
                } else {
                    x = tl.x;
                }
            }
        }
        */

        // Go right
        // let mut on_border = true;
        // while x < br.x {
        //     if on_border {
        //         if let Some(x_range) = horizontal_borders[&y]
        //             .iter()
        //             .find(|x_range| x_range.start() == &x)
        //         {
        //             x = min(*x_range.end(), br.x);
        //             if x == br.x {
        //                 break;
        //             }
        //             if let Some(_) = vertical_borders[&x]
        //                 .iter()
        //                 .find(|y_range| y_range.start() == &y)
        //             {
        //                 // Going up
        //                 x += 1;
        //                 on_border = false;
        //             } else {
        //                 // Outside
        //                 continue 'inner;
        //             }
        //         }
        //     } else {
        //         if let Some(nx, _) = vertical_borders.iter().sorted_by_key(|(k, _)| k).find(|(nx, s)| nx > x && s.contains(|r| r.co))
        //     }
        // }

        // Go right
        // {
        //     let mut horizontal = horizontal_borders[&tl.y]
        //         .iter()
        //         .filter(|r| r.contains(&tl.x) || r.contains(&br.x))
        //         .sorted_by_key(|r| r.start())
        //         .collect::<VecDeque<_>>();
        //     let mut vertical = vertical_borders
        //         .iter()
        //         .filter(|(x, s)| {
        //             **x >= tl.x && **x <= br.x && s.iter().any(|r| r.contains(&tl.y))
        //         })
        //         .flat_map(|(x, s)| s.iter().filter(|r| r.contains(&tl.y)).map(|r| (x, r)))
        //         .sorted_by_key(|(x, _)| **x)
        //         .collect::<VecDeque<_>>();

        //     let mut on_border = true;
        //     while x < br.x {
        //         if let Some(x_range) = horizontal.pop_front()
        //             && x_range.start() == &x
        //         {
        //             x = min(*x_range.end(), br.x);
        //             if x == br.x {
        //                 break;
        //             }
        //             if let Some((vx, y_range)) = vertical.pop_front()
        //                 && vx == &x
        //                 && y_range.start() == &y
        //             {
        //                 if vertical.is_empty() {
        //                     break;
        //                 } else {
        //                     continue 'inner;
        //                 }
        //             } else {
        //                 continue 'inner;
        //             }
        //         }
        //     }
        // }
        // for x in tl.x..=br.x {
        //     if !is_inside(x, tl.y, &horizontal_borders, &vertical_borders) {
        //         continue 'inner;
        //     }
        //     if !is_inside(x, br.y, &horizontal_borders, &vertical_borders) {
        //         continue 'inner;
        //     }
        // }
        // for y in br.y..=tl.y {
        //     if !is_inside(tl.x, y, &horizontal_borders, &vertical_borders) {
        //         continue 'inner;
        //     }
        //     if !is_inside(br.x, y, &horizontal_borders, &vertical_borders) {
        //         continue 'inner;
        //     }
        // }
        max_area = area;
        break;
    }

    println!("{}", max_area);
}
