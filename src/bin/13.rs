use nom::{
    branch::alt, bytes::complete::tag, character::complete::i32, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};
use std::cmp::Ordering;
use std::fs;
use std::iter::once;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Clone)]
enum ListEl {
    Num(i32),
    L(Vec<ListEl>),
}

impl Ord for ListEl {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListEl::Num(n1), ListEl::Num(n2)) => n1.cmp(n2),
            (ListEl::L(l1), ListEl::L(l2)) => l1.cmp(l2),
            (ListEl::Num(n1), l2) => ListEl::L(vec![ListEl::Num(*n1)]).cmp(l2),
            (l1, ListEl::Num(n2)) => l1.cmp(&ListEl::L(vec![ListEl::Num(*n2)])),
        }
    }
}

impl PartialOrd for ListEl {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_list(s: &str) -> IResult<&str, ListEl> {
    map(delimited(tag("["), parse_list_inside, tag("]")), ListEl::L)(s)
}

fn parse_list_inside(s: &str) -> IResult<&str, Vec<ListEl>> {
    separated_list0(tag(","), alt((map(i32, ListEl::Num), parse_list)))(s)
}

fn gen(input: &str) -> Vec<(ListEl, ListEl)> {
    input
        .split("\n\n")
        .map(|g| {
            let (l1, l2) = g.split_once('\n').unwrap();
            (parse_list(l1).unwrap().1, parse_list(l2).unwrap().1)
        })
        .collect()
}

fn p1(data: &[(ListEl, ListEl)]) -> usize {
    data.iter()
        .enumerate()
        .filter(|(_, (l1, l2))| l1.cmp(l2).is_lt())
        .map(|(i, _)| i + 1)
        .sum()
}

fn p2(data: Vec<(ListEl, ListEl)>) -> usize {
    let k1 = parse_list("[[2]]").unwrap().1;
    let k2 = parse_list("[[6]]").unwrap().1;
    let mut data: Vec<_> = data
        .into_iter()
        .flat_map(|(l1, l2)| vec![l1, l2])
        .chain(once(k1.clone()))
        .chain(once(k2.clone()))
        .collect();
    data.sort_unstable();
    let p1 = data.iter().position(|l| *l == k1).unwrap() + 1;
    let p2 = data.iter().position(|l| *l == k2).unwrap() + 1;
    p1 * p2
}

fn main() {
    let path = "inputs/13.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let data = gen(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = p1(&data);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(data);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d13 {
    use super::*;

    #[test]
    fn t1() {
        let s = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let data = gen(s);
        assert_eq!(p1(&data), 13);
        assert_eq!(p2(data), 140);
    }
}
