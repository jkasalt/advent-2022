use nom::{
    bytes::complete::tag, character::complete::i32, multi::separated_list1,
    sequence::separated_pair, IResult,
};
use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Clone, PartialEq)]
enum Cell {
    Sand,
    Rock,
}

type Grid = HashMap<(i32, i32), Cell>;

fn parse_line(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(tag(" -> "), separated_pair(i32, tag(","), i32))(s)
}

fn gen(s: &str) -> Grid {
    let mut h = HashMap::new();
    for l in s.lines() {
        let points = parse_line(l).unwrap().1;
        for a in points.windows(2) {
            let a1 = a[0];
            let a2 = a[1];
            let diff_x = (a1.0 - a2.0).abs();
            let diff_y = (a1.1 - a2.1).abs();
            for dx in 0..=diff_x {
                h.insert((dx + min(a1.0, a2.0), a1.1), Cell::Rock);
            }
            for dy in 0..=diff_y {
                h.insert((a1.0, dy + min(a1.1, a2.1)), Cell::Rock);
            }
        }
    }
    h
}

fn p1(mut h: Grid) -> u32 {
    let the_void = h.keys().map(|pos| pos.1).max().unwrap() + 1;
    let mut count = 0;
    'outer: loop {
        let mut pos = (500, 0);
        loop {
            let new_y = pos.1 + 1;
            if new_y == the_void {
                break 'outer;
            }
            let down = (pos.0, new_y);
            let down_left = (pos.0 - 1, new_y);
            let down_right = (pos.0 + 1, new_y);
            if !h.contains_key(&down) {
                pos = down;
            } else if !h.contains_key(&down_left) {
                pos = down_left;
            } else if !h.contains_key(&down_right) {
                pos = down_right;
            } else {
                // The grain has settled
                h.insert(pos, Cell::Sand);
                count += 1;
                break;
            }
        }
    }
    count
}

fn p2(mut h: Grid) -> u32 {
    let wall_y = h.keys().map(|pos| pos.1).max().unwrap() + 2;
    let mut count = 0;
    loop {
        let mut pos = (500, 0);
        loop {
            let new_y = pos.1 + 1;
            // if we are on the wall, settle immediately
            if new_y == wall_y {
                h.insert(pos, Cell::Sand);
                count += 1;
                break;
            }
            // Otherwise simulate as usual
            let down = (pos.0, new_y);
            let down_left = (pos.0 - 1, new_y);
            let down_right = (pos.0 + 1, new_y);
            if !h.contains_key(&down) {
                pos = down;
            } else if !h.contains_key(&down_left) {
                pos = down_left;
            } else if !h.contains_key(&down_right) {
                pos = down_right;
            } else {
                // The grain has settled
                h.insert(pos, Cell::Sand);
                count += 1;
                // if we settled at the start position, return
                if pos == (500, 0) {
                    return count;
                }
                break;
            }
        }
    }
}

fn main() {
    let path = "inputs/14.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let data = gen(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = p1(data.clone());
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(data);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d14 {
    use super::*;

    #[test]
    fn t1() {
        let s = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let data = gen(s);
        assert_eq!(p1(data), 24);
    }
}
