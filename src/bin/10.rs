use std::fs;
use std::time::Instant;

struct State {
    x: i32,
    cycle: usize,
    operation: Option<(i32, usize)>,
    instr: Vec<Option<i32>>,
    acc: i32,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("cycle", &self.cycle)
            .field("x", &self.x)
            .field("operation", &self.operation)
            .field("acc", &self.acc)
            .finish()
    }
}

impl State {
    fn new(instr: &[Option<i32>]) -> State {
        State {
            cycle: 0,
            operation: None,
            x: 1,
            acc: 0,
            instr: instr.to_owned(),
        }
    }
}

fn gen(input: &str) -> Vec<Option<i32>> {
    input
        .lines()
        .map(|line| {
            if let Some((_, num)) = line.split_once(' ') {
                Some(num.parse().unwrap())
            } else {
                None
            }
        })
        .chain(std::iter::repeat(None).take(2))
        .collect()
}

fn p1(mut state: State) -> i32 {
    let mut instrs = state.instr.iter();
    loop {
        state.cycle += 1;
        if state.operation.is_none() {
            match instrs.next() {
                None => break,
                Some(instr) => state.operation = instr.map(|val| (val, 1)),
            };
        }
        if (state.cycle + 20) % 40 == 0 {
            state.acc += state.cycle as i32 * state.x;
        }
        if let Some((val, cycles_left)) = &mut state.operation {
            if *cycles_left == 0 {
                state.x += *val;
                state.operation = None;
            } else {
                *cycles_left -= 1;
            }
        }
    }
    state.acc
}

struct Screen([bool; 240]);

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = self
            .0
            .iter()
            .map(|b| if *b { '#' } else { '.' })
            .enumerate()
            .flat_map(|(i, c)| {
                if i == 0 || i % 40 != 0 {
                    vec![c]
                } else {
                    vec!['\n', c]
                }
            })
            .collect();
        f.write_str(&string)
    }
}

fn p2(mut state: State) -> String {
    let mut instrs = state.instr.iter();
    let mut pixels = Screen([false; 240]);

    loop {
        state.cycle += 1;
        let pixel_idx = state.cycle - 1;
        if pixel_idx > 240 {
            break;
        }
        if state.operation.is_none() {
            match instrs.next() {
                None => break,
                Some(instr) => state.operation = instr.map(|val| (val, 1)),
            };
        }
        if (state.x - 1..=state.x + 1).contains(&(pixel_idx as i32 % 40)) {
            pixels.0[state.cycle - 1] = true;
        }

        if let Some((val, cycles_left)) = &mut state.operation {
            if *cycles_left == 0 {
                state.x += *val;
                state.operation = None;
            } else {
                *cycles_left -= 1;
            }
        }
    }
    format!("{pixels}")
}

fn main() {
    let path = "inputs/10.txt";
    let input = gen(&fs::read_to_string(path).unwrap());
    let s1 = State::new(&input);
    let s2 = State::new(&input);

    let i11 = Instant::now();
    let res1 = p1(s1);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(s2);
    let i22 = Instant::now();
    println!("gold:\n{}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d10 {
    use super::*;
    #[test]
    fn t1() {
        let opes = gen("noop
addx 3
addx -5");
        assert_eq!(p1(State::new(&opes)), 0);
    }

    #[test]
    fn t2() {
        let opes = gen("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");
        assert_eq!(p1(State::new(&opes)), 13140);
        assert_eq!(
            p2(State::new(&opes)),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
