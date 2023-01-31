use advent_2022::matrix::Matrix;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::time::Instant;

// #[derive(PartialEq)]
// enum Cell {
//     Start,
//     End,
//     El(u8),
// }
//
// impl std::fmt::Debug for Cell {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Cell::Start => f.write_char('S'),
//             Cell::End => f.write_char('E'),
//             Cell::El(val) => f.write_fmt(format_args!("({val})")),
//         }
//     }
// }

fn gen(input: &str) -> Matrix<u8> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let items = input.chars().filter(|c| c.is_alphanumeric()).map(|c| {
        if c == 'S' {
            0
        } else if c == 'E' {
            27
        } else {
            let mut buf = [0];
            c.encode_utf8(&mut buf);
            buf[0] - 96
        }
    });
    Matrix::new(items, width, height)
}

fn p1(mat: &Matrix<u8>) -> u64 {
    let mut memory = HashMap::new();
    let start_pos = mat.index_of(|c| *c == 0).unwrap();
    solve_p1(mat, start_pos, vec![start_pos], 0, &mut memory).unwrap()
}

fn solve_p1(
    cells: &Matrix<u8>,
    cur_pos: (usize, usize),
    cur_path: Vec<(usize, usize)>,
    cur_steps: u64,
    memory: &mut HashMap<Vec<(usize, usize)>, Option<u64>>,
) -> Option<u64> {
    let cur_cell = cells[cur_pos];
    // println!("{cur_cell}");
    // println!("path: {cur_path:?}, cur_val: {cur_cell}");
    if cur_cell == 27 {
        // println!("Found the end!");
        return Some(cur_steps);
    }
    if let Some(res) = memory.get(&cur_path) {
        return *res;
    }
    cells
        .rook_neighbor_indices(cur_pos.0, cur_pos.1)
        .filter_map(|(x, y)| {
            // print!("from {cur_pos:?}, looking at ({x}, {y})");
            if cur_path.contains(&(x, y)) {
                // println!(" but we already visited it");
                return None;
            }
            let maybe_next_cell = cells[(x, y)];
            if (0..=cur_cell + 1).contains(&maybe_next_cell) {
                // println!("... we move there");
                let mut path = cur_path.clone();
                path.push((x, y));
                if let Some(res) = memory.get(&path) {
                    *res
                } else {
                    let res = solve_p1(cells, (x, y), path.clone(), cur_steps + 1, memory);
                    memory.insert(path, res);
                    // println!("Finished path: {res:?}");
                    res
                }
            } else {
                // println!(" but it's too high");
                None
            }
        })
        .min()
}

fn p2() -> u32 {
    0
}

fn main() {
    let path = "inputs/12.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let mat = gen(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = p1(&mat);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}\n", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2();
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d12 {
    use super::*;

    #[test]
    fn t1() {
        println!(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
        );
        let cells = gen("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi");
        assert_eq!(p1(&cells), 31)
    }
}
