use std::{fs, collections::{HashMap}};

#[derive(Debug, Copy, Clone)]
enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "+" => Op::Plus,
            "-" => Op::Minus,
            "*" => Op::Multiply,
            "/" => Op::Divide,
            _ => panic!("unrecognized op"),
        }
    }
}

impl Op {
    fn perform(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Plus => lhs + rhs,
            Op::Minus => lhs - rhs,
            Op::Multiply => lhs * rhs,
            Op::Divide => lhs / rhs,
        }
    }

    fn perform_alg_l(&self, lhs: &Vec<Task>, rhs: i64) -> Vec<Task> {
        let mut new = lhs.clone();
        new.push(Task{op: *self, val: rhs});
        new
    }

    fn perform_alg_r(&self, lhs:i64, rhs: &Vec<Task>) ->  Vec<Task> {
        match self {
            Op::Plus => self.perform_alg_l(rhs, lhs),
            Op::Minus => Op::Plus.perform_alg_l(&Op::Multiply.perform_alg_l(rhs, -1), lhs),
            Op::Multiply => self.perform_alg_l(rhs, lhs),
            Op::Divide => panic!("OH NO"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Task {
    op: Op,
    val: i64,
}

#[derive(Debug)]
struct Calc {
    lhs: String,
    rhs: String,
    op: Op,
}

#[derive(Debug)]
struct Monkey {
    name: String, 
    number: Option<i64>,
    algebra: Option<Vec<Task>>, // the value is ax + b
    calc: Option<Calc>,
}

impl Monkey {
    fn new(source: &str) -> Self {
        let (name, right) = source.split_once(":").unwrap();
        let right_parts = right.trim_start().split(" ").collect::<Vec<&str>>();
        if right_parts.len() == 3 {
            Monkey { 
                name: name.to_string(), 
                number: None,
                algebra: None,
                calc: Some(Calc { 
                    lhs: right_parts[0].to_string(), 
                    rhs: right_parts[2].to_string(), 
                    op: Op::from(right_parts[1]) })
            }
        } else {
            Monkey { 
                name: name.to_string(), 
                number: Some(right_parts[0].parse().unwrap()),
                algebra: None,
                calc: None,
            }
        }
    }
}



fn main() {
    let input_data = fs::read_to_string("src/day21/input.txt").unwrap();

    let mut monkeys = input_data.lines()
        .map(|line| Monkey::new(line))
        .map(|m| (m.name.to_string(), m))
        .collect::<HashMap<String, Monkey>>();

    while monkeys.values().any(|m| m.number.is_none()) {
        let lookup = &monkeys.values()
            .filter(|m| m.number.is_some())
            .map(|m| (m.name.to_string(), m.number.unwrap()))
            .collect::<HashMap<String,i64>>();

        monkeys.values_mut()
            .filter(|m| m.number.is_none())
            .for_each(|mut m| {
                let calc = m.calc.as_ref().unwrap();
                let left_val = lookup.get(&calc.lhs);
                let right_val = lookup.get(&calc.rhs);
                if left_val.is_some() && right_val.is_some() {
                    let calc = m.calc.as_ref().unwrap();
                    m.number = Some(calc.op.perform(*left_val.unwrap() , *right_val.unwrap()))
                }
            })
    }

    println!("Part 1 - Root monkey {}", monkeys.get("root").unwrap().number.unwrap());

    let mut monkeys2 = input_data.lines()
        .map(|line| Monkey::new(line))
        .map(|m| (m.name.to_string(), m))
        .collect::<HashMap<String, Monkey>>();

    let root = monkeys2.remove("root").unwrap();
    let mut human = monkeys2.get_mut("humn").unwrap();
    human.number = None;
    human.calc = None;
    human.algebra = Some(Vec::new());

    while monkeys2.values().any(|m| m.number.is_none() && m.algebra.is_none()) {
        let lookup = &monkeys2.values()
            .filter(|m| m.number.is_some())
            .map(|m| (m.name.to_string(), m.number.unwrap()))
            .collect::<HashMap<String,i64>>();

        let mut did_calc = false;
        monkeys2.values_mut()
            .filter(|m| m.number.is_none() && m.algebra.is_none() )
            .for_each(|mut m| {
                let calc = m.calc.as_ref().unwrap();
                let left_val = lookup.get(&calc.lhs);
                let right_val = lookup.get(&calc.rhs);
                if left_val.is_some() && right_val.is_some() {
                    m.number = Some(calc.op.perform(*left_val.unwrap() , *right_val.unwrap()));
                    did_calc = true;
                }
            });

        if did_calc {
            continue;
        }

        let alglookup  = &monkeys2.values()
            .filter(|m| m.algebra.is_some())
            .map(|m| (m.name.to_string(), m.algebra.as_ref().unwrap().clone()))
            .collect::<HashMap<String, Vec<Task>>>();

        monkeys2.values_mut()
            .filter(|m| m.number.is_none() &&m.algebra.is_none())
            .for_each(|mut m| {
                let calc = m.calc.as_ref().unwrap();
                let left_val = lookup.get(&calc.lhs);
                let right_val = lookup.get(&calc.rhs);
                let left_alg = alglookup.get(&calc.lhs);
                let right_alg = alglookup.get(&calc.rhs);

                if left_val.is_some() && right_alg.is_some() {
                    let lhs = left_val.unwrap();
                    let rhs = right_alg.unwrap();
                    let c = calc.op.perform_alg_r(*lhs, rhs);
                    m.algebra = Some(c);
                }
                if left_alg.is_some() && right_val.is_some() {
                    let lhs = left_alg.unwrap();
                    let rhs = right_val.unwrap();
                    let c = calc.op.perform_alg_l(lhs, *rhs);
                    m.algebra = Some(c);
                }
                if left_alg.is_some() && right_alg.is_some() {
                    panic!("NOPE")
                }
            });
    }
    let target;
    let alg;
    let m1 = monkeys2.get(&root.calc.as_ref().unwrap().lhs).unwrap();
    let m2 = monkeys2.get(&root.calc.as_ref().unwrap().rhs).unwrap();
    if m1.number.is_some() {
        target = m1.number.unwrap();
        alg = m2.algebra.as_ref().unwrap();
    } else {
        target = m2.number.unwrap();
        alg = m1.algebra.as_ref().unwrap();
    }

    let mut x = target;
    alg.iter().rev().for_each(|t| {
        match t.op {
            Op::Plus => x = x - t.val,
            Op::Minus => x = x + t.val,
            Op::Multiply => x = x / t.val,
            Op::Divide => x = x * t.val,
        }
    });
    println!("Part 2 - Target is [{}], value {}", target, x);
}