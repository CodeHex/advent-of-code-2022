use std::collections::HashMap;
use std::fmt;
use std::fs;

struct File {
    original: Vec<i64>,
    mixer: Vec<usize>,
    zero_idx: usize,
    cache_loops: HashMap<i64, usize>
}
impl File {
    fn new(source: &str, decryption_key: i64) -> Self {
        let org = source.lines()
            .map(|x| x.parse::<i64>().unwrap() * decryption_key)
            .collect::<Vec<i64>>();
        let mixer =  (0..org.len()).collect();
        let z = org.iter().position(|x| *x == 0).unwrap();
        File { 
            original: org, 
            mixer: mixer, 
            zero_idx: z, 
            cache_loops: HashMap::<i64,usize>::new() 
        }
    }

    fn loop_in_range(&mut self, pos: i64) -> usize {
        let cached = self.cache_loops.get(&pos);
        if cached.is_some() {
            return *cached.unwrap();
        }
        let i = pos.rem_euclid((self.original.len() - 1) as i64);     
        self.cache_loops.insert(pos, i as usize);
        i as usize
    }

    fn get_value(&self, pos: i64) -> i64 {
        let zero_pos = self.mixer.iter().position(|x| *x == self.zero_idx).unwrap() as i64;
        let val_pos = (pos + zero_pos).rem_euclid(self.original.len() as i64);
        self.original[self.mixer[val_pos as usize]]
    }

    fn mix(&mut self) {
        for idx in 0..self.original.len() {
            let val = self.original[idx];
            let old_idx = self.mixer.iter().position(|x| *x == idx).unwrap();
            let new_idx = self.loop_in_range( val + old_idx as i64);
            if new_idx < old_idx {
                // move left
                //println!("MOVE LEFT - [{},{}] {}", old_idx, new_idx, val);
                for i in (new_idx..old_idx).rev() {
                    self.mixer[i+1] = self.mixer[i]
                }
                self.mixer[new_idx] = idx
            } else {
                // move right
                //println!("MOVE RIGHT - [{},{}] {}", old_idx, new_idx, val);
                for i in old_idx..new_idx {
                    self.mixer[i] = self.mixer[i+1]
                }
                self.mixer[new_idx] = idx;
            }
            //println!("{}", self)
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let top = self.original.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let bottom = self.mixer.iter()
            .map(|x| self.original[*x as usize].to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}\n{}",top,bottom)
    }
}

fn main() {
    let input_data = fs::read_to_string("src/day20/input.txt").unwrap();

    let mut encrypted_file1 = File::new(&input_data, 1);
    encrypted_file1.mix();

    let coord1 = encrypted_file1.get_value(1000);
    let coord2 = encrypted_file1.get_value(2000);
    let coord3 = encrypted_file1.get_value(3000);
    let total = coord1 + coord2 + coord3;
    println!("Part 1 - Coords [{},{},{}] = {}", coord1, coord2, coord3, total);

    let mut encrypted_file = File::new(&input_data, 811589153);
    (1..=10).for_each(|i| {
        encrypted_file.mix();
        println!("..mixed {}", i)
    });

    let coord2_1 = encrypted_file.get_value(1000);
    let coord2_2 = encrypted_file.get_value(2000);
    let coord2_3 = encrypted_file.get_value(3000);
    let total2 = coord2_1 + coord2_2 + coord2_3;
    println!("Part 2 - Coords [{},{},{}] = {}", coord2_1, coord2_2, coord2_3, total2);
}