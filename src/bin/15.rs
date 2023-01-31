use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(i64, i64);

fn gen(input: &str) -> Vec<(Pos, Pos)> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    re.captures_iter(input)
        .map(|cap| {
            (
                Pos(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                Pos(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            )
        })
        .collect()
}

fn p1(data: &[(Pos, Pos)], row: i64) -> usize {
    let mut row_cover: Vec<i64> = data
        .iter()
        .flat_map(|(s, b)| {
            let l1_dist = (s.0 - b.0).abs() + (s.1 - b.1).abs();
            let cover_at_row = std::cmp::max(l1_dist - (s.1 - row).abs(), 0);
            (s.0..(s.0 + cover_at_row))
                .chain((s.0 - cover_at_row)..s.0)
                .collect::<Vec<_>>()
        })
        .collect();
    row_cover.sort();
    row_cover.dedup();
    row_cover.len()
}

fn p2(data: &[(Pos, Pos)], max_search: i64) -> i64 {
    let l1_dists: Vec<_> = data
        .iter()
        .map(|(s, b)| (s.0 - b.0).abs() + (s.1 - b.1).abs())
        .collect();
    let (x, y) = (0..=max_search)
        .cartesian_product(0..=max_search)
        .find(|(x, y)| {
            println!("{x}, {y}");
            !data
                .iter()
                .enumerate()
                .any(|(i, (s, _))| (s.0 - x).abs() + (s.1 - y).abs() <= l1_dists[i])
        })
        .unwrap();
    println!("{x}, {y}");
    x * 4_000_000 + y
}

fn main() {
    let path = "inputs/15.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let data = gen(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = p1(&data, 2_000_000);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(&data, 4_000_000);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d15 {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    #[test]
    fn t1() {
        let data = gen(INPUT);
        assert_eq!(p1(&data, 10), 26);
    }

    #[test]
    fn t2() {
        let data = gen(INPUT);
        assert_eq!(p2(&data, 20), 56000011);
    }
}
