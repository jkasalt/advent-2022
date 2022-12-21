use advent_2022::matrix::Matrix;
use itertools::Itertools;
use std::fs;
use std::time::Instant;

fn p1(mat: &Matrix<u32>) -> u32 {
    let w = mat.width();
    let h = mat.height();
    (0..w)
        .cartesian_product(0..h)
        .map(|(x, y)| {
            let mut to_left = 0..x;
            let mut to_right = (x..w).skip(1);
            let mut to_up = 0..y;
            let mut to_down = (y..h).skip(1);

            u32::from(
                to_left.all(|x2| mat[(x2, y)] < mat[(x, y)])
                    || to_right.all(|x2| mat[(x2, y)] < mat[(x, y)])
                    || to_up.all(|y2| mat[(x, y2)] < mat[(x, y)])
                    || to_down.all(|y2| mat[(x, y2)] < mat[(x, y)]),
            )
        })
        .sum()
}

fn score_lr(x: usize, y: usize, range: impl Iterator<Item = usize>, mat: &Matrix<u32>) -> u32 {
    let mut count = 0;
    for x2 in range {
        if mat[(x2, y)] < mat[(x, y)] {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    count
}

fn score_ud(x: usize, y: usize, range: impl Iterator<Item = usize>, mat: &Matrix<u32>) -> u32 {
    let mut count = 0;
    for y2 in range {
        if mat[(x, y2)] < mat[(x, y)] {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    count
}

fn p2(mat: &Matrix<u32>) -> u32 {
    let w = mat.width();
    let h = mat.height();
    (0..w)
        .cartesian_product(0..h)
        .map(|(x, y)| {
            let left_score = score_lr(x, y, (0..x).rev(), mat);
            let right_score = score_lr(x, y, (x..w).skip(1), mat);
            let up_score = score_ud(x, y, (0..y).rev(), mat);
            let down_score = score_ud(x, y, (y..h).skip(1), mat);
            left_score * right_score * up_score * down_score
        })
        .max()
        .unwrap()
}

fn main() {
    let path = "inputs/8.txt";
    let input = fs::read_to_string(path).unwrap();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let in1 = Instant::now();
    let mat = Matrix::new(input.chars().filter_map(|c| c.to_digit(10)), width, height);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = p1(&mat);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}\n", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(&mat);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d8 {
    use super::*;

    #[test]
    fn test() {
        let input = "30373
25512
65332
33549
35390";

        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mat = dbg!(Matrix::new(
            input.chars().filter_map(|c| c.to_digit(10)),
            width,
            height
        ));

        assert_eq!(p1(&mat), 21);
        assert_eq!(p2(&mat), 8);
    }
}
