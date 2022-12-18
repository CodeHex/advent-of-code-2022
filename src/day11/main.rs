
struct Monkey {
    items: Vec<i128>,
    op: fn(i128) -> i128,
    count: i128,
    test: i128,
    test_passed: usize,
    test_failed: usize,
}

impl Monkey {
    fn new(items: Vec<i128>, op: fn(i128) -> i128, test: i128, p: usize, f:usize) -> Self {
        Monkey { items: items, op: op, count: 0, test: test, test_passed: p, test_failed: f }
    }
}

fn sq(x: i128, m: i128) -> i128 {
    mult(x, x, m)
}

fn mult(x: i128, y: i128, m: i128) -> i128 {
    ((x % m) * (y % m)) % m
}

fn add(x: i128, y: i128, m: i128) -> i128 {
    ((x % m) + (y % m)) % m
}

fn play_rounds(monkeys: &mut Vec<Monkey>, rounds: usize, divide_by_3: bool) {
    for _ in 0..rounds {
        for m_idx in 0..monkeys.len() {
            
            // Transfer ownership of items out of monkeys and replace with a cleared version
            let items = monkeys.get_mut(m_idx).unwrap().items.to_owned();
            monkeys.get_mut(m_idx).unwrap().items = Vec::new();

            // Get an address to the monkey (but we won't be modifying it)
            let current = monkeys.get(m_idx).unwrap();
            let pass_idx = current.test_passed;
            let fail_idx = current.test_failed;
            let op  = current.op;
            let test = current.test;

            items.iter().for_each(|item| {
                let mut worry = (op)(*item);
                if divide_by_3 {
                    worry = worry / 3;
                }
                if worry % test == 0 {
                    monkeys.get_mut(pass_idx).unwrap().items.push(worry);
                } else {
                    monkeys.get_mut(fail_idx).unwrap().items.push(worry);
                }
            });
            monkeys.get_mut(m_idx).unwrap().count += items.len() as i128;
        }
    }
}

fn calc_score(monkeys: &Vec<Monkey>) -> i128 {
    let mut counts:Vec<i128> = monkeys.iter()
        .map(|m| m.count)
        .collect();

    counts.sort();

    counts.iter()
        .rev()
        .take(2)
        .product()
}

fn main() {
    // let example = vec![
    //     Monkey::new(vec![79,98], |x| mult(x,19,96577), 23, 2, 3),
    //     Monkey::new(vec![54,65,75,74], |x| add(x,6,96577), 19, 2, 0),
    //     Monkey::new(vec![79,60,97], |x| sq(x,96577), 13, 1, 3),
    //     Monkey::new(vec![74], |x| add(x,3,96577), 17, 0, 1),
    // ];

    let input1 = vec![
        Monkey::new(vec![65,78],|x| mult(x,3,9699690), 5, 2, 3),
        Monkey::new(vec![54,78,86,79,73,64,85,88],|x| add(x,8,9699690), 11, 4, 7),
        Monkey::new(vec![69,97,77,88,87], |x| add(x,2,9699690), 2, 5, 3),
        Monkey::new(vec![99], |x| add(x,4,9699690), 13, 1, 5),
        Monkey::new(vec![60, 57, 52], |x| mult(x,19,9699690), 7, 7, 6),
        Monkey::new(vec![91, 82, 85, 73, 84, 53], |x| add(x,5,9699690), 3, 4, 1),
        Monkey::new(vec![88, 74, 68, 56], |x| sq(x, 9699690), 17, 0, 2),
        Monkey::new(vec![54, 82, 72, 71, 53, 99, 67], |x| add(x,1,9699690), 19, 6, 0),
    ];

    let mut monkeys1 = input1;
    play_rounds(&mut monkeys1, 20, true);
    println!("Part 1 : Score - {}", calc_score(&monkeys1));

    let input2 = vec![
        Monkey::new(vec![65,78],|x| mult(x,3,9699690), 5, 2, 3),
        Monkey::new(vec![54,78,86,79,73,64,85,88],|x| add(x,8,9699690), 11, 4, 7),
        Monkey::new(vec![69,97,77,88,87], |x| add(x,2,9699690), 2, 5, 3),
        Monkey::new(vec![99], |x| add(x,4,9699690), 13, 1, 5),
        Monkey::new(vec![60, 57, 52], |x| mult(x,19,9699690), 7, 7, 6),
        Monkey::new(vec![91, 82, 85, 73, 84, 53], |x| add(x,5,9699690), 3, 4, 1),
        Monkey::new(vec![88, 74, 68, 56], |x| sq(x, 9699690), 17, 0, 2),
        Monkey::new(vec![54, 82, 72, 71, 53, 99, 67], |x| add(x,1,9699690), 19, 6, 0),
    ];

    let mut monkeys2 = input2;
    play_rounds(&mut monkeys2, 10000, false);
    println!("Part 2 : Score - {}", calc_score(&monkeys2));
}