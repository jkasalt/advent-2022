#![allow(dead_code)]
#![allow(unused_imports)]
use anyhow::{anyhow, Context, Result};
use derivative::Derivative;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{alpha1, digit1, space1, u32},
        is_alphabetic,
    },
    combinator::{cut, map, map_res, recognize},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair},
    Parser,
};
use nom::{Finish, IResult};
use regex::Regex;
use std::str::FromStr;
use std::time::Instant;
use std::{collections::HashMap, default, fs, hash};

#[derive(Debug, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Resource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Resource::*;
        match s {
            "ore" => Ok(Ore),
            "clay" => Ok(Clay),
            "obsidian" => Ok(Obsidian),
            "geode" => Ok(Geode),
            _ => Err(()),
        }
    }
}

type Cost = (u8, Resource);

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_robot_cost: u8,
    clay_robot_cost: u8,
    obsidian_robot_cost: (u8, u8),
    geode_robot_cost: (u8, u8),
}

fn gen(input: &str) -> Result<Vec<Blueprint>> {
    input
        .lines()
        .map(|l| Blueprint::new(l.trim()))
        .collect::<Result<Vec<Blueprint>>>()
        .context("Failed to parse all of the blueprints")
}

fn parse_one_cost(input: &str) -> IResult<&str, Cost> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse::<u8>()),
        space1,
        map_res(
            alt((tag("ore"), tag("clay"), tag("obsidian"), tag("geode"))),
            Resource::from_str,
        ),
    )(input)
}

fn parse_cost(input: &str) -> IResult<&str, Vec<Cost>> {
    separated_list0(tag(" and "), parse_one_cost)(input)
}

fn parse_recipe_output(input: &str) -> IResult<&str, Resource> {
    map_res(
        delimited(
            tag(". Each ").or(tag("Each ")),
            alpha1,
            tag(" robot costs "),
        ),
        Resource::from_str,
    )(input)
}

fn parse_id(input: &str) -> IResult<&str, u8> {
    delimited(
        tag("Blueprint "),
        map_res(digit1, |s: &str| s.parse::<u8>()),
        tag(": "),
    )(input)
}

fn parse_recipe(input: &str) -> IResult<&str, Vec<(Resource, Vec<Cost>)>> {
    many0(pair(parse_recipe_output, parse_cost))(input)
}

impl Blueprint {
    fn new(input: &str) -> Result<Blueprint> {
        let (rem, id) = parse_id(input).map_err(|e| anyhow!("Failed to parse id: {e}"))?;
        let (_, bp_vec) = parse_recipe(rem).map_err(|e| anyhow!("Failed to parse recipe {e}"))?;
        let mut ore_robot_cost = None;
        let mut clay_robot_cost = None;
        let mut obsidian_robot_cost = None;
        let mut geode_robot_cost = None;
        for recipe in bp_vec {
            match recipe {
                (Resource::Clay, cost) => clay_robot_cost = Some(cost[0].0),
                (Resource::Ore, cost) => ore_robot_cost = Some(cost[0].0),
                (Resource::Obsidian, cost) => obsidian_robot_cost = Some((cost[0].0, cost[1].0)),
                (Resource::Geode, cost) => geode_robot_cost = Some((cost[0].0, cost[1].0)),
            }
        }
        let ore_robot_cost = ore_robot_cost.ok_or(anyhow!("Missing ore robot recipe"))?;
        let clay_robot_cost = clay_robot_cost.ok_or(anyhow!("Missing clay robot recipe"))?;
        let obsidian_robot_cost =
            obsidian_robot_cost.ok_or(anyhow!("Missing obsidian robot recipe"))?;
        let geode_robot_cost = geode_robot_cost.ok_or(anyhow!("Missing geode robot recipe"))?;
        Ok(Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        })
    }
}

#[derive(Derivative, Debug, Clone, PartialEq, Eq)]
#[derivative(Default, Hash)]
struct Status {
    #[derivative(Hash = "ignore")]
    time: u32,
    #[derivative(Default(value = "1"))]
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
    ore_amount: u8,
    clay_amount: u8,
    obsidian_amount: u8,
    geode_amount: u8,
}

