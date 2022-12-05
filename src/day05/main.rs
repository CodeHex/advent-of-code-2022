use std::fs;
use regex::Regex;

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

struct CrateLocation {
    stack_idx: usize,
    crate_contents: char,
}

struct Move {
    size:usize,
    from:usize,
    to:usize,
}

impl Move {
    fn new(source: &str) -> Move {
        let move_regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        let caps: Vec<usize> = move_regex
            .captures(source)
            .unwrap()
            .iter()
            .skip(1) // Ignore the complete phrase match
            .map(|x| x.unwrap().as_str().parse::<usize>().unwrap())
            .collect();
        Move{size: caps[0], from: caps[1] - 1, to: caps[2] - 1}
    }

    fn execute(&self, stacks: &mut Stacks, is_single_crate_moved: bool){
        let idx_to_split = stacks[self.from].len() - self.size;
        let crates_to_move = &mut stacks[self.from].split_off(idx_to_split);
        if is_single_crate_moved {
            crates_to_move.reverse();
        }
        stacks[self.to].append(crates_to_move)
    }
}

fn get_code(stacks: Stacks) -> String {
    stacks.iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn parse_crate_line(source: &str) -> Vec<CrateLocation> {
    source.chars()
        .collect::<Vec<char>>()
        .chunks(4) // Split into individual crates
        .map(|item| item[1]) // Read the crate value from crate
        .enumerate() // Attach an index so the stack can be identified
        .filter(|(_, item_val)| *item_val != ' ') // Drop any positions that have no crates
        .map(|(i, val)| CrateLocation { stack_idx: i, crate_contents: val })
        .collect()
}

fn add_crate(stacks: &mut Stacks, crate_loc: CrateLocation){
    while crate_loc.stack_idx >= stacks.len() {
        stacks.push(Vec::new())
    }
    stacks[crate_loc.stack_idx].push(crate_loc.crate_contents)
}

fn main() {
    let input_data = fs::read_to_string("src/day05/input.txt").unwrap();
    let (stack_data, move_data) = input_data.split_once("\n\n").unwrap();

    let mut stacks_part1: Stacks = Stacks::new();
    stack_data.lines()
        .rev() // Start at the bottom of the stacks
        .skip(1) // Ignore the numbered line
        .flat_map(|crate_line| parse_crate_line(crate_line))
        .for_each(|c| add_crate(&mut stacks_part1, c));
    let mut stacks_part2 = stacks_part1.clone();

    let move_list:Vec<Move> = move_data.lines()
        .map(|line| Move::new(line))
        .collect();

    // Part 1
    move_list.iter().for_each(|m| {m.execute(&mut stacks_part1, true)});
    println!("Part 1 - Code: {}", get_code(stacks_part1));

    // Part 2
    move_list.iter().for_each(|m| {m.execute(&mut stacks_part2, false)});
    println!("Part 2 - Code: {}", get_code(stacks_part2));
}