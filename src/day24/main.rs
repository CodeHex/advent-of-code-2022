use std::{fs, collections::HashMap, collections::HashSet};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {Up, Down, Left, Right}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cyclone {
    dir: Dir,
    initial_pos: Pos,
    max: i32,
}

impl Cyclone {
    fn new(pos: Pos, dir: Dir, max: i32) -> Cyclone {
        Cyclone { dir: dir, initial_pos: pos, max: max }
    }

    fn is_horizontal(&self) -> bool {
        return self.dir == Dir::Left || self.dir == Dir::Right 
    }

    fn is_vertical(&self) -> bool {
        return self.dir == Dir::Up || self.dir == Dir::Down 
    }

    fn pos_at(&self, t: i32) -> Pos {
        match self.dir {
            Dir::Up => Pos(self.initial_pos.0, (self.initial_pos.1 - t - 1).rem_euclid(self.max - 2) + 1),
            Dir::Down => Pos(self.initial_pos.0, (self.initial_pos.1 + t - 1).rem_euclid(self.max - 2) + 1),
            Dir::Left => Pos((self.initial_pos.0 - t - 1).rem_euclid(self.max - 2) + 1, self.initial_pos.1),
            Dir::Right => Pos((self.initial_pos.0 + t - 1).rem_euclid(self.max - 2) + 1, self.initial_pos.1),
        }
    }
}

#[derive(Debug)]
struct Basin {
    horizontal_cyclones: HashMap<i32, Vec<Cyclone>>,
    vertical_cyclones: HashMap<i32, Vec<Cyclone>>,
    max_x: i32,
    max_y: i32,
    start: Pos,
    end: Pos,
}

impl Basin {
    fn new<'a>(source: &str) -> Self {
        let size_y = source.lines().count() as i32;
        let size_x = source.lines().nth(1).unwrap().len() as i32;
    
        let cyclones = source.lines()
            .enumerate()
            .map(|(row_idx,  line)| {
                line.char_indices()
                    .filter(|(_, c)| *c != '#' && *c != '.')
                    .map(move |(column_idx, c)| {
                        let p = Pos(column_idx as i32, row_idx as i32);
                        match c {
                            '^' => Cyclone::new(p, Dir::Up, size_y),
                            '>' => Cyclone::new(p, Dir::Right, size_x),
                            '<' => Cyclone::new(p, Dir::Left, size_x),
                            'v' => Cyclone::new(p, Dir::Down, size_y),
                            _ => panic!("unrecogonized character")
                        }
                    })
            })
            .flatten()
            .collect::<Vec<Cyclone>>();

        let horizontal_cyclones = cyclones.iter()
            .filter(|c| c.is_horizontal())
            .map(|c| (c.initial_pos.1, c))
            .fold(HashMap::<i32,Vec<Cyclone>>::new(), |mut acc,x| {
                if !acc.contains_key(&x.0) {
                    acc.insert(x.0, vec![*x.1]);
                } else {
                    acc.entry(x.0).and_modify(|y| y.push(*x.1));
                }
                acc
            });

        let vertical_cyclones = cyclones.iter()
            .filter(|c| c.is_vertical())
            .map(|c| (c.initial_pos.0, c))
            .fold(HashMap::<i32,Vec<Cyclone>>::new(), |mut acc,x| {
                if !acc.contains_key(&x.0) {
                    acc.insert(x.0, vec![*x.1]);
                } else {
                    acc.entry(x.0).and_modify(|y| y.push(*x.1));
                }
                acc
            });
        Basin { 
            vertical_cyclones: vertical_cyclones, 
            horizontal_cyclones: horizontal_cyclones, 
            max_x: size_x - 1, 
            max_y: size_y - 1,
            start: Pos(1, 0),
            end : Pos(size_x - 2, size_y - 1)
        }
    }

    fn is_cyclone_at(&self, p: &Pos, t: i32) -> bool {
        self.horizontal_cyclones.get(&p.1).unwrap_or(&Vec::<Cyclone>::new()).iter()
            .chain(self.vertical_cyclones.get(&p.0).unwrap_or(&Vec::<Cyclone>::new()).iter())
            .any(|c| c.pos_at(t) == *p)
    }

    fn is_valid_pos(&self, p: &Pos) -> bool {
        p.0 > 0 && p.0 < self.max_x && p.1 > 0 && p.1 < self.max_y || *p == self.end || *p == self.start
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Walker {
    pos: Pos,
    at_end: bool,
    back_at_start: bool,
}

impl Walker {
    fn new(p: Pos) -> Self {
        Walker { pos: p, at_end: false, back_at_start: false }
    }

    fn next(&self, p: Pos) -> Self {
        Walker { pos: p, at_end: self.at_end, back_at_start: self.back_at_start}
    }
}

fn main() {
    let input_data = fs::read_to_string("src/day24/input.txt").unwrap();

    let basin = Basin::new(&input_data);
    let mut walkers = HashSet::<Walker>::new();
    walkers.insert(Walker::new(basin.start));
    let mut minute = 0;

    let mut first_end_minute: Option<i32> = None;

    'outer: loop {
        minute += 1;
        let mut next_walkers = HashSet::<Walker>::new();
        for walk in walkers.iter() {
            let w = walk.pos;
            // Check up
            let up = Pos(w.0, w.1 - 1);
            if basin.is_valid_pos(&up) && !basin.is_cyclone_at(&up, minute) {
                let mut n = walk.next(up);
                if up == basin.start {
                    if walk.at_end == true {
                        n.back_at_start = true;
                    }
                }
                next_walkers.insert(n);
            }
            // Check left
            let left = Pos(w.0 - 1, w.1);
            if basin.is_valid_pos(&left) && !basin.is_cyclone_at(&left, minute) {
                next_walkers.insert(walk.next(left));
            }
            // Check right
            let right = Pos(w.0 + 1, w.1);
            if basin.is_valid_pos(&right) && !basin.is_cyclone_at(&right, minute) {
                next_walkers.insert(walk.next(right));
            }
            // Check down
            let down = Pos(w.0, w.1 + 1);
            if basin.is_valid_pos(&down) && !basin.is_cyclone_at(&down, minute) {
                let mut n = walk.next(down);
                if down == basin.end {
                    if walk.at_end == false {
                        if first_end_minute == None {
                            first_end_minute = Some(minute);
                        }
                        n.at_end = true;
                    }
                    if walk.at_end && walk.back_at_start {
                        break 'outer;
                    }
                }
                next_walkers.insert(n);
            }
            // Wait
            if basin.is_valid_pos(&w) && !basin.is_cyclone_at(&w, minute) {
                next_walkers.insert(walk.next(walk.pos));
            }

        }
        println!("End of minute {} - {}", minute, next_walkers.len());
        walkers = next_walkers;
        if minute > 1000 {
            println!("TOO LONG");
            return
        }
    }
    println!("Part 1: End found in minute {}", first_end_minute.unwrap());
    println!("Part 2: End after returning found in minute {}", minute);
}