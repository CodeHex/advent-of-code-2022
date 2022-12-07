use std::{fs, collections::HashMap};

enum Cmd<'a> {
    CdRoot,
    CdUp,
    Cd(&'a str),
    Ls,
    Dir(&'a str),
    File(&'a str, u32)
}

impl Cmd<'_> {
    fn parse<'a>(line: &'a str) -> Cmd<'a>{
        if line.starts_with("$ cd") {
            let dir = line.strip_prefix("$ cd ").unwrap();
            match dir {
                "/" => return Cmd::CdRoot,
                ".." => return Cmd::CdUp,
                _ => return Cmd::Cd(dir)
            }
        } else if line == "$ ls" {
            return Cmd::Ls;
        } else if line.starts_with("dir") {
            return Cmd::Dir(line.strip_prefix("dir ").unwrap());
        } else {
            let (size_str, filename) = line.split_once(" ").unwrap();
            return Cmd::File(filename, size_str.parse().unwrap())
        }
    }
}

struct DirWalker<'a> {
    cmds: Vec<Cmd<'a>>,
    cwd: Vec<&'a str>,
    dir_sizes: HashMap<String, u32>,
}

impl DirWalker<'_> {
    fn new<'a>(cmds: Vec<Cmd<'a>>) -> DirWalker<'a> {
        DirWalker { cmds: cmds, cwd: vec!["/"], dir_sizes: HashMap::new() }
    }

    fn execute(&mut self) {
        for cmd in self.cmds.iter() {
            match cmd {
                Cmd::CdRoot => self.cwd = vec!["/"],
                Cmd::CdUp => _ = self.cwd.pop(),
                Cmd::Cd(dir) => self.cwd.push(dir),
                Cmd::Dir(_) => (),
                Cmd::Ls => (),
                Cmd::File(_,size) => DirWalker::add_file(&self.cwd, &mut self.dir_sizes, *size)
            }
        };
    }

    fn add_file(cwd: &Vec<&str>, dir_sizes: &mut HashMap<String, u32> , file_size: u32) {
        // Starting at the root directory, as the size to all containing folders
        let mut path = String::from("");
        cwd.iter().for_each(|x| {
            if path != "/" && *x != "/" {
                path.push_str("/");
            }
            path.push_str(x);
            dir_sizes.entry(path.to_owned())
                .and_modify(|y| *y += file_size)
                .or_insert(file_size);
        })
    }
}

fn main() {
    let input_data = fs::read_to_string("src/day07/input.txt").unwrap();
    let input = input_data
        .lines()
        .map(|line| Cmd::parse(line))
        .collect::<Vec<Cmd>>();

    let mut dir_walker = DirWalker::new(input);
    dir_walker.execute();

    let total: u32 = dir_walker.dir_sizes.values()
        .filter(|size| **size <= 100000 )
        .sum();

    println!("Part 1 : Total {}", total);

    const DISK_SIZE: u32 = 70000000;
    const REQ_SIZE: u32 = 30000000;
    let used_size =  dir_walker.dir_sizes.get("/").unwrap();
    let free_size = DISK_SIZE - used_size;
    let need_to_free = REQ_SIZE - free_size;

    let min_size =  dir_walker.dir_sizes.values()
        .filter(| size| **size > need_to_free)
        .min()
        .unwrap();

    println!("Part 2 : Min Size {}", min_size);
}