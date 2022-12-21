use itertools::Itertools;
use std::time::Instant;

const INPUT: &str = include_str!("../../inputs/3.txt");

fn priority(c: char) -> u64 {
    let mut buf = [0; 1];
    c.encode_utf8(&mut buf);
    if let Some(val) = buf.first() {
        if (65..=90).contains(val) {
            return (val - 38) as u64;
        } else if (97..=122).contains(val) {
            return (val - 96) as u64;
        }
    }
    panic!("Unexpected char {c}");
}

fn p1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (s1, s2) = l.split_at(l.len() / 2);
            let c = s1.chars().find(|s1c| s2.contains(*s1c)).unwrap();
            priority(c)
        })
        .sum()
}

fn p2(input: &str) -> u64 {
    let mut will_sum = vec![];
    for (first, second, third) in input.lines().tuples() {
        let mut letters: Vec<_> = first.chars().collect();
        letters.retain(|l| second.contains(*l) && third.contains(*l));
        will_sum.push(priority(letters[0]))
    }
    will_sum.iter().sum()
}

fn main() {
    let i11 = Instant::now();
    let res1 = p1(INPUT);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(INPUT);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d3 {
    use super::*;

    #[test]
    fn t1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(p1(input), 157);
    }

    #[test]
    fn t2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(p2(input), 70);
    }
}