impl Status {
    fn can_build_ore_robot(&self, bp: &Blueprint) -> bool {
        self.ore_amount >= bp.ore_robot_cost
    }
    fn can_build_clay_robot(&self, bp: &Blueprint) -> bool {
        self.ore_amount >= bp.clay_robot_cost
    }
    fn can_build_obsidian_robot(&self, bp: &Blueprint) -> bool {
        let ore_cost = bp.obsidian_robot_cost.0;
        let clay_cost = bp.obsidian_robot_cost.1;
        self.ore_amount >= ore_cost && self.clay_amount >= clay_cost
    }
    fn can_build_geode_robot(&self, bp: &Blueprint) -> bool {
        let ore_cost = bp.geode_robot_cost.0;
        let obsidian_cost = bp.geode_robot_cost.1;
        self.ore_amount >= ore_cost && self.obsidian_amount >= obsidian_cost
    }
    fn build_ore_robot(&self, bp: &Blueprint) -> Self {
        Status {
            ore_amount: self.ore_amount - bp.ore_robot_cost,
            ore_robots: self.ore_robots + 1,
            ..*self
        }
    }
    fn build_clay_robot(&self, bp: &Blueprint) -> Self {
        Status {
            ore_amount: self.ore_amount - bp.clay_robot_cost,
            clay_robots: self.clay_robots + 1,
            ..*self
        }
    }
    fn build_obsidian_robot(&self, bp: &Blueprint) -> Self {
        let ore_cost = bp.obsidian_robot_cost.0;
        let clay_cost = bp.obsidian_robot_cost.1;
        Status {
            ore_amount: self.ore_amount - ore_cost,
            clay_amount: self.clay_amount - clay_cost,
            obsidian_robots: self.obsidian_robots + 1,
            ..*self
        }
    }
    fn build_geode_robot(&self, bp: &Blueprint) -> Self {
        let ore_cost = bp.geode_robot_cost.0;
        let obsidian_cost = bp.geode_robot_cost.1;
        Status {
            ore_amount: self.ore_amount - ore_cost,
            obsidian_amount: self.obsidian_amount - obsidian_cost,
            geode_robots: self.geode_robots + 1,
            ..*self
        }
    }

    fn tick(&self) -> Self {
        Status {
            time: self.time + 1,
            ore_amount: self.ore_amount + self.ore_robots,
            clay_amount: self.clay_amount + self.clay_robots,
            obsidian_amount: self.obsidian_amount + self.obsidian_robots,
            geode_amount: self.geode_amount + self.geode_robots,
            ..*self
        }
    }
}

use rayon::prelude::*;

fn p1(input: &[Blueprint]) -> u32 {
    input
        .par_iter()
        .map(|bp| {
            bp.id as u32
                * best_geodes(
                    bp,
                    Status::default(),
                    // &mut HashMap::new(),
                    24,
                    &mut 0,
                    false,
                    false,
                    false,
                    false,
                ) as u32
        })
        .sum()
}

