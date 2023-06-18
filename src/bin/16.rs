use anyhow::Result;
use derivative::Derivative;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, Parser,
};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::time::Instant;
use std::{
    cmp::{max, Ordering},
    hash::Hash,
};

const TIME_LIMIT: u32 = 30;
const TIME_LIMIT_P2: u32 = 26;

#[derive(Debug)]
struct ValveInfo {
    flow: u32,
    neighbors: Vec<String>,
}

fn parse_line(input: &str) -> IResult<&str, (String, u32, Vec<String>)> {
    tuple((parse_valve_name, parse_flow_rate, parse_neighbors))(input)
}

fn parse_valve_name(input: &str) -> IResult<&str, String> {
    preceded(tag("Valve "), map(alpha1, String::from))(input)
}

fn parse_flow_rate(input: &str) -> IResult<&str, u32> {
    preceded(tag(" has flow rate="), u32)(input)
}

fn parse_neighbors(input: &str) -> IResult<&str, Vec<String>> {
    preceded(
        tag("; tunnels lead to valves ").or(tag("; tunnel leads to valve ")),
        separated_list1(tag(", "), map(alpha1, String::from)),
    )(input)
}

fn gen(input: &str) -> HashMap<String, ValveInfo> {
    input
        .lines()
        .map(|l| parse_line(l).unwrap().1)
        .map(|(name, flow, neighbors)| (name, ValveInfo { flow, neighbors }))
        .collect()
}

#[derive(Debug, Derivative, Clone, PartialEq, Eq)]
#[derivative(Hash)]
struct Status {
    #[derivative(Hash = "ignore")]
    time: u32,
    position: String,
    opened: BTreeSet<String>,
    #[derivative(Hash = "ignore")]
    release: u32,
}

fn p1(graph: &HashMap<String, ValveInfo>) -> u32 {
    let status = Status {
        opened: BTreeSet::new(),
        position: String::from("AA"),
        time: 1,
        release: 0,
    };
    let mut memo = HashMap::new();
    p1_inner(status, graph, &mut memo, &mut 0)
}

fn p1_inner(
    status: Status,
    graph: &HashMap<String, ValveInfo>,
    memo: &mut HashMap<Status, u32>,
    best_so_far: &mut u32,
) -> u32 {
    if let Some(val) = memo.get(&status) {
        return *val;
    }
    if status.time >= TIME_LIMIT {
        let result = status.release;
        memo.insert(status, result);
        return result;
    }
    if status.opened.len() >= graph.values().filter(|valve| valve.flow > 0).count() {
        let result = status.release;
        memo.insert(status, result);
        return result;
    }
    let time_left = TIME_LIMIT - status.time;
    let best_valve = graph
        .values()
        .max_by_key(|valve| valve.flow)
        .map(|valve| valve.flow)
        .unwrap();
    let upper_bound = status.release + time_left * (time_left - 1) / 2 * best_valve;
    if upper_bound < *best_so_far {
        return 0;
    }

    // println!("{status:?}");

    // if the current valve has greater than zero release value, we recurse opening it
    let here = &status.position;
    let mut result = 0;
    if graph[here].flow > 0 && !status.opened.contains(here) {
        let mut new_opened = status.opened.clone();
        new_opened.insert(here.clone());
        let new_status = Status {
            time: status.time + 1,
            release: status.release + time_left * graph[here].flow,
            opened: new_opened,
            ..status.clone()
        };
        result = max(p1_inner(new_status, graph, memo, best_so_far), result);
    }
    // Then we recurse on the neighbors
    for neighbor in graph[here].neighbors.iter() {
        let new_status = Status {
            time: status.time + 1,
            position: neighbor.clone(),
            ..status.clone()
        };
        result = max(p1_inner(new_status, graph, memo, best_so_far), result);
    }
    memo.insert(status, result);
    *best_so_far = max(result, *best_so_far);
    result
}

