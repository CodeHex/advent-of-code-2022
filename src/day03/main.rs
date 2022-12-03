use std::fs;
use std::collections::HashSet;

fn convert_to_digit(c: char) -> u32 {
    match c {
        lower if lower.is_ascii_lowercase() => lower as u32 - 96,
        upper if upper.is_ascii_uppercase() => upper as u32 - 38,
        _ => panic!("Invalid character"),
    }
}

fn bisect(source: &str) -> (&str, &str) {
    let mid = source.len() / 2;
    return source.split_at(mid);
}

type ItemSet = HashSet<char>;

// Finds the intersection for all given item sets. Errors if there is more than 1
// duplicate item across all item sets
fn single_duplicate(sets: &[ItemSet]) -> Result<char, &'static str> {
    let intersect = sets.iter()
        .fold(None, |acc, x| {
            match acc {
                None => Some(x.clone()),
                Some(acc) => Some(acc.intersection(x).cloned().collect()),
            }
        })
        .unwrap();

    
    match intersect.len() {
        0 => Err::<char, &str>("No duplicates"),
        1 => Ok(intersect.iter().next().unwrap().clone()),
        _ => Err::<char, &str>("Multiple duplicates"),
    }
}


fn main() {
    let input_data = fs::read_to_string("src/day03/input.txt").unwrap();

    let backpack_compartments = input_data
        .lines()
        .map(|line| bisect(line))
        .map(|(left, right)| [left.chars().collect(), right.chars().collect()])
        .collect::<Vec<[ItemSet; 2]>>();
   

    let duplicate_total:u32 = backpack_compartments
        .iter()
        .map(|backpack| single_duplicate(backpack).unwrap())
        .map(|x| convert_to_digit(x))
        .sum();

    println!("Part 1 - Duplicate items priority sum: {}", duplicate_total);

    let group_total:u32 = input_data
        .lines()
        .map(|raw_backpack| raw_backpack.chars().collect())
        .collect::<Vec<ItemSet>>()
        .chunks(3)
        .map(|group| single_duplicate(group).unwrap() )
        .map(|x| convert_to_digit(x))
        .sum();

        println!("Part 2 - Group Sum: {}", group_total);
}