#[allow(clippy::too_many_arguments)]
fn best_geodes(
    bp: &Blueprint,
    rs: Status,
    // mem: &mut HashMap<Status, u8>,
    time_limit: u32,
    best_geode_amount: &mut u32,
    waited_on_ore_robot: bool,
    waited_on_clay_robot: bool,
    waited_on_obsidian_robot: bool,
    waited_on_geode_robot: bool,
) -> u8 {
    // if let Some(val) = mem.get(&rs) {
    //     return *val;
    // }
    if rs.time == time_limit {
        let result = rs.geode_amount;
        *best_geode_amount = std::cmp::max(result as u32, *best_geode_amount);
        // mem.insert(rs, result);
        return result;
    }
    let time_left = time_limit - rs.time;
    let maximum_possible_geodes = rs.geode_amount as u32
        + time_left * rs.geode_robots as u32
        + time_left * (time_left - 1) / 2;
    if maximum_possible_geodes < *best_geode_amount {
        return 0;
    }
    let mut result = 0;
    let mut now_waited_ore = false;
    let mut now_waited_clay = false;
    let mut now_waited_obsidian = false;
    let mut now_waited_geode = false;

    let max_ore_cost = vec![
        bp.ore_robot_cost,
        bp.clay_robot_cost,
        bp.obsidian_robot_cost.0,
        bp.geode_robot_cost.0,
    ]
    .into_iter()
    .max()
    .unwrap();

    if rs.can_build_ore_robot(bp) && !waited_on_ore_robot && rs.ore_robots < max_ore_cost {
        result = std::cmp::max(
            result,
            best_geodes(
                bp,
                rs.tick().build_ore_robot(bp),
                // mem,
                time_limit,
                best_geode_amount,
                false,
                waited_on_clay_robot,
                waited_on_obsidian_robot,
                waited_on_geode_robot,
            ),
        );
        now_waited_ore = true;
    }
    if rs.can_build_clay_robot(bp)
        && !waited_on_clay_robot
        && rs.clay_robots < bp.obsidian_robot_cost.1
    {
        result = std::cmp::max(
            result,
            best_geodes(
                bp,
                rs.tick().build_clay_robot(bp),
                // mem,
                time_limit,
                best_geode_amount,
                waited_on_ore_robot,
                false,
                waited_on_obsidian_robot,
                waited_on_geode_robot,
            ),
        );
        now_waited_clay = true;
    }
    if rs.can_build_obsidian_robot(bp)
        && !waited_on_obsidian_robot
        && rs.obsidian_robots < bp.geode_robot_cost.1
    {
        result = std::cmp::max(
            result,
            best_geodes(
                bp,
                rs.tick().build_obsidian_robot(bp),
                // mem,
                time_limit,
                best_geode_amount,
                waited_on_ore_robot,
                waited_on_clay_robot,
                false,
                waited_on_geode_robot,
            ),
        );
        now_waited_obsidian = true;
    }
    if rs.can_build_geode_robot(bp) && !waited_on_geode_robot {
        result = std::cmp::max(
            result,
            best_geodes(
                bp,
                rs.tick().build_geode_robot(bp),
                // mem,
                time_limit,
                best_geode_amount,
                waited_on_ore_robot,
                waited_on_clay_robot,
                waited_on_obsidian_robot,
                false,
            ),
        );
        now_waited_geode = true;
    }
    result = std::cmp::max(
        result,
        best_geodes(
            bp,
            rs.tick(),
            // mem,
            time_limit,
            best_geode_amount,
            now_waited_ore,
            now_waited_clay,
            now_waited_obsidian,
            now_waited_geode,
        ),
    );
    *best_geode_amount = std::cmp::max(result as u32, *best_geode_amount);
    // mem.insert(rs, result);
    result
}

fn p2(input: &[Blueprint]) -> u32 {
    input[..3]
        .par_iter()
        .map(|bp| {
            best_geodes(
                bp,
                Status::default(),
                // &mut HashMap::new(),
                32,
                &mut 0,
                false,
                false,
                false,
                false,
            ) as u32
        })
        .product()
}

fn main() -> Result<()> {
    let path = "inputs/19.txt";
    let input = fs::read_to_string(path).unwrap();

    let in1 = Instant::now();
    let blueprints = gen(&input)?;
    let in0 = Instant::now();
    println!("Input parsed in: {:?}", in0.duration_since(in1));

    let i11 = Instant::now();
    let res1 = p1(&blueprints);
    let i12 = Instant::now();
    println!("silver: {:?}\ntime: {:?}", res1, i12.duration_since(i11));

    println!("-----");

    let i21 = Instant::now();
    let res2 = p2(&blueprints);
    let i22 = Instant::now();
    println!("gold: {:?}\ntime: {:?}", res2, i22.duration_since(i21));
    Ok(())
}

#[cfg(test)]
mod d19 {
    use core::panic;

    use super::*;

    #[test]
    fn tp1() {
        let s = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
          Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        // let s = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        // let s = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        let bps = dbg!(gen(s).unwrap());
        assert_eq!(p1(&bps), 33);
    }
}
