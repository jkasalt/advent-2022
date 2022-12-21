const INPUT: &str = include_str!("../../inputs/1.txt");

fn p1(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<u64>().unwrap()).sum::<u64>())
        .max()
        .unwrap()
}

fn p2(input: &str) -> u64 {
    let sums = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<u64>().unwrap()).sum::<u64>());

    let mut top_three = [0; 3];
    for s in sums {
        let t = top_three.iter_mut().min().unwrap();
        if s > *t {
            *t = s;
        }
    }

    top_three.iter().sum::<u64>()
}

fn main() {
    println!("{}", p1(INPUT));
    println!("{}", p2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tp2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(p2(input), 45000);
    }
}
