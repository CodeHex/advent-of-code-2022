use std::{fs, collections::HashSet};
use std::fmt;

enum Dir { Up, Down, Left, Right }

struct Step{ d: Dir, steps: u16 }

impl Step {
    fn new(source: &str) -> Self {
        let (dir_str, count_str) = source.split_once(" ").unwrap();
        let count: u16 = count_str.parse().unwrap();
        let dir = match dir_str {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("unrecognized direction")
        };
        Step { d:dir, steps: count }
        
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.d {
            Dir::Up => write!(f, "Up {}", self.steps),
            Dir::Down => write!(f, "Down {}", self.steps),
            Dir::Left => write!(f, "Left {}", self.steps),
            Dir::Right => write!(f, "Right {}", self.steps),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {x:i32, y:i32}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Pos {
    fn next_to(&self, other: &Pos) -> bool {
        let xdiff = (self.x - other.x).abs();
        let ydiff = (self.y-other.y).abs();
        xdiff <=1 && ydiff <=1
    }
}

struct Walker {
    head : Pos,
    tail: Vec<Pos>,
    visited : HashSet<Pos>
}

impl Walker {
    fn new(tail_len: u16) -> Walker {
        let t: Vec<Pos> = (0..tail_len)
            .map(|_| Pos{x:0, y:0})
            .collect();

        Walker { head: Pos{x: 0,y: 0}, tail: t, visited: HashSet::new() }
    }

    fn move_head(&mut self, d: &Dir) {
        match d {
            Dir::Up => self.head.y += 1,
            Dir::Down => self.head.y -= 1,
            Dir::Left => self.head.x -= 1,
            Dir::Right => self.head.x += 1,
        }
    }

    fn move_tail(&mut self) {
        let mut tail_idx = 0;
        let mut seg_top = self.head;

        while tail_idx < self.tail.len() {

            if !seg_top.next_to(&self.tail[tail_idx]) {
                let xdiff = seg_top.x - self.tail[tail_idx].x;
                let ydiff = seg_top.y - self.tail[tail_idx].y;

                if xdiff == 2 && ydiff == 2 {
                    self.tail[tail_idx].x = seg_top.x - 1;
                    self.tail[tail_idx].y = seg_top.y - 1;
                } else if xdiff == -2 && ydiff == -2 {
                    self.tail[tail_idx].x = seg_top.x + 1;
                    self.tail[tail_idx].y = seg_top.y + 1;
                } else if xdiff == 2 && ydiff == -2 {
                    self.tail[tail_idx].x = seg_top.x - 1;
                    self.tail[tail_idx].y = seg_top.y + 1;
                } else if xdiff == -2 && ydiff == 2 {
                    self.tail[tail_idx].x = seg_top.x + 1;
                    self.tail[tail_idx].y = seg_top.y - 1;
                } else if xdiff > 1 {
                    self.tail[tail_idx].x = seg_top.x - 1;
                    self.tail[tail_idx].y = seg_top.y
                } else if xdiff < -1 {
                    self.tail[tail_idx].x = seg_top.x + 1;
                    self.tail[tail_idx].y = seg_top.y
                } else if ydiff > 1 {
                    self.tail[tail_idx].x = seg_top.x;
                    self.tail[tail_idx].y = seg_top.y - 1
                } else {
                    self.tail[tail_idx].x = seg_top.x;
                    self.tail[tail_idx].y = seg_top.y + 1
                }
            }
            seg_top = self.tail[tail_idx];
            tail_idx += 1;
        }
        self.visited.insert(self.tail[tail_idx-1].clone().to_owned());

    }

    fn walk(&mut self, action: &Step) {
        for _ in 0..action.steps {
            self.move_head(&action.d);
            self.move_tail();
        }
    }
}


fn main() {
    let input_data = fs::read_to_string("src/day09/input.txt").unwrap();
    let instructions:Vec<Step> = input_data.lines()
        .map(|line| Step::new(line))
        .collect();

    let mut walker1 = Walker::new(1);
    instructions.iter().for_each(|i| walker1.walk(i));

    println!("PART 1: Visited coords - {}", walker1.visited.len());

    let mut walker2 = Walker::new(9);
    instructions.iter().for_each(|i| walker2.walk(i));
    
    println!("PART 2: Visited coords - {}", walker2.visited.len());
}