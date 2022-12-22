use std::{fs, ops::Range};
use std::cmp;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

fn parse_line(source: &str) -> (Pos, Pos) {
    let sensor_regex = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();
    let caps: Vec<i32> = sensor_regex
        .captures(source)
        .unwrap()
        .iter()
        .skip(1) // Ignore the complete phrase match
        .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
        .collect();

    (Pos(caps[0],caps[1]), Pos(caps[2],caps[3]))
}

fn overlaps(r1: &Range<i32> ,r2: &Range<i32>) -> bool{
    (r2.start >= r1.start && r2.start <= r1.end)
    || (r2.end >= r1.start && r2.end <= r1.end)
}

fn merge(r1: &Range<i32>, r2: &Range<i32>) -> Range<i32>{
    cmp::min(r1.start, r2.start)..(cmp::max(r1.end, r2.end))
}

fn compress_ranges(mut ranges: Vec<Range<i32>>) -> Vec<Range<i32>>{
    let mut compressed = true;
    'outer: while compressed {
        compressed = false;
        for idx in 0..ranges.len() {
            for idx2 in 0..ranges.len() {
                if idx == idx2 {
                    continue
                }
                if overlaps(&ranges[idx], &ranges[idx2]) {
                    compressed = true;
                    let new = merge(&ranges[idx], &ranges[idx2]);
                    ranges.push(new);
                    ranges.swap_remove(idx);
                    ranges.remove(idx2);
                    continue 'outer
                }

            }
        }
    }
    ranges
}

fn calc_ranges(data: &Vec<(Pos,Pos)>, y:i32) -> Vec<Range<i32>> {
    let raw_ranges = data.iter()
        .map(|(s,b)| (s,b, (s.0 - b.0).abs() + (s.1 - b.1).abs() ))
        .filter(|(s,_,m)|(s.1 + m > y && s.1 - m < y))
        .map(|(s,_,m)| {
            let new_m = m - (y - s.1).abs();
            (s.0 - new_m)..(s.0+new_m)
        })
        .collect::<Vec<Range<i32>>>();

    compress_ranges(raw_ranges)
}

fn main() {
    let input_data = fs::read_to_string("src/day15/input.txt").unwrap();

    let data = input_data.lines()
        .map(|line| parse_line(line))
        .collect::<Vec<(Pos,Pos)>>();


    let ranges = calc_ranges(&data, 2000000);

    let points: usize = ranges.iter()
        .map(|r| r.len())
        .sum();

    println!("Part 1 - Total points: {}", points);

    for test_y in 0..4000000 {
        let ranges = calc_ranges(&data, test_y);
        if ranges.len() == 2 {
            let test_x = ranges[0].end + 1;
            println!("Part 2 - Tuning: {}", (test_x as i128 * 4000000) + test_y as i128);
            break
        }
    }
}