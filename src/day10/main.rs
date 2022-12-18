use std::{fs, collections::{HashMap, HashSet}};

#[derive(PartialEq)]
enum OpType {Noop, Addx}

struct Op {
    op_type: OpType,
    cycles: u8,
    arg: Option<i32>,
}

impl Op {
    fn parse(source: &str) -> Self {
        let (op, arg) = source.split_once(" ").unwrap_or((source, ""));
        match op {
            "noop" => Op { op_type: OpType::Noop, cycles:1, arg: None},
            "addx" => Op { op_type: OpType::Addx, cycles:2, arg: Some(arg.parse().unwrap())},
            _ => panic!("unrecogonzied op")
        }
    }
}

#[derive(Debug)]
struct Comp {
    cycle: i32,
    prev_cycle: i32,
    x : i32,
    prev_x: i32,
    watcher_idx: usize,
    signals : HashMap<i32, i32>,
    crt: HashSet<(i32,i32)>,
}

impl Comp {
    const SIGNAL: &'static [i32] = &[20, 60, 100, 140, 180, 220];

    fn new() -> Self {
        Comp { 
            cycle: 0, 
            prev_cycle:0, 
            x: 1, 
            prev_x: 1, 
            watcher_idx: 0, 
            signals: HashMap::new(),
            crt: HashSet::new(),
         }
    }

    fn run(&mut self, op: &Op) {
        self.prev_cycle = self.cycle;
        self.prev_x = self.x;
        self.cycle += op.cycles as i32;
        if op.op_type == OpType::Addx {
            self.x += op.arg.unwrap();
        }

        // Record Signal Strengths
        let sig = Self::SIGNAL.get(self.watcher_idx);
        if sig.is_some() && self.cycle >= *sig.unwrap() {
            self.signals.insert(Self::SIGNAL[self.watcher_idx], Self::SIGNAL[self.watcher_idx] * self.prev_x);
            self.watcher_idx += 1;
        }

        // Record Sprites
        for cycle in self.prev_cycle..self.cycle {
            let row = cycle / 40;
            let column = cycle - (row * 40);
            if (self.prev_x - column).abs() <= 1 {
                self.crt.insert((row, column));
            }
        }
    }

    fn signal_strength(&self) -> i32 {
        self.signals.iter()
            .map(|(_,v)| v)
            .sum()
    }

    fn print_crt(&self) {
        for j in 0..6 {
            for i in 0..40 {
                if self.crt.contains(&(j,i)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
}

fn main() {
    let input_data = fs::read_to_string("src/day10/input.txt").unwrap();

    let ops: Vec<Op> = input_data.lines()
        .map(|line| Op::parse(line))
        .collect();

    let mut cmp = Comp::new();

    ops.iter().for_each(|x| cmp.run(x));

    println!("Part 1 - Signal Strength: {}", cmp.signal_strength());
    println!("Part 2");
    cmp.print_crt()

}