use std::fs;

fn main() {
    // Extract the top 3 values
    let top_3 = top_3_elves("src/day01/input.txt");
    let total_top_3: i32 = top_3.iter().sum();

    println!("Top 3 are {}, {} and {}", top_3[0], top_3[1], top_3[2]);
    println!("Total is {}", total_top_3);
}

// Returns the total calories of the top 3 elves (or less if there are less than 3 elves)
fn top_3_elves(path: &str) -> Vec<i32> {
    // Read file contents into a string
    let input = fs::read_to_string(path).unwrap();

    // Break the input into parts representing one elf
    let elves = input.split("\n\n");

    // Calculate the totals for each elf and order the results (descending)
    let mut sums: Vec<i32> = elves.map(|x| 
        x.lines().map(|l| l.parse::<i32>().unwrap()).sum()
    ).collect();
    sums.sort();

    // Extract the top 3 values
    sums.iter().rev().take(3).map(|x|*x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day01_errored_input() {
        let top_3 = top_3_elves("src/day01/error.txt");
        assert_eq!(top_3, [2, 1]);
    }

    #[test]
    fn day01_example() {
        let top_3 = top_3_elves("src/day01/example.txt");
        assert_eq!(top_3, [24000, 11000, 10000]);
    }
}
