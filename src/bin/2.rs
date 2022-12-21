use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/2.txt");

enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Sign {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Sign::Rock),
            "B" | "Y" => Ok(Sign::Paper),
            "C" | "Z" => Ok(Sign::Scissors),
            _ => Err(()),
        }
    }
}

fn points(yours: &Sign, others: &Sign) -> u64 {
    use Sign::*;
    match (yours, others) {
        (Rock, Rock) => 4,
        (Rock, Paper) => 1,
        (Rock, Scissors) => 7,
        (Paper, Rock) => 8,
        (Paper, Paper) => 5,
        (Paper, Scissors) => 2,
        (Scissors, Rock) => 3,
        (Scissors, Paper) => 9,
        (Scissors, Scissors) => 6,
    }
}

fn p1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (o, y) = l.split_once(' ').unwrap();
            let o = Sign::from_str(o).unwrap();
            let y = Sign::from_str(y).unwrap();
            points(&y, &o)
        })
        .sum()
}

enum End {
    Win,
    Draw,
    Loss,
}

impl FromStr for End {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(End::Loss),
            "Y" => Ok(End::Draw),
            "Z" => Ok(End::Win),
            _ => Err(()),
        }
    }
}

fn points_2(others: &Sign, end: &End) -> u64 {
    use End::*;
    use Sign::*;
    match (others, end) {
        (Rock, Win) => 8,
        (Rock, Draw) => 4,
        (Rock, Loss) => 3,
        (Paper, Win) => 9,
        (Paper, Draw) => 5,
        (Paper, Loss) => 1,
        (Scissors, Win) => 7,
        (Scissors, Draw) => 6,
        (Scissors, Loss) => 2,
    }
}

fn p2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (o, e) = l.split_once(' ').unwrap();
            let o = Sign::from_str(o).unwrap();
            let e = End::from_str(e).unwrap();
            points_2(&o, &e)
        })
        .sum()
}

fn main() {
    println!("{}", p1(INPUT));
    println!("{}", p2(INPUT));
}

#[cfg(test)]
mod d2 {
    use super::*;

    #[test]
    fn t2() {
        let input = "A Y
B X
C Z";
        assert_eq!(p2(input), 12);
    }
}
