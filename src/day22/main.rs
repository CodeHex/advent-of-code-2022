use std::{fs, collections::HashMap};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Tile {
    Open,
    Wall
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PathInstruction {
    TurnLeft,
    TurnRight,
    MoveForward(usize)
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl Dir {
    fn opposite(&self) -> Dir {
        match &self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Walker {
    pos: Pos,
    dir: Dir,
    is_cube: bool,
}

struct Map {
    start: Pos,
    data: HashMap<Pos, Tile>,
    cube_warps: HashMap<(Pos, Dir), (Pos, Dir)>,
}

impl Map {
    fn new(source: &str) -> Map {
        let points = Self::generate_map(source);
        let start_row = points.keys()
            .map(|p| p.1)
            .min()
            .unwrap(); 
        let start_column = points.keys()
            .filter(|p| p.1 == start_row)
            .map(|p| p.0)
            .min()
            .unwrap();

        let mut warps: HashMap<(Pos, Dir), (Pos, Dir)> = HashMap::new();
        if start_column == 8 {
            println!("USING EXAMPLE MAP");
            warps.extend( (0..4).map(|x| (Pos(x, 4), Pos(11 - x, 0)))
                .map(|(a, b)| [(a, (b, Dir::Down)), (b, (a, Dir::Down))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());
            
            warps.extend( (0..4).map(|y| (Pos(8, y), Pos(y + 4, 4)))
                .map(|(a, b)| [(a, (b, Dir::Down)), (b, (a, Dir::Right))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend( (0..4).map(|y| (Pos(11, y), Pos(15, 11-y)))
                .map(|(a, b)| [(a, (b, Dir::Left)), (b, (a, Dir::Left))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend( (0..4).map(|y| (Pos(11, y + 4), Pos(15-y, 8)))
                .map(|(a, b)| [(a, (b, Dir::Down)), (b, (a, Dir::Left))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend( (0..4).map(|x| (Pos(x + 12, 11), Pos(0, 7-x)))
                .map(|(a, b)| [(a, (b, Dir::Right)), (b, (a, Dir::Up))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend( (0..4).map(|x| (Pos(x + 8, 11), Pos(3-x, 7)))
                .map(|(a, b)| [(a, (b, Dir::Up)), (b, (a, Dir::Up))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend( (0..4).map(|y| (Pos(8, y+7), Pos(7-y, 7)))
                .map(|(a, b)| [(a, (b, Dir::Up)), (b, (a, Dir::Right))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());
            
        } else {
            warps.extend((0..50).map(|x| (Pos(x+50, 0), Pos(0,x+150)))
                .map(|(a, b)| [(a, (b, Dir::Right)), (b, (a, Dir::Down))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend((0..50).map(|x| (Pos(x+100, 0), Pos(x, 199)))
                .map(|(a, b)| [(a, (b, Dir::Up)), (b, (a, Dir::Down))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend((0..50).map(|y| (Pos(149, y), Pos(99, 149-y)))
                .map(|(a, b)| [(a, (b, Dir::Left)), (b, (a, Dir::Left))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend((0..50).map(|x| (Pos(100+x, 49), Pos(99, 50+x)))
                .map(|(a, b)| [(a, (b, Dir::Left)), (b, (a, Dir::Up))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend((0..50).map(|x| (Pos(50+x, 149), Pos(49, 150+x)))
                .map(|(a, b)| [(a, (b, Dir::Left)), (b, (a, Dir::Up))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend((0..50).map(|y| (Pos(0, 100+y), Pos(50, 49-y)))
                .map(|(a, b)| [(a, (b, Dir::Right)), (b, (a, Dir::Right))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());

            warps.extend((0..50).map(|x| (Pos(x, 100), Pos(50, x+50)))
                .map(|(a, b)| [(a, (b, Dir::Right)), (b, (a, Dir::Down))])
                .map(|x| [((x[0].0, x[1].1.1.opposite()), x[0].1), ((x[1].0, x[0].1.1.opposite()) , x[1].1)] )
                .flatten().collect::<HashMap<(Pos, Dir), (Pos, Dir)>>());
        }
        Map { start: Pos(start_column, start_row), data: points, cube_warps: warps }
    }

    fn generate_map(source: &str) -> HashMap<Pos, Tile> {
        source.lines()
            .enumerate()
            .map(|(row_idx,  line)| {
                line.char_indices()
                    .filter(|(_, c)| *c != ' ')
                    .map(move |(column_idx, c)| {
                        let p = Pos(column_idx as i32, row_idx as i32);
                        match c {
                            '.' => (p, Tile::Open),
                            '#' => (p, Tile::Wall),
                            _ => panic!("unrecogized space")
                        }
                    })
            })
            .flatten()
            .collect::<HashMap<Pos, Tile>>()
    }
}

impl Walker {
    fn new(map: &Map, cube_wrap: bool) -> Walker {
        Walker { pos: map.start, dir: Dir::Right, is_cube: cube_wrap }
    }

    fn turn_left(&mut self) {
        match self.dir {
            Dir::Up => self.dir = Dir::Left,
            Dir::Down => self.dir = Dir::Right,
            Dir::Left => self.dir = Dir::Down,
            Dir::Right => self.dir = Dir::Up,
        }
    }

    fn turn_right(&mut self) {
        match self.dir {
            Dir::Up => self.dir = Dir::Right,
            Dir::Down => self.dir = Dir::Left,
            Dir::Left => self.dir = Dir::Up,
            Dir::Right => self.dir = Dir::Down,
        }
    }

    fn move_forward(&mut self, map: &Map, steps: &usize) {
        for _ in 0..*steps {
            let mut next_move: Pos;
            let mut next_dir = self.dir;
            match self.dir {
                Dir::Up => next_move = Pos(self.pos.0, self.pos.1 - 1),
                Dir::Down => next_move = Pos(self.pos.0, self.pos.1 + 1),
                Dir::Left => next_move = Pos(self.pos.0 - 1, self.pos.1),
                Dir::Right => next_move = Pos(self.pos.0 + 1, self.pos.1),
            }
            if !map.data.contains_key(&next_move) {
                if self.is_cube {
                    (next_move, next_dir) = *map.cube_warps.get(&(self.pos, self.dir)).unwrap();
                } else {
                    next_move = self.flat_wrap(&map, &next_move);
                }
            }

            match map.data.get(&next_move) {
                Some(Tile::Open) => {
                    self.pos = next_move;
                    self.dir = next_dir;
                },
                Some(Tile::Wall) => return,
                None => panic!("unexpected, should have wrapped {:?}", next_move),
            }
        }
    }

    fn flat_wrap(&self, map: &Map, next: &Pos) -> Pos {
        let mut next_move = *next;
        match self.dir {
            Dir::Up => next_move.1 = map.data.keys().filter(|p|p.0 == self.pos.0).map(|p| p.1).max().unwrap(),
            Dir::Down => next_move.1 = map.data.keys().filter(|p|p.0 == self.pos.0).map(|p| p.1).min().unwrap(),
            Dir::Left => next_move.0 = map.data.keys().filter(|p|p.1 == self.pos.1).map(|p| p.0).max().unwrap(),
            Dir::Right => next_move.0 = map.data.keys().filter(|p|p.1 == self.pos.1).map(|p| p.0).min().unwrap(),
        }
        next_move
    }
}

fn generate_instructions(source: &str) -> Vec<PathInstruction> {
    let (last_num, mut dirs) = source.trim().chars()
    .fold((Vec::<char>::new(), Vec::<PathInstruction>::new()), |mut acc, x| {
        if x == 'L' || x == 'R' {
            if acc.0.len() != 0 {
                let steps: usize  = acc.0.iter().collect::<String>().parse().unwrap();
                acc.1.push(PathInstruction::MoveForward(steps));
                acc.0.clear();
            }
            if x == 'L' {
                acc.1.push(PathInstruction::TurnLeft)
            } else {
                acc.1.push(PathInstruction::TurnRight)
            }
        } else {
            acc.0.push(x)
        }
        acc
    });
    if last_num.len() != 0 {
        let steps: usize  = last_num.iter().collect::<String>().parse().unwrap();
        dirs.push(PathInstruction::MoveForward(steps))
    }
    dirs
}

fn main() {
    let input_data = fs::read_to_string("src/day22/input.txt").unwrap();
    let (map_source, path_source) = input_data.split_once("\n\n").unwrap();

    let map = Map::new(map_source);
    let instructions = generate_instructions(path_source);

    let mut w = Walker::new(&map, false);
    instructions.iter().for_each(|x| {
        match x {
            PathInstruction::TurnLeft => w.turn_left(),
            PathInstruction::TurnRight => w.turn_right(),
            PathInstruction::MoveForward(steps) => w.move_forward(&map, steps),
        }
    });

    let row = w.pos.1 + 1;
    let col = w.pos.0 + 1;
    let dir = match w.dir {
        Dir::Right => 0,
        Dir::Down => 1,
        Dir::Left => 2,
        Dir::Up => 3,
    };
    
    println!("Part 1 : Row = {}, Columm = {}, Dir = {}, Answer = {}", 
        row, col, dir, (1000 * row) + (4 * col) + dir);

    let mut w2 = Walker::new(&map, true);
    instructions.iter().for_each(|x| {
        match x {
            PathInstruction::TurnLeft => w2.turn_left(),
            PathInstruction::TurnRight => w2.turn_right(),
            PathInstruction::MoveForward(steps) => w2.move_forward(&map, steps),
        }
    });

    let row2 = w2.pos.1 + 1;
    let col2 = w2.pos.0 + 1;
    let dir2 = match w2.dir {
        Dir::Right => 0,
        Dir::Down => 1,
        Dir::Left => 2,
        Dir::Up => 3,
    };
    
    println!("Part 2 : Row = {}, Columm = {}, Dir = {}, Answer = {}", 
        row2, col2, dir2, (1000 * row2) + (4 * col2) + dir2)
        
}