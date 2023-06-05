use advent_2022::matrix::Matrix;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::BinaryHeap;
use std::fs;
use std::time::Instant;

fn gen(input: &str) -> (Matrix<u8>, (usize, usize), (usize, usize)) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut start_pos = None;
    let mut end_pos = None;
    let items = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .enumerate()
        .map(|(i, c)| {
            if c == 'S' {
                start_pos = Some((i % width, i / width));
                1
            } else if c == 'E' {
                end_pos = Some((i % width, i / width));
                26
            } else {
                let mut buf = [0];
                c.encode_utf8(&mut buf);
                buf[0] - 96
            }
        });
    (
        Matrix::new(items, width, height),
        start_pos.unwrap(),
        end_pos.unwrap(),
    )
}

#[derive(PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn path_len(mat: &Matrix<u8>, start_pos: (usize, usize), end_pos: (usize, usize)) -> usize {
    let width = mat.width();
    let height = mat.height();
    let mut dist = Matrix::new_with(width, height, || usize::MAX);
    dist[start_pos] = 0;
    let mut queue = BinaryHeap::<State>::new();
    queue.push(State {
        pos: start_pos,
        cost: 0,
    });

    while let Some(State { pos, cost }) = queue.pop() {
        let val = mat[pos];
        if dist[pos] == usize::MAX {
            continue;
        }
        mat.rook_neighbor_indices(pos.0, pos.1)
            .filter(|v| mat[*v] <= val + 1)
            .for_each(|v| {
                let next = State {
                    cost: cost + 1,
                    pos: v,
                };
                if next.cost < dist[v] {
                    dist[v] = next.cost;
                    queue.push(next);
                }
            })
    }
    dist[end_pos]
}

fn p2(mat: &Matrix<u8>, end_pos: (usize, usize)) -> usize {
    let xy: Vec<_> = (0..1).cartesian_product(0..mat.height()).collect();
    xy.par_iter()
        .filter(|(x, y)| mat[(x, y)] == 1)
        .map(|(x, y)| path_len(mat, (*x, *y), end_pos))
        .min()
        .unwrap()
}

fn main() {
    let path = "inputs/12.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let (mat, start_pos, end_pos) = gen(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = path_len(&mat, start_pos, end_pos);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(&mat, end_pos);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d12 {
    use super::*;

    #[test]
    fn t1() {
        let (mat, start_pos, end_pos) = gen("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi");
        assert_eq!(path_len(&mat, start_pos, end_pos), 31)
    }

    #[test]
    fn t2() {
        let (mat, _, end_pos) = gen("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi");
        assert_eq!(p2(&mat, end_pos), 29)
    }
}
