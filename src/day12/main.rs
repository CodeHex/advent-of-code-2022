use std::{fs, collections::HashMap, fmt};
use pathfinding::prelude::dijkstra;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

#[derive(Debug)]
struct Map {
    start: Pos,
    end: Pos,
    lowest: Vec<Pos>,
    heights: HashMap<Pos, usize>
}

impl Map {
    fn new(source: &str) -> Self {
        let mut start = Pos(0,0);
        let mut end = Pos(0,0);
        let mut lowest = Vec::new();

        let heights = source.lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x,c)| (x,y,c))
                    .collect::<Vec<(usize,usize,char)>>()
                })
            .flatten()
            .map(|(x,y,c)| (Pos(x,y), c))
            .inspect(|(p, c)| {
                if *c == 'S' {
                    start = *p
                } else if *c == 'E' {
                    end = *p
                } else if *c == 'a' {
                    lowest.push(*p)
                }
            })
            .map(|(p,c)| (p, Self::get_height(c)))
            .collect::<HashMap<Pos, usize>>();

        Map { start:start, end: end, lowest: lowest, heights: heights }
    }

    fn get_height(c_to_find: char) -> usize {
        let mut t = c_to_find;
        if c_to_find == 'S' {
            t = 'a'
        } else if c_to_find == 'E' {
            t = 'z'
        }

        static HEIGHT_REF: &str = "abcdefghijklmnopqrstuvwxyz";
        HEIGHT_REF.char_indices()
            .find(|(_, c)| *c == t)
            .unwrap().0
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl Pos {
    fn neighbours(&self, map: &Map) -> Vec<(Pos, usize)> {
      let &Pos(x, y) = self;

      let current_height = *map.heights.get(self).unwrap_or(&0);
      let mut positions = Vec::<Pos>::new();

      if y != 0 && (*map.heights.get(&Pos(x, y-1)).unwrap_or(&99999) <= current_height + 1 ){
        positions.push(Pos(x, y-1))
      }

      if *map.heights.get(&Pos(x, y+1)).unwrap_or(&99999) <= current_height + 1 {
        positions.push(Pos(x, y+1))
      }

      if x != 0 && *map.heights.get(&Pos(x-1, y)).unwrap_or(&99999) <= current_height + 1 {
        positions.push(Pos(x-1, y))
      }

      if *map.heights.get(&Pos(x+1, y)).unwrap_or(&99999) <= current_height + 1 {
        positions.push(Pos(x+1, y))
      }
      //println!("CURR {:?}:{} , POS{:?}",self, current_height, positions);
      positions.into_iter().map(|p| (p, 1)).collect()
      
    }
}


fn calc_path_cost(map: &Map, start: Pos) -> Option<usize> {
    let result = dijkstra(&start, |p| p.neighbours(&map), |p| *p == map.end);
    match result {
        None => None,
        Some((_, cost)) => Some(cost),
    }
}

fn main() {
    let input_data = fs::read_to_string("src/day12/input.txt").unwrap();

    let map = Map::new(&input_data);

    println!("Part 1 : Cost - {}", calc_path_cost(&map, map.start).unwrap());

    let mut start_positions = map.lowest.clone();
    start_positions.push(map.start);

    let lowest_cost = start_positions.iter()
        .map(|x| calc_path_cost(&map, *x))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .min()
        .unwrap();
    println!("Part 2 : Lowest Cost - {}", lowest_cost);
    


}