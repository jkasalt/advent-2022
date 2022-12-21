use std::collections::HashSet;
use std::fs;
use std::time::Instant;

struct Ope {
    dir: char,
    num: u32,
}

fn gen(input: &str) -> Vec<Ope> {
    input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_once(' ').unwrap();
            let dir = dir.chars().next().unwrap();
            let num = num.parse().unwrap();
            Ope { dir, num }
        })
        .collect()
}

fn p1(opes: &[Ope]) -> usize {
    let mut hpos = (0i32, 0i32);
    let mut tpos = (0i32, 0i32);
    let mut res = HashSet::new();
    for ope in opes {
        for _ in 0..ope.num {
            match ope.dir {
                'R' => hpos.0 += 1,
                'U' => hpos.1 += 1,
                'L' => hpos.0 -= 1,
                'D' => hpos.1 -= 1,
                x => panic!("Unexpected dirction {x}"),
            }
            let to_move = match (hpos.0 - tpos.0, hpos.1 - tpos.1) {
                // overlapping
                (0, 0) => (0, 0),
                // touching up/left/down/right
                (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                // touching diagonally
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                // need to move up/left/down/right
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                // need to move to the right diagonally
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),
                // need to move to the left diagonally
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),
                // need to move up/down diagonally
                (1, 2) => (1, 1),
                (-1, 2) => (-1, 1),
                (1, -2) => (1, -1),
                (-1, -2) => (-1, -1),
                d => panic!("Unexpected difference {d:?}"),
            };
            tpos.0 += to_move.0;
            tpos.1 += to_move.1;
            res.insert(tpos);
        }
    }
    res.len()
}

fn p2<const N: usize>(opes: &[Ope]) -> usize {
    let mut poses = [(0i32, 0i32); N];
    let mut res = HashSet::new();

    for ope in opes {
        for _ in 0..ope.num {
            match ope.dir {
                'R' => poses.first_mut().unwrap().0 += 1,
                'U' => poses.first_mut().unwrap().1 += 1,
                'L' => poses.first_mut().unwrap().0 -= 1,
                'D' => poses.first_mut().unwrap().1 -= 1,
                x => panic!("Unexpected dirction {x}"),
            }
            for i in 1..poses.len() {
                let (left, right) = poses.split_at_mut(i);
                let hpos = left.last().unwrap();
                let tpos = right.first_mut().unwrap();
                let to_move = match (hpos.0 - tpos.0, hpos.1 - tpos.1) {
                    // overlapping
                    (0, 0) => (0, 0),
                    (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                    (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),

                    (0, 2) => (0, 1),
                    (0, -2) => (0, -1),
                    (2, 0) => (1, 0),
                    (-2, 0) => (-1, 0),

                    (2, 1) | (1, 2) | (2, 2) => (1, 1),
                    (2, -1) | (1, -2) | (2, -2) => (1, -1),
                    (-2, 1) | (-1, 2) | (-2, 2) => (-1, 1),
                    (-2, -1) | (-1, -2) | (-2, -2) => (-1, -1),
                    d => panic!("Unexpected difference {d:?}"),
                };

                tpos.0 += to_move.0;
                tpos.1 += to_move.1;
            }

            res.insert(*poses.last().unwrap());
        }
    }
    res.len()
}

fn main() {
    let path = "inputs/9.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let opes = gen(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = p1(&opes);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2::<10>(&opes);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d9 {
    use super::*;

    #[test]
    fn test() {
        let opes: Vec<_> = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .lines()
        .map(|line| {
            let dir = line.chars().next().unwrap();
            let num = line.chars().nth(2).unwrap().to_digit(10).unwrap();
            Ope { dir, num }
        })
        .collect();
        assert_eq!(p1(&opes), 13);
    }
    #[test]
    fn test2() {
        let opes: Vec<_> = gen("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        assert_eq!(p2::<10>(&opes), 1);
    }

    #[test]
    fn test3() {
        let opes: Vec<_> = gen("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
        assert_eq!(p2::<10>(&opes), 36);
    }
}
