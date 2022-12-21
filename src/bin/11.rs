use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
enum Operation {
    Add(Option<u64>),
    Mul(Option<u64>),
}

#[derive(Clone, Debug)]
struct Monkey {
    inspected: usize,
    items: Vec<u64>,
    ope: Operation,
    divisible_by: u64,
    throw_to: (usize, usize),
}

impl Monkey {}

fn gen(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|s| {
            // let mut monkey_num = None;
            let mut starting_items = None;
            let mut operation = None;
            let mut divisible_by = None;
            let mut throw_to_true = None;
            let mut throw_to_false = None;

            for l in s.lines() {
                match l.split_whitespace().next().unwrap() {
                    "Monkey" => {
                        // let num = l.split_whitespace().nth(1).unwrap().trim_end_matches(':');
                        // monkey_num = Some(num);
                    }
                    "Starting" => {
                        let items: Vec<_> = l
                            .split_whitespace()
                            .filter_map(|w| w.trim_end_matches(',').parse().ok())
                            .collect();
                        starting_items = Some(items);
                    }
                    "Operation:" => {
                        let sign = l.split_whitespace().nth(4).unwrap().chars().next().unwrap();
                        let term = l.split_whitespace().nth(5).unwrap().parse().ok();
                        match sign {
                            '+' => operation = Some(Operation::Add(term)),
                            '*' => operation = Some(Operation::Mul(term)),
                            a => panic!("Unexpected sign {a}"),
                        }
                    }
                    "Test:" => {
                        let d = l.split_whitespace().nth(3).unwrap().parse().unwrap();
                        divisible_by = Some(d);
                    }
                    "If" => {
                        let num = l.split_whitespace().nth(5).unwrap().parse().unwrap();
                        let to = l.split_whitespace().nth(1).unwrap();
                        if to == "true:" {
                            throw_to_true = Some(num)
                        } else if to == "false:" {
                            throw_to_false = Some(num)
                        } else {
                            panic!("Unexpected word after `if`: {to}");
                        }
                    }
                    a => {
                        panic!("Unexpected first word {a}")
                    }
                }
            }
            Monkey {
                inspected: 0,
                items: starting_items.unwrap(),
                ope: operation.unwrap(),
                divisible_by: divisible_by.unwrap(),
                throw_to: (throw_to_false.unwrap(), throw_to_true.unwrap()),
            }
        })
        .collect()
}

fn monkey_round(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        monkeys[i].inspected += monkeys[i].items.len();
        let monkey = monkeys[i].clone();
        for item in monkey.items {
            // Get new worry level
            let worry = match monkey.ope {
                Operation::Add(Some(val)) => item + val,
                Operation::Add(None) => 2u64 * item,
                Operation::Mul(Some(val)) => item * val,
                Operation::Mul(None) => item.pow(2),
            } / 3u64;
            if worry % monkey.divisible_by == 0 {
                monkeys[monkey.throw_to.1].items.push(worry);
            } else {
                monkeys[monkey.throw_to.0].items.push(worry);
            }
        }
        monkeys[i].items.clear();
    }
}

fn monkey_round2(monkeys: &mut [Monkey], ppmc: u64) {
    for i in 0..monkeys.len() {
        monkeys[i].inspected += monkeys[i].items.len();
        let monkey = monkeys[i].clone();
        for item in monkey.items {
            // Get new worry level
            let worry = match monkey.ope {
                Operation::Add(Some(val)) => item + val,
                Operation::Add(None) => 2u64 * item,
                Operation::Mul(Some(val)) => item * val,
                Operation::Mul(None) => item.pow(2),
            } % ppmc;
            if worry % monkey.divisible_by == 0 {
                let to = monkey.throw_to.1;
                monkeys[to].items.push(worry);
            } else {
                let to = monkey.throw_to.0;
                monkeys[to].items.push(worry);
            }
        }
        monkeys[i].items.clear();
    }
}

fn p1(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        monkey_round(&mut monkeys);
    }
    let mut vals: Vec<_> = monkeys.iter().map(|m| m.inspected).collect();
    vals.sort_unstable();
    vals.pop().unwrap() * vals.pop().unwrap()
}

fn p2(mut monkeys: Vec<Monkey>) -> usize {
    let prod = monkeys.iter().map(|m| m.divisible_by).product();
    for t in 0..10000 {
        monkey_round2(&mut monkeys, prod);
    }
    let mut vals: Vec<_> = monkeys.iter().map(|m| m.inspected).collect();
    vals.sort_unstable();
    vals.pop().unwrap() * vals.pop().unwrap()
}

fn main() {
    let path = "inputs/11.txt";
    let input = fs::read_to_string(path).unwrap();
    let monkeys = gen(&input);

    let i11 = Instant::now();
    let res1 = p1(monkeys.clone());
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(monkeys);
    let i22 = Instant::now();
    println!("gold: {}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d11 {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn t1() {
        let monkeys = gen(INPUT);
        assert_eq!(p1(monkeys), 10605);
    }

    #[test]
    fn t2() {
        let monkeys = gen(INPUT);
        assert_eq!(p2(monkeys), 2713310158);
    }
}
