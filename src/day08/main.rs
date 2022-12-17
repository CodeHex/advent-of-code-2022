use std::fs;
use std::collections::HashSet;

struct Grid {
    data: Vec<Vec<u8>>
}

impl Grid {
    fn new<'a>(source: &'a str) -> Grid {
        let data = source.lines()
            .map(|line| line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect())
                .collect::<Vec<Vec<u8>>>();
        Grid { data: data }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        return self.data[y][x]
    }
}

fn main() {
    let input_data = fs::read_to_string("src/day08/input.txt").unwrap();

    let _ = Grid::new(&input_data);

    let grid = input_data.lines()
        .map(|line| line.chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut seen_trees: HashSet<(usize, usize)> = HashSet::new();
    let grid_size = grid[0].len();

    
    for j in 0..grid_size {
        let mut last: i32 = -1;
        for i in 0..grid_size {
            let tree = grid[j][i] as i32;
            if tree > last {
                last = tree;
                seen_trees.insert((i,j));
                if last == 9 {
                    break
                }
            }
        }
    }

    for j in 0..grid_size {
        let mut last: i32 = -1;
        for i in (0..grid_size).rev() {
            let tree = grid[j][i] as i32;
            if tree > last {
                last = tree;
                seen_trees.insert((i,j));
                if last == 9 {
                    break
                }
            }
        }
    }

    for i in 0..grid_size {
        let mut last: i32 = -1;
        for j in 0..grid_size {
            let tree = grid[j][i] as i32;
            if tree > last {
                last = tree;
                seen_trees.insert((i,j));
                if last == 9 {
                    break
                }
            }
        }
    }

    for i in 0..grid_size {
        let mut last: i32 = -1;
        for j in (0..grid_size).rev() {
            let tree = grid[j][i] as i32;
            if tree > last {
                last = tree;
                seen_trees.insert((i,j));
                if last == 9 {
                    break
                }
            }
        }
    }

    println!("{:?}", seen_trees.len());

    let mut max = 0;
    for j in 0..grid_size {
        for i in 0..grid_size {
            let tree = grid[j][i];

            let mut trees_up = 0;
            let mut j_look:i32 = j as i32 - 1;
            while j_look >= 0 {
                trees_up = trees_up + 1;
                if tree <= grid[j_look as usize][i] {
                    break;
                }
                j_look = j_look - 1;
            }

            let mut trees_down = 0;
            j_look = j as i32 + 1;
            while j_look < grid_size as i32 {
                trees_down = trees_down + 1;
                if tree <= grid[j_look as usize][i] {
                    break;
                }
                j_look = j_look + 1;
            }

            let mut trees_left = 0;
            let mut i_look:i32 = i as i32 - 1;
            while i_look >= 0 {
                trees_left = trees_left + 1;
                if tree <= grid[j][i_look as usize] {
                    break;
                }
                i_look = i_look - 1;
            }

            let mut trees_right = 0;
            let mut i_look:i32 = i as i32 + 1;
            while i_look < grid_size as i32{
                trees_right = trees_right + 1;
                if tree <= grid[j][i_look as usize] {
                    break;
                }
                i_look = i_look + 1;
            }

            let score = trees_up * trees_down * trees_left * trees_right;
            if max < score {
                max = score;
            }
        }
    }
    println!("MAX {}", max)
}