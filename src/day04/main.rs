use std::{fmt, fs};
struct CleanRange {
    start: u32,
    end: u32,
}

impl CleanRange {
    fn new(source: &str) -> CleanRange {
        let (start_str, end_str) = source.split_once("-").unwrap();
        CleanRange {
            start: start_str.parse().unwrap(),
            end: end_str.parse().unwrap(),
        }
    }

    fn contains(&self, target: &CleanRange) -> bool {
        return target.start >= self.start && target.end <= self.end;
    }

    fn overlaps(&self, target: &CleanRange) -> bool {
        let seperate = self.end < target.start || self.start > target.end;
        !seperate
    }
}

impl fmt::Display for CleanRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <-> {}", self.start, self.end)
    }
}

fn main() {
    let input_data = fs::read_to_string("src/day04/input.txt").unwrap();

    let cleaning_ranges: Vec<(CleanRange, CleanRange)> = input_data
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(elf1_str, elf2_str)| (CleanRange::new(elf1_str), CleanRange::new(elf2_str)))
        .collect();

    let contains_total = cleaning_ranges
        .iter()
        .filter(|(elf1, elf2)| elf1.contains(&elf2) || elf2.contains(&elf1))
        .count();

    println!("Part 1 Containing ranges: {}", contains_total);

    let overlap_total = cleaning_ranges
        .iter()
        .filter(|(elf1, elf2)| elf1.overlaps(&elf2))
        .count();

    println!("Part 2 Overlapping ranges: {}", overlap_total);
}