#[derive(Debug, Clone)]
struct StatusPart2 {
    time: u32,
    position_me: String,
    previous_me: Option<String>,
    position_bear: String,
    previous_bear: Option<String>,
    opened: BTreeSet<String>,
    release: u32,
}

impl Hash for StatusPart2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (smaller, bigger) = match self.position_me.cmp(&self.position_bear) {
            Ordering::Less | Ordering::Equal => (&self.position_me, &self.position_bear),
            Ordering::Greater => (&self.position_bear, &self.position_me),
        };
        smaller.hash(state);
        bigger.hash(state);
        self.opened.hash(state);
    }
}

impl PartialEq for StatusPart2 {
    fn eq(&self, other: &Self) -> bool {
        (self.time == other.time)
            && (self.opened == other.opened)
            && (self.position_me == other.position_me || self.position_me == other.position_bear)
            && (self.position_bear == other.position_bear
                || self.position_bear == other.position_me)
    }
}

impl Eq for StatusPart2 {}

impl StatusPart2 {
    fn tick(&self) -> Self {
        StatusPart2 {
            time: self.time + 1,
            ..self.clone()
        }
    }
    fn me_open_valve(&self, pos: &str, graph: &HashMap<String, ValveInfo>) -> StatusPart2 {
        let time_left = TIME_LIMIT_P2 - self.time;
        let mut new_opened = self.opened.clone();
        new_opened.insert(pos.to_string());
        let new_release = self.release + time_left * graph[pos].flow;
        StatusPart2 {
            previous_me: None,
            release: new_release,
            opened: new_opened,
            ..self.clone()
        }
    }

    fn bear_open_valve(&self, pos: &str, graph: &HashMap<String, ValveInfo>) -> StatusPart2 {
        let time_left = TIME_LIMIT_P2 - self.time;
        let mut new_opened = self.opened.clone();
        new_opened.insert(pos.to_string());
        let new_release = self.release + time_left * graph[pos].flow;
        StatusPart2 {
            previous_bear: None,
            release: new_release,
            opened: new_opened,
            ..self.clone()
        }
    }

    fn move_me(&self, pos: &str) -> StatusPart2 {
        StatusPart2 {
            previous_me: Some(self.position_me.clone()),
            position_me: pos.to_string(),
            ..self.clone()
        }
    }

    fn move_bear(&self, pos: &str) -> StatusPart2 {
        StatusPart2 {
            previous_bear: Some(self.position_bear.clone()),
            position_bear: pos.to_string(),
            ..self.clone()
        }
    }
}

fn p2(graph: &HashMap<String, ValveInfo>) -> u32 {
    let status = StatusPart2 {
        opened: BTreeSet::new(),
        position_bear: String::from("AA"),
        position_me: String::from("AA"),
        previous_bear: None,
        previous_me: None,
        time: 0,
        release: 0,
    };
    let mut memo = HashMap::new();
    p2_inner(status, graph, &mut memo, &mut 0)
}

