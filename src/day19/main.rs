use std::{fs, collections::HashSet};
use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    idx: usize,
    ore_robot_ore: usize,
    clay_robot_ore: usize,
    obs_robot_ore: usize,
    obs_robot_clay: usize,
    geode_robot_ore: usize,
    geode_robot_obs: usize,
    max_ore:usize,
}

impl Blueprint {
    fn can_build_ore_robot(&self, ore: &usize) -> bool {
        *ore >= self.ore_robot_ore
    }

    fn can_build_clay_robot(&self, ore: &usize) -> bool {
        *ore >= self.clay_robot_ore
    }

    fn can_build_obs_robot(&self, ore: &usize, clay: &usize) -> bool {
        *ore >= self.obs_robot_ore && *clay >= self.obs_robot_clay
    }

    fn can_build_geode_robot(&self, ore: &usize, obs: &usize) -> bool {
        *ore >= self.geode_robot_ore && *obs >= self.geode_robot_obs
    }
}


fn parse_line(source: &str) -> Blueprint {
    let sensor_regex = Regex::new(r"Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();
    let caps: Vec<&str> = sensor_regex
        .captures(source)
        .unwrap()
        .iter()
        .skip(1) // Ignore the complete phrase match
        .map(|x| x.unwrap().as_str())
        .collect();
    let mut b = Blueprint{
        idx: caps[0].parse::<usize>().unwrap(),
        ore_robot_ore: caps[1].parse::<usize>().unwrap(),
        clay_robot_ore: caps[2].parse::<usize>().unwrap(),
        obs_robot_ore: caps[3].parse::<usize>().unwrap(), 
        obs_robot_clay: caps[4].parse::<usize>().unwrap(), 
        geode_robot_ore: caps[5].parse::<usize>().unwrap(), 
        geode_robot_obs: caps[6].parse::<usize>().unwrap(),
        max_ore: 0, 
    };
    b.max_ore = *[b.clay_robot_ore, b.obs_robot_ore, b.geode_robot_ore].iter().max().unwrap();
    b
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct OptState {
    ore: usize,
    clay: usize,
    obs: usize,
    geodes: usize,
    ore_robots: usize,
    clay_robots: usize,
    obs_robots: usize,
    geode_robots: usize,
}

impl OptState {
    fn new() -> Self {
        OptState { ore: 0, clay: 0, obs: 0, geodes: 0, ore_robots: 1, clay_robots: 0, obs_robots: 0, geode_robots: 0 }
    }

    fn add_ores(&self) -> Self {
        let mut state = self.clone();
        state.ore += state.ore_robots;
        state.clay += state.clay_robots;
        state.obs += state.obs_robots;
        state.geodes += state.geode_robots;
        state
    }

    fn progress(&self, b: &Blueprint) -> HashSet<OptState> {
        let mut next_states = HashSet::<OptState>::new();
        // Build

        let maxed_ore_robots = self.ore_robots == b.max_ore;
        let maxed_clay_robots = self.clay_robots == b.obs_robot_clay ;
        let maxed_obs_robots = self.obs_robots == b.geode_robot_obs;

        if b.can_build_ore_robot(&self.ore) && !maxed_ore_robots {
            let mut state = self.add_ores();
            state.ore -= b.ore_robot_ore;
            state.ore_robots += 1;
            next_states.insert(state);
        }

        if b.can_build_clay_robot(&self.ore) && !maxed_clay_robots {
            let mut state = self.add_ores();
            state.ore -= b.clay_robot_ore;
            state.clay_robots += 1;
            next_states.insert(state);
        }

        if b.can_build_obs_robot(&self.ore, &self.clay) && !maxed_obs_robots {
            let mut state = self.add_ores();
            state.ore -= b.obs_robot_ore;
            state.clay -= b.obs_robot_clay;
            state.obs_robots += 1;
            next_states.insert(state);
        }

        if b.can_build_geode_robot(&self.ore, &self.obs) {
            let mut state = self.add_ores();
            state.ore -= b.geode_robot_ore;
            state.obs -= b.geode_robot_obs;
            state.geode_robots += 1;
            next_states.insert(state);
        }

        let dont_do_nothing = b.can_build_geode_robot(&self.ore, &self.obs);

        // Or do nothing
        if !dont_do_nothing {
            let state = self.add_ores();
            next_states.insert(state);
        }
        next_states
    }
}


fn optimize(b: &Blueprint, s: usize) -> usize {
    let mut states = HashSet::<OptState>::from([OptState::new()]);
    for _ in 1..=s {
        let next_states = states.iter()
            .map(|s| s.progress(b))
            .fold(HashSet::<OptState>::new(), |mut acc, x| {
                acc.extend(x.iter());
                acc
            });
        states = next_states;
        
        let max = states.iter().map(|x| x.geodes).max().unwrap();
        let count = states.iter().filter(|x| x.geodes > 0).count();
        if count > 100000 {
            states = states.iter().filter(|x| x.geodes + 5 > max).map(|x|*x).collect();
        }
        //println!("[{}] STATES: {} :[max-{}, count-{}]", i, states.len(), max, count)
    }
    states.iter().map(|s| s.geodes).max().unwrap()

}


fn main() {
    let input_data = fs::read_to_string("src/day19/input.txt").unwrap();

    let blueprints = input_data.lines()
        .map(|line| parse_line(line))
        .collect::<Vec<Blueprint>>();

    let result = blueprints.iter()
        .map(|x| (x, optimize(x, 24)))
        .inspect(|(b, result)| println!("Blueprint {} complete - {}", b.idx, result))
        .map(|(b, result)| b.idx * result)
        .sum::<usize>();

    println!("Part 1 - Quality level: {}", result);

    let result2 = blueprints.iter().take(3)
        .map(|x| (x, optimize(x, 32)))
        .inspect(|(b, result)| println!("Blueprint {} complete - {}", b.idx, result))
        .map(|(_, result)| result)
        .product::<usize>();

    println!("Part 2 - Answer: {}", result2)
}