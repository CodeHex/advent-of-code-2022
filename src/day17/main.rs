use std::{fs, collections::HashSet, collections::HashMap};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Memory{
    floor: [i32; 7],
    jet_idx: usize,
    rock_idx: usize,
}

struct Cavern {
    jetstream_idx: usize,
    jetstream: String,
    rockstream_idx: usize,
    rockpixels: HashSet<Pos>,
    initial_records: HashMap<usize, i32>,
    initial_section_max: usize,
    initial_size:i32,
    repeat_records: HashMap<usize, i32>,
    repeat_section_size: usize,
    repeat_section_increase: i32
}

impl Cavern {
    fn new(stream: &str) -> Cavern {
        let mut c = Cavern { 
            jetstream_idx: 0, 
            jetstream: stream.to_string(),
            rockstream_idx: 0,
            rockpixels: HashSet::new(),
            initial_records: HashMap::new(),
            initial_section_max: 0,
            initial_size: 0,
            repeat_records: HashMap::new(),
            repeat_section_size: 0,
            repeat_section_increase: 0,
        };
        c.init();
        c
    }

    fn init(&mut self) {
        let mut drops:usize = 0;
        let mut history = HashMap::<usize, i32>::new();
        let mut mem = HashMap::<Memory, usize>::new();

        loop {
            self.drop();
            drops += 1;
            history.insert(drops, self.height());

            let m = Memory{
                floor: self.get_floor(),
                jet_idx: self.jetstream_idx,
                rock_idx: self.rockstream_idx,
            };

            if mem.contains_key(&m) { 
                self.initial_section_max = *mem.get(&m).unwrap();
                self.repeat_section_size = drops - self.initial_section_max;
                break
            }
            mem.insert(m, drops);
        }

        self.initial_records = history.iter()
            .filter(|(k,_)| **k <= self.initial_section_max)
            .map(|(k,v)| (*k, *v))
            .collect();

        let min_val = history.get(&(self.initial_section_max)).unwrap();
        self.repeat_records = history.iter()
            .filter(|(k,_)| **k > self.initial_section_max)
            .map(|(k,v)| (*k - self.initial_section_max, *v - min_val))
            .collect();
        
        self.repeat_section_increase = self.height() - history.get(&(self.initial_section_max)).unwrap();
        self.initial_size = *self.initial_records.get(&(self.initial_section_max)).unwrap();
    }

    fn drop(&mut self) {
        // Get next rock
        let mut rock = Rock::new(RockType::from(self.rockstream_idx));
        self.rockstream_idx = (self.rockstream_idx + 1).rem_euclid(5);

        // Move rock to starting position
        rock.to_start_position(self.height());

        let mut stopped = false;
        while !stopped {
            // Push rock
            let next_dir = self.next_jet();
            rock.move_dir(&next_dir, &self.rockpixels);
 
            // drop down
            stopped = !rock.move_dir(&Dir::Down, &self.rockpixels);
        }

        // Add rock to structure
        self.add_stopped_rock(&rock);
    }

    fn height(&self) -> i32 {
        self.rockpixels.iter().map(|p| p.1 + 1).max().unwrap_or(0)
    }

    fn calc_height(&self, drops: usize) -> i64 {
        if drops <= self.initial_section_max {
            return *self.initial_records.get(&drops).unwrap() as i64;
        } else {
            let leftover = drops - self.initial_section_max;
            let reps = leftover / self.repeat_section_size;
            let rem = leftover.rem_euclid(self.repeat_section_size);
            let mut rem_val: i64 = 0;
            if rem != 0 {
                rem_val = *self.repeat_records.get(&(rem)).unwrap() as i64
            }
            let init = self.initial_size as i64;
            init + (reps as i64 * self.repeat_section_increase as i64) + rem_val
        }
    }

    fn next_jet(&mut self) -> Dir {
        let c = self.jetstream.chars().nth(self.jetstream_idx).unwrap();
        self.jetstream_idx = (self.jetstream_idx+1).rem_euclid(self.jetstream.len());
        match c {
            '>' => Dir::Right,
            '<' => Dir::Left,
            _ => panic!("unexpected char {}", c)
        }
    }

    fn add_stopped_rock(&mut self, r: &Rock) {
        r.pixels.iter().for_each(|p| {
            self.rockpixels.insert(*p);
        })
    }

    fn get_floor(&self) -> [i32; 7] {
        let mut result: [i32; 7] = [1,1,1,1,1,1,1];
        let level = self.height() - 1;
        let mut current = self.height() - 1;
        while result.iter().any(|x| *x == 1) {
            self.rockpixels.iter().filter(|x| x.1 == current)
                .for_each(|x| {
                    if result[x.0 as usize] == 1 {
                        result[x.0 as usize] = x.1 - level;
                    }
                });
            current -= 1;
            if current == -1 {
                break
            }
        }
        result
    }
}

#[derive(Debug)]
enum RockType {
    HorizontalLine = 0,
    Plus = 1,
    LShape = 2,
    VerticalLine = 3,
    Square = 4,
}

impl From<usize> for RockType {
    fn from(value: usize) -> Self {
        let val = value.rem_euclid(5);
        match val {
            0 => RockType::HorizontalLine,
            1 => RockType::Plus,
            2 => RockType::LShape,
            3 => RockType::VerticalLine,
            4 => RockType::Square,
            _ => panic!("unexpect value")
        }
    }
}


impl RockType {
    fn pixels(&self) -> HashSet<Pos> {
        match self {
            RockType::HorizontalLine => HashSet::from([Pos(0,0),Pos(1,0),Pos(2,0),Pos(3,0)]),
            RockType::Plus  => HashSet::from([Pos(0,1),Pos(1,0),Pos(1,1),Pos(1,2),Pos(2,1)]),
            RockType::LShape  => HashSet::from([Pos(0,0),Pos(1,0),Pos(2,0),Pos(2,1),Pos(2,2)]),
            RockType::VerticalLine  => HashSet::from([Pos(0,0),Pos(0,1),Pos(0,2),Pos(0,3)]),
            RockType::Square  => HashSet::from([Pos(0,0),Pos(0,1),Pos(1,0),Pos(1,1)]),
        }
    }
}

struct Rock {
    pixels: HashSet<Pos>,
}

impl Rock {
    fn new(t: RockType) -> Rock {
        let pixels = t.pixels();
        Rock { 
            pixels: pixels,
        }
    }

    fn translate_pixels(&mut self, x:i32, y:i32, rockpixels: &HashSet<Pos>) -> bool {
        let org_pixels = self.pixels.iter().collect::<Vec<&Pos>>();
        let mut new_pixels = HashSet::<Pos>::new();

        for p in org_pixels {
            let n = Pos(p.0 + x, p.1 + y);
            if n.0 < 0 || n.0 >= 7 || n.1 < 0 {
                return false
            }
            new_pixels.insert(n);
        }
        if rockpixels.intersection(&new_pixels).count() != 0 {
            return false;
        }

        self.pixels = new_pixels;
        true
    }

    fn to_start_position(&mut self, top: i32) {
        self.translate_pixels(2, top + 3, &HashSet::new());
    }

    fn move_dir(&mut self, d: &Dir, rockpixels: &HashSet<Pos>) -> bool {
        match d {
            Dir::Right => self.translate_pixels(1, 0, rockpixels),
            Dir::Left => self.translate_pixels(-1, 0, rockpixels),
            Dir::Down => self.translate_pixels(0, -1, rockpixels),
        }
    }


}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
    Down
}

fn main() {
    let input_data = fs::read_to_string("src/day17/input.txt").unwrap();
    let cavern = Cavern::new(&input_data.trim());
    println!("Part 1 -  Height of rocks at 2022: {:?}", cavern.calc_height(2022));
    println!("Part 2 -  Height of rocks at 1000000000000: {:?}", cavern.calc_height(1000000000000));
}