use std::{fs, collections::HashMap};

enum Cmd {
    CdRoot,
    CdUp,
    Cd(String),
    Ls,
    Dir(String),
    File(String, u32)
}

impl Cmd {
    fn parse(line: &str) -> Cmd{
        if line.starts_with("$ cd") {
            let dir = line.strip_prefix("$ cd ").unwrap();
            match dir {
                "/" => return Cmd::CdRoot,
                ".." => return Cmd::CdUp,
                _ => return Cmd::Cd(dir.to_string())
            }
        } else if line == "$ ls" {
            return Cmd::Ls;
        } else if line.starts_with("dir") {
            return Cmd::Dir(line.strip_prefix("dir ").unwrap().to_string());
        } else {
            let (size_str, filename) = line.split_once(" ").unwrap();
            return Cmd::File(filename.to_string(), size_str.parse().unwrap())
        }
    }
}

struct DirWalker {
    cwd: Vec<String>,
    dir_sizes: HashMap<String, u32>,
}

impl DirWalker {
    fn new() -> DirWalker {
        DirWalker { cwd: vec!["/".to_string()], dir_sizes: HashMap::new() }
    }

    fn execute(&mut self, cmd: &Cmd) {
        match cmd {
            Cmd::CdRoot => self.cwd = vec!["/".to_string()],
            Cmd::CdUp => _ = self.cwd.pop(),
            Cmd::Cd(dir) => self.cwd.push(dir.clone() + "/"),
            Cmd::Dir(_) => (),
            Cmd::Ls => (),
            Cmd::File(_,size) => self.add_file(*size)
        }
    }

    fn add_file(&mut self, file_size: u32) {
        // Starting at the root directory, as the size to all containing folders
        let mut path = String::from("");
        self.cwd.iter().for_each(|x| {
            path.push_str(x);
            self.dir_sizes.entry(path.clone())
                .and_modify(|y| *y += file_size)
                .or_insert(file_size);
        })
    }
}



fn main() {
    let input = fs::read_to_string("src/day07/input.txt").unwrap()
        .lines()
        .map(|line| Cmd::parse(line))
        .collect::<Vec<Cmd>>();

    let mut dir_walker = DirWalker::new();
    input.iter().for_each(|cmd| dir_walker.execute(cmd));

    let total: u32 = dir_walker.dir_sizes.iter()
        .map(|(_, size)| *size)
        .filter(|size| *size <= 100000 )
        .sum();

    println!("Part 1 : Total {}", total);

    const DISK_SIZE: u32 = 70000000;
    const REQ_SIZE: u32 = 30000000;
    let used_size =  dir_walker.dir_sizes.get("/").unwrap();
    let free_size = DISK_SIZE - used_size;
    let need_to_free = REQ_SIZE - free_size;

    let min_name =  dir_walker.dir_sizes.values()
        .filter(| size| **size > need_to_free)
        .min()
        .unwrap();

    println!("Part 2 : Min Size {}", min_name);
}