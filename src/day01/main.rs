use std::fs;

fn main() {
    // Read file contents into a string
    let input = fs::read_to_string("src/day01/input.txt").unwrap();

    // Break the string into each part representing one elf
    let elves = input.split("\n\n");

    // Calculate the totals for each elf and order the results (descending)
    let mut sums: Vec<i32> = elves.map(|x| 
        x.lines().map(|l| l.parse::<i32>().unwrap()).sum()
    ).collect();
    sums.sort();

    // Extract the top 3 values
    let top_3 = &sums.as_slice()[sums.len()-3..];
    let total_tot_3: i32 = top_3.iter().sum();

    println!("Top 3 are {}, {} and {}", top_3[0], top_3[1], top_3[2]);
    println!("Total is {}", total_tot_3);
}
