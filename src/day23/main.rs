use std::{fs, collections::HashSet, collections::HashMap};

use rayon::prelude::*;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {North, South, West, East}

fn parse_elves(source: &str) -> HashSet<Pos> {
    source.lines()
        .enumerate()
        .map(|(row_idx,  line)| {
            line.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(move |(column_idx, _)| Pos(column_idx as i32, row_idx as i32))
        })
        .flatten()
        .collect()
}

fn propose_move(elf: &Pos, elves: &HashSet<Pos>, moves: &[Dir; 4]) -> Option<Pos> {
    let (nw, n, ne) = (Pos(elf.0 - 1, elf.1 - 1), Pos(elf.0, elf.1 - 1), Pos(elf.0 + 1, elf.1 - 1));
    let (w, e) = (Pos(elf.0 - 1, elf.1), Pos(elf.0 + 1, elf.1));
    let (sw, s, se) = (Pos(elf.0 - 1, elf.1 + 1), Pos(elf.0, elf.1 + 1), Pos(elf.0 + 1, elf.1 + 1));
    if !elves.contains(&nw) && !elves.contains(&n) && !elves.contains(&ne) && 
       !elves.contains(&w) && !elves.contains(&e) && !elves.contains(&sw) && 
       !elves.contains(&s) && !elves.contains(&se) {
        return None;
    }

    for m in moves {
        match m {
            Dir::North => if !elves.contains(&nw) && !elves.contains(&n) && !elves.contains(&ne) { return Some(n) },
            Dir::South => if !elves.contains(&sw) && !elves.contains(&s) && !elves.contains(&se) { return Some(s) },
            Dir::East => if !elves.contains(&ne) && !elves.contains(&e) && !elves.contains(&se) { return Some(e) },
            Dir::West => if !elves.contains(&nw) && !elves.contains(&w) && !elves.contains(&sw) { return Some(w) },
        }
    }
    None
}

fn field_size(elves: &HashSet<Pos>) -> i32 {
    let min_x = elves.iter().map(|p|p.0).min().unwrap();
    let max_x = elves.iter().map(|p|p.0).max().unwrap();
    let min_y = elves.iter().map(|p|p.1).min().unwrap();
    let max_y = elves.iter().map(|p|p.1).max().unwrap();
    return ((max_y - min_y + 1) * (max_x - min_x + 1)) - elves.len() as i32;
}

fn main() {
    let input_data = fs::read_to_string("src/day23/input.txt").unwrap();
    let mut elves = parse_elves(&input_data);
    let mut moves = [Dir::North, Dir::South, Dir::West, Dir::East];

    let mut round_count = 1;
    loop {
        // Calculate proposed moves
        let proposed:HashMap<Pos, Option<Pos>> = elves.iter()
            .map(|e| (*e, propose_move(e, &elves, &moves)))
            .collect();

        let mut uniq = HashSet::<Pos>::new();
        let non_unique = proposed.values()
            .filter(|x| x.is_some() && (!uniq.insert(x.unwrap())))
            .map(|x|x.unwrap())
            .collect::<HashSet<Pos>>();

        
        elves = proposed.par_iter()
            .map(|(e, p)| {
                if p.is_none() {
                    *e
                } else if non_unique.contains(&p.unwrap()) {
                    *e
                } else {
                    p.unwrap()
                }
            })
            .collect();
        moves.rotate_left(1);



        if round_count == 10 {
            println!("Part 1: Field Size {:?}", field_size(&elves))
        }

        if proposed.values().all(|x| x.is_none()) {
            println!("Part 2: Round count for no moves {:?}", round_count);
            break;
        }
        round_count += 1;
    }
}