use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::time::Instant;

static INPUT: Lazy<Data> = Lazy::new(|| {
    let path = "bigboys/5/bigboy.txt";
    // let path = "inputs/5.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let res = Data::new(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    res
});

#[derive(Debug, Clone)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Data {
    crates: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Data {
    fn new(input: &str) -> Data {
        let lines: Vec<_> = input.lines().collect();
        let stack_line = lines
            .iter()
            .position(|l| l.chars().any(|c| c.is_numeric()))
            .unwrap();
        let crates = {
            let re_num_cols = Regex::new(r"(\d+)").unwrap();
            let num_cols: usize = re_num_cols
                .captures_iter(lines[stack_line])
                .map(|cap| cap[1].parse().unwrap())
                .max()
                .unwrap();
            let mut will_return = vec![Vec::<char>::new(); num_cols];
            let re_crates = Regex::new(r"(?:\[(\w)\])|(?:^\s{3}|\s{4})").unwrap();
            for line in lines[..stack_line].iter().rev() {
                for (i, c) in re_crates.captures_iter(line).enumerate() {
                    let c = match c.get(1) {
                        Some(found) => found.as_str().chars().next().unwrap(),
                        None => continue,
                    };
                    will_return[i].push(c);
                }
            }
            will_return
        };
        let instructions = {
            let re_instructions = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            lines[stack_line + 2..]
                .iter()
                .map(|line| {
                    let caps = re_instructions.captures(line).unwrap();
                    Instruction {
                        num: caps[1].parse().unwrap(),
                        from: caps[2].parse::<usize>().unwrap() - 1,
                        to: caps[3].parse::<usize>().unwrap() - 1,
                    }
                })
                .collect()
        };
        Data {
            crates,
            instructions,
        }
    }
}

fn p1(mut data: Data) -> String {
    for instruction in data.instructions {
        for _ in 0..instruction.num {
            let to_push = data.crates[instruction.from].pop().unwrap();
            data.crates[instruction.to].push(to_push);
        }
    }
    data.crates.iter_mut().map(|c| c.pop().unwrap()).collect()
}

fn p2(mut data: Data) -> String {
    for inst in data.instructions {
        let split = data.crates[inst.from].len() - inst.num;
        let mut moving = data.crates[inst.from].split_off(split);
        data.crates[inst.to].append(&mut moving);
    }
    data.crates.iter_mut().map(|c| c.pop().unwrap()).collect()
}

fn main() {
    let data = Lazy::force(&INPUT);

    let data1 = data.clone();
    let data2 = data.clone();

    let i11 = Instant::now();
    let res1 = p1(data1);
    let i12 = Instant::now();
    println!(
        "silver: {:?}\ntime: {:?}\n--",
        res1,
        i12.duration_since(i11)
    );

    let i21 = Instant::now();
    let res2 = p2(data2);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d5 {
    use super::*;

    #[test]
    fn tp1() {
        let input = Data::new(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        assert_eq!(p1(input), "CMZ");
    }
    #[test]
    fn tp2() {
        let input = Data::new(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        assert_eq!(p2(input), "MCD");
    }
}
