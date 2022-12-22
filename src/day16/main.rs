use std::{fs, collections::HashMap, collections::HashSet};
use regex::Regex;

#[derive(Clone, Debug)]
struct Walker {
    open: HashSet<String>,
    closed: HashSet<String>,
    current_pressure: i32,
    current_location: String,
    elephant_location: String,
}

impl Walker {
    fn new(valves: Vec<&String>) -> Walker {
        Walker{
            open: HashSet::new(),
            closed:  valves.iter().map(|x|x.to_string()).collect(),
            current_pressure: 0, 
            current_location: "AA".to_string(),
            elephant_location: "AA".to_string(),
        }
    }

    fn copy(&self) -> Walker {
        Walker{
            open: self.open.iter().map(|x|x.to_string()).collect(), 
            closed:  self.closed.iter().map(|x|x.to_string()).collect(),
            current_pressure: self.current_pressure, 
            current_location: self.current_location.to_string(),
            elephant_location: self.elephant_location.to_string(),
        }
    }

    fn update_pressure(&mut self, valves: &HashMap<String, Valve>) {
        let increase:i32 = self.open.iter()
            .map(|x| valves.get(x).unwrap().rate)
            .sum();
        self.current_pressure += increase
    }

    fn step(&self, valves: &HashMap<String, Valve>) -> Vec<Walker> {
        let mut next_walkers = Vec::<Walker>::new();
        let current_valve = valves.get(&self.current_location).unwrap();
        
        // open current valve
        if self.closed.contains(&self.current_location) && current_valve.rate != 0 {
            let mut w = self.copy();
            w.open.insert(self.current_location.to_string());
            w.closed.remove(&self.current_location);
            next_walkers.push(w);
        }
        
        // move to connecting chamber
        current_valve.connectors.iter()
            .for_each(|x| {
                let mut w = self.copy();
                w.current_location = x.to_string();
                next_walkers.push(w);
            });
        return next_walkers
    }

    fn step_elephant(&self, valves: &HashMap<String, Valve>) -> Vec<Walker> {
        let mut next_walkers = Vec::<Walker>::new();
        let current_valve = valves.get(&self.elephant_location).unwrap();
        
        // open current valve
        if self.closed.contains(&self.elephant_location) && current_valve.rate != 0 {
            let mut w = self.copy();
            w.open.insert(self.elephant_location.to_string());
            w.closed.remove(&self.elephant_location);
            next_walkers.push(w);
        }
        
        // move to connecting chamber
        current_valve.connectors.iter()
            .for_each(|x| {
                let mut w = self.copy();
                w.elephant_location = x.to_string();
                next_walkers.push(w);
            });
        return next_walkers
    }
}

#[derive(Debug)]
struct Valve {
    label: String,
    rate: i32,
    connectors: Vec<String>,
}

fn parse_line(source: &str) -> Valve {
    let sensor_regex = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(-?[0-9]+); tunnels? leads? to valves? (.*$)").unwrap();
    let caps: Vec<&str> = sensor_regex
        .captures(source)
        .unwrap()
        .iter()
        .skip(1) // Ignore the complete phrase match
        .map(|x| x.unwrap().as_str())
        .collect();
    Valve{ 
        label: caps[0].to_string(), 
        rate: caps[1].parse::<i32>().unwrap(), 
        connectors: caps[2].split(",").map(|x| x.trim().to_string()).collect()
    }
}


fn main() {
    let input_data = fs::read_to_string("src/day16/input.txt").unwrap();

    let valves = input_data.lines()
        .map(|line| parse_line(line))
        .map(|v| (v.label.to_string(), v))
        .collect::<HashMap<String, Valve>>();

    let mut walkers = vec![Walker::new(valves.keys().collect())];

    for step in 0..30 {
        walkers.iter_mut().for_each(|w| w.update_pressure(&valves));
        walkers.sort_by(|a,b| b.current_pressure.partial_cmp(&a.current_pressure).unwrap());
        walkers = walkers.iter()
            .take(10000)
            .map(|w|  w.step(&valves))
            .flatten()
            .collect::<Vec<Walker>>();
        println!("Step {} - {} walkers", step+1, walkers.len())
    }
    println!("Part 1 - Pressure release : {}", walkers.iter().map(|x|x.current_pressure).max().unwrap());

    let mut walkers2 = vec![Walker::new(valves.keys().collect())];

    for step in 0..26 {
        walkers2.iter_mut().for_each(|w| w.update_pressure(&valves));
        walkers2.sort_by(|a,b| b.current_pressure.partial_cmp(&a.current_pressure).unwrap());
        walkers2 = walkers2.iter()
            .take(10000)
            .map(|w|  w.step(&valves))
            .flatten()
            .map(|w|  w.step_elephant(&valves))
            .flatten()
            .collect::<Vec<Walker>>();
        println!("Step {} - {} walkers", step+1, walkers2.len())
    }
    println!("Part 2 - Pressure release : {}", walkers2.iter().map(|x|x.current_pressure).max().unwrap());

    
}