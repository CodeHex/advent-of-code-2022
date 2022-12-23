use std::{fs, collections::HashSet};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, i32);

fn neighbours(p: &Pos) -> Vec<Pos> {
    vec![
        Pos(p.0 + 1, p.1, p.2),
        Pos(p.0 - 1, p.1, p.2),
        Pos(p.0, p.1 + 1, p.2),
        Pos(p.0, p.1 - 1, p.2),
        Pos(p.0, p.1, p.2 + 1),
        Pos(p.0, p.1, p.2 - 1),
    ]
}

fn calc_surface_area(cubes: &HashSet<Pos>) -> i32 {
    let mut total:i32 = 0;

    cubes.iter().for_each(|c| {
        let n = neighbours(c);
        let clear_sides = n.iter().filter(|p| !cubes.contains(p)).map(|_| 1).sum::<i32>();
        total += clear_sides;
    });
    return total;
}

fn calc_touching_area(cubes: &HashSet<Pos>) -> i32 {
    let first = *cubes.iter().next().unwrap();
    let pos_min = cubes.iter().fold(first, |mut acc, x| {
        if x.0 - 1 < acc.0 { acc.0 = x.0 - 1; }
        if x.1 - 1 < acc.1 { acc.1 = x.1 - 1; }
        if x.2 - 1 < acc.2 { acc.2 = x.2 - 1; }
        acc
    });
    let pos_max = cubes.iter().fold(first, |mut acc, x| {
        if x.0 + 1 > acc.0 { acc.0 = x.0 + 1; }
        if x.1 + 1 > acc.1 { acc.1 = x.1 + 1; }
        if x.2 + 1 > acc.2 { acc.2 = x.2 + 1; }
        acc
    });
    
    let mut steam = HashSet::<Pos>::new();
    let mut touching:i32 = 0;
    steam.insert(pos_min);
    expand(&pos_min, cubes, &mut steam, &mut touching, &pos_min, &pos_max);
    touching
}

fn in_bounds(p: &Pos, min: &Pos, max:&Pos) -> bool {
    p.0 >= min.0 && p.0 <= max.0 &&
    p.1 >= min.1 && p.1 <= max.1 &&
    p.2 >= min.2 && p.2 <= max.2
}

fn expand(s: &Pos, cubes: &HashSet<Pos>, steam: &mut HashSet<Pos>, touching: &mut i32, min: &Pos, max:&Pos) {
    let expanded = neighbours(s).iter()
        .filter(|p| in_bounds(*p, min, max))
        .filter(|p| !steam.contains(p))
        .filter(|p| {
            let is_lava = cubes.contains(p);
            if is_lava { *touching += 1;}
            !is_lava
        })
        .map(|x|*x)
        .collect::<Vec<Pos>>();
    steam.extend(expanded.iter());
    expanded.iter().for_each(|p| {
        expand(p, cubes, steam, touching, min, max)
    });
}

fn main() {
    let input_data = fs::read_to_string("src/day18/input.txt").unwrap();
    let cubes = input_data.lines()
        .map(|line| {
            let parts = line.split(",").collect::<Vec<&str>>();
            Pos(parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap())
        })
        .collect::<HashSet<Pos>>();

    println!("Part 1 - Surface area: {}", calc_surface_area(&cubes));
    println!("Part 2 - Surface area: {}", calc_touching_area(&cubes));
}