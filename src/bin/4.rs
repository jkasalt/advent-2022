const INPUT: &str = include_str!("../../inputs/4.txt");

fn ranges(l: &str) -> (u64, u64, u64, u64) {
    let (r1, r2) = l.split_once(',').unwrap();
    let (s11, s12) = r1.split_once('-').unwrap();
    let (s21, s22) = r2.split_once('-').unwrap();
    let s11: u64 = s11.parse().unwrap();
    let s12: u64 = s12.parse().unwrap();
    let s21: u64 = s21.parse().unwrap();
    let s22: u64 = s22.parse().unwrap();
    (s11, s12, s21, s22)
}

fn p1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (s11, s12, s21, s22) = ranges(l);
            u64::from((s21 <= s11 && s22 >= s12) || (s11 <= s21 && s12 >= s22))
        })
        .sum()
}

fn p2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (s11, s12, s21, s22) = ranges(l);
            u64::from(
                (s21 >= s11 || s12 <= s22) && s12 >= s21
                    || (s11 >= s21 && s11 <= s22)
                    || (s22 >= s11 && s22 <= s12),
            )
        })
        .sum::<u64>()
}

fn main() {
    println!("{}", p1(INPUT));
    println!("{}", p2(INPUT));
}

#[cfg(test)]
mod d4 {
    use super::*;

    #[test]
    fn t1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(p1(input), 2);
    }

    #[test]
    fn t2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(p2(input), 4);
    }
}