fn p2_inner(
    status: StatusPart2,
    graph: &HashMap<String, ValveInfo>,
    memo: &mut HashMap<StatusPart2, u32>,
    best_so_far: &mut u32,
) -> u32 {
    if let Some(val) = memo.get(&status) {
        return *val;
    }

    if status.time >= TIME_LIMIT_P2 {
        let result = status.release;
        memo.insert(status, result);
        return result;
    }
    if status.opened.len() >= graph.values().filter(|valve| valve.flow > 0).count() {
        let result = status.release;
        memo.insert(status, result);
        return result;
    }
    let time_left = TIME_LIMIT_P2 - status.time;
    let best_valve = graph
        .values()
        .max_by_key(|valve| valve.flow)
        .map(|valve| valve.flow)
        .unwrap();
    let upper_bound = status.release + time_left * (time_left - 1) * best_valve;
    if upper_bound < *best_so_far {
        return 0;
    }

    let pos_me = &status.position_me;
    let pos_bear = &status.position_bear;
    let can_open_me = graph[pos_me].flow > 0 && !status.opened.contains(pos_me);
    let can_open_bear = graph[pos_bear].flow > 0 && !status.opened.contains(pos_bear);

    let mut result = 0;

    // The case where we both open a valve
    if can_open_me && can_open_bear && pos_bear != pos_me {
        let new_status = status
            .tick()
            .me_open_valve(pos_me, graph)
            .bear_open_valve(pos_bear, graph);
        result = max(p2_inner(new_status, graph, memo, best_so_far), result);
    }

    // Otherwise either one moves and one opens or we both move
    for neighbor_me in graph[pos_me].neighbors.iter() {
        // I move and the bear opens
        let can_move_me = match status.previous_me {
            None => true,
            Some(ref prev) => *prev != *neighbor_me,
        };
        if can_open_bear && can_move_me {
            let new_status = status
                .tick()
                .bear_open_valve(pos_bear, graph)
                .move_me(neighbor_me);
            result = max(p2_inner(new_status, graph, memo, best_so_far), result);
        }
        for neighbor_bear in graph[pos_bear].neighbors.iter() {
            // The bear moves and I open
            let can_move_bear = match status.previous_bear {
                None => true,
                Some(ref prev) => *prev != *neighbor_bear,
            };
            if can_open_me && can_move_bear {
                // println!("The bear moves and I open");
                let new_status = status
                    .tick()
                    .me_open_valve(pos_me, graph)
                    .move_bear(neighbor_bear);
                result = max(p2_inner(new_status, graph, memo, best_so_far), result);
            }
            // We both move
            if can_move_bear && can_move_me {
                let new_status = status.tick().move_me(neighbor_me).move_bear(neighbor_bear);
                result = max(p2_inner(new_status, graph, memo, best_so_far), result);
            }
        }
    }

    *best_so_far = max(result, *best_so_far);

    memo.insert(status, result);
    result
}

fn main() {
    let path = "inputs/16.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let graph = gen(&input);
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = p1(&graph);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(&graph);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
}

#[cfg(test)]
mod d16 {
    use super::*;
    use std::{
        collections::{hash_map::DefaultHasher, HashSet},
        hash::Hasher,
    };

    const S: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn t1() {
        let graph = gen(S);
        assert_eq!(p1(&graph), 1651)
    }
    #[test]
    fn t2() {
        let graph = gen(S);
        assert_eq!(p2(&graph), 1707)
    }

    #[test]
    fn comm_hash() {
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        let s1 = StatusPart2 {
            time: 1,
            position_bear: String::from("CC"),
            position_me: String::from("DD"),
            previous_bear: None,
            previous_me: None,
            release: 0,
            opened: BTreeSet::new(),
        };
        let s2 = StatusPart2 {
            time: 2,
            position_bear: String::from("DD"),
            position_me: String::from("CC"),
            previous_bear: None,
            previous_me: None,
            release: 23,
            opened: BTreeSet::new(),
        };
        s1.hash(&mut hasher1);
        let h1 = hasher1.finish();
        s2.hash(&mut hasher2);
        let h2 = hasher2.finish();
        assert_eq!(h1, h2);
    }

    #[test]
    fn comm_hash2() {
        let mut set = HashSet::new();

        let s1 = StatusPart2 {
            time: 1,
            position_bear: String::from("CC"),
            position_me: String::from("DD"),
            previous_bear: None,
            previous_me: None,
            release: 0,
            opened: BTreeSet::new(),
        };
        let s2 = StatusPart2 {
            time: 1,
            position_bear: String::from("DD"),
            position_me: String::from("CC"),
            previous_bear: None,
            previous_me: None,
            release: 0,
            opened: BTreeSet::new(),
        };
        set.insert(s1);
        set.insert(s2);
        assert_eq!(set.len(), 1);
    }
}
