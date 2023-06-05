use itertools::Itertools;
use once_cell::sync::Lazy;
use std::fs;
use std::time::Instant;

static INPUT: Lazy<String> = Lazy::new(|| {
    // let path = "bigboys/6/bigboy.txt";
    let path = "inputs/6.txt";
    fs::read_to_string(path).unwrap()
});

fn solve(input: &str, distinct_chars: usize) -> usize {
    input
        .as_bytes()
        .windows(distinct_chars)
        .position(|w| {
            w.iter()
                .enumerate()
                .cartesian_product(w.iter().enumerate())
                .all(|((i1, b1), (i2, b2))| b1 != b2 || i1 == i2)
        })
        .unwrap_or_else(|| panic!("No window of {distinct_chars} distinct letters has been found"))
        + distinct_chars
}

fn p1(input: &str) -> usize {
    solve(input, 4)
}

fn p2(input: &str) -> usize {
    solve(input, 14)
}

fn main() {
    let data = Lazy::force(&INPUT);

    let i11 = Instant::now();
    let res1 = p1(data);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("---");

    let i21 = Instant::now();
    let res2 = p2(data);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d6 {
    use super::*;
    use test_case::test_case;

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 7; "sample 1")]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz" => 5; "sample 2")]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg"=> 6; "sample 3")]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"=> 10; "sample 4")]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"=> 11; "sample 5")]
    fn tp1(input: &str) -> usize {
        p1(input)
    }
    #[test]
    fn tp2() {}
}
