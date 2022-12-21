use std::fs;
use std::time::Instant;

fn p1() -> u32 {
    0
}

fn p2() -> u32 {
    0
}

fn main() {
    let path = "inputs/8.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let mat = Matrix::new(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = p1();
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}\n", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2();
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d8 {
    use super::*;
}
