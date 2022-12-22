use std::{fs, collections::HashMap};

#[derive(Debug)]
enum Stuff {
    Rock,
    Sand
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn new(source: &str) -> Self {
        let (x,y) = source.split_once(",").unwrap();
        Pos(x.parse().unwrap(), y.parse().unwrap())
    }
}

fn generate_points(a: &Pos, b: &Pos) -> Vec<Pos> {
    if a.0 == b.0 {
        if a.1 < b.1 {
            (a.1..=b.1).map(|y| Pos(a.0, y)).collect()
        } else {
            (b.1..=a.1).map(|y| Pos(a.0, y)).collect()
        }
    } else {
        if a.0 < b.0 {
            (a.0..=b.0).map(|x| Pos(x, a.1)).collect()
        } else {
            (b.0..=a.0).map(|x| Pos(x, a.1)).collect()
        }
        
    }
}

fn add_sand(items: &HashMap<Pos,Stuff>, max: usize, use_floor: bool) -> Option<Pos> {
    let mut sand = Pos(500,0);
    let floor = max + 2;

    loop {
        // If sand is past max depth, the sand will always fall
        if !use_floor && sand.1 > max {
            return None
        }

        // check below
        let below = Pos(sand.0, sand.1 + 1);
        if !items.contains_key(&below) && below.1 < floor  {
            sand = below;
            continue
        }

        // check left
        let left = Pos(sand.0 - 1, sand.1 + 1);
        if !items.contains_key(&left) && left.1 < floor{
            sand = left;
            continue
        }

        // check right
        let right = Pos(sand.0 + 1, sand.1 + 1);
        if !items.contains_key(&right) && right.1 < floor{
            sand = right;
            continue
        }

        // No where for sand to go
        return Some(sand)
    }
}

fn main() {
    let input_data = fs::read_to_string("src/day14/input.txt").unwrap();

    let mut items = HashMap::<Pos,Stuff>::new();

    let rock_lines: Vec<(Pos,Pos)> = input_data.lines()
        .map(|line| line.split(" -> "))
        .map(|parts| {
            parts.map(|part| Pos::new(part)).collect::<Vec<Pos>>()
        })
        .map(|points| {
            points.windows(2)
                .map(|x| (x[0], x[1])).collect::<Vec<(Pos,Pos)>>()
        })
        .flatten()
        .collect();

    let max_depth = rock_lines.iter()
        .map(|(a,b) | [a.1, b.1])
        .flatten()
        .max()
        .unwrap();
    
    println!("Max Depth is {}", max_depth);

    rock_lines.iter()
        .map(|(a,b)| generate_points(a,b))
        .flatten()
        .for_each(|p| {
            items.insert(p, Stuff::Rock); 
        });

    let mut into_abyss = false;
    let mut sand_count:u32 = 0;
    while !into_abyss {
        let res = add_sand(&items, max_depth, false);
        match res {
            None => into_abyss = true,
            Some(pos) => _ = {
                items.insert(pos, Stuff::Sand);
                sand_count += 1
            },
        }
    }

    println!("Part 1 - Sand units : {}", sand_count);

    items = HashMap::<Pos,Stuff>::new();

    rock_lines.iter()
        .map(|(a,b)| generate_points(a,b))
        .flatten()
        .for_each(|p| {
            items.insert(p, Stuff::Rock); 
        });

    let mut sand_blocked = false;
    sand_count = 0;
    while !sand_blocked {
        let res = add_sand(&items, max_depth, true).unwrap();
        items.insert(res, Stuff::Sand);
        sand_count += 1;
        if res == Pos(500,0) {
            sand_blocked = true
        }
    }

    println!("Part 2 - Sand units : {}", sand_count);

}