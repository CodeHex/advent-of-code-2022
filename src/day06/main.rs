use std::{fs, collections::HashSet};

fn start_of_packet(source: &str, marker_size: usize) -> usize {
    let input = source.chars().collect::<Vec<char>>();

    input.as_slice()
        .windows(marker_size)
        .map(|w| w.iter().map(|x|*x).collect::<HashSet<char>>())
        .enumerate()
        .find(|(_,set)| set.len() == marker_size)
        .map(|(idx, _)| idx + marker_size)
        .unwrap()
}

fn main() {
    let input_data = fs::read_to_string("src/day06/input.txt").unwrap();

    input_data.lines().for_each(|line| {
        println!("Part 1: Start index = {}", start_of_packet(line, 4))
    });

    input_data.lines().for_each(|line| {
        println!("Part 2: Start index = {:?}", start_of_packet(line, 14))
    });
}