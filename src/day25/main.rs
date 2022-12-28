use std::fs;

fn from_snafu(num: &str) -> i64 {
    let mut total = 0;
    let mut unit = 1;
    for c in num.chars().rev() {
        match c {
            '=' => total += -2 * unit,
            '-' => total += -1 * unit,
            '0' => (),
            '1' => total += 1 * unit,
            '2' => total += 2 * unit,
            _ => panic!("unrecognized number")
        }
        unit = unit * 5;
    }
    total
}

fn to_snafu(n: i64) -> String {
    let mut result = Vec::<char>::new();
    let mut unit = 1;
    loop {
        if unit * -2 <= n && n <= unit * 2 {
            break;
        }
        unit *= 5;
    }
    let mut x = n;
    loop {
        let two_units = unit * 2;
        let half_unit = unit / 2;
        if two_units + half_unit >= x && x >= two_units - half_unit  {
            result.push('2');
            x = x - two_units;
        } else if unit + half_unit  >= x && x > half_unit {
            result.push('1');
            x = x - unit;
        } else if  half_unit >= x && x >= -half_unit {
            result.push('0');
        } else if -unit - half_unit  <= x && x < -half_unit {
            result.push('-');
            x = x + unit;
        } else if -two_units - half_unit  <= x && x <= -two_units + half_unit {
            result.push('=');
            x = x + two_units;
        } else {
            panic!("what!")
        }
        if unit == 1 {
            break;
        }
        unit /= 5;
    }
    result.iter().collect()

}


fn main() {
    let input_data = fs::read_to_string("src/day25/input.txt").unwrap();

    let total = input_data.lines()
        .map(|line| from_snafu(line))
        .sum();

    println!("Part 1: Sum is {} -> {}", total, to_snafu(total));
}