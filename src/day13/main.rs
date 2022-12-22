use std::{fs, cmp::Ordering};

#[derive(PartialEq)]
enum Order {
    Right,
    Wrong
}

struct Packet {
    idx: usize,
    source: String,
    indent: usize,
}

impl Packet {
    fn new(source: &str) -> Self {
        Packet { idx: 0, indent:0, source: source.to_string() }
    }

    fn from_digit(digit: usize) -> String {
       format!("[{}]", digit).to_string()
    }

    fn get_digit(&self) -> Option<usize> {
        if self.source.contains("[") {
            return None
        }
        return Some(self.source.parse::<usize>().unwrap());
    }
}

impl Iterator for Packet {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        self.indent = 0;
        let part = self.source[1..self.source.len() - 1]
            .chars()
            .skip(self.idx)
            .map(|c| {
                if c == '[' {
                    self.indent += 1
                } else if c == ']' {
                    self.indent -= 1
                }
                (self.indent, c)
            })
            .take_while(|(indent, c)| (*indent != 0 || *c != ','))
            .map(|(_, c)| c)
            .collect::<String>();

        if part.is_empty() {
            return None
        }
        self.idx += part.len() + 1;
        Some(Packet::new(&part))
    }
}

fn order(l_str: &str, r_str:&str) -> Option<Order> {
    let mut left = Packet::new(l_str);
    let mut right: Packet = Packet::new(r_str);

    loop {
        let l = left.next();
        let r = right.next();

        match (l, r) {
            (None, None) => return None,
            (None, Some(_)) => return Some(Order::Right),
            (Some(_), None) => return Some(Order::Wrong),
            (Some(lhs), Some(rhs)) => {
                let (left_val, right_val):(Option<usize>, Option<usize>) = (lhs.get_digit(), rhs.get_digit());
                if left_val.is_some() && right_val.is_some() {
                    let lv = left_val.unwrap();
                    let rv = right_val.unwrap();
                    if lv == rv {
                        continue
                    }
                    if lv < rv {
                        return Some(Order::Right)
                    } else {
                        return Some(Order::Wrong)
                    }
                }

                if left_val.is_none() && right_val.is_none() {
                    let sub = order(&lhs.source, &rhs.source);
                    if sub.is_none() {
                        continue;
                    } else {
                        return sub
                    }
                }

                if left_val.is_some() {
                    let sub = order(&Packet::from_digit(left_val.unwrap()), &rhs.source);
                    if sub.is_none() {
                        continue;
                    } else {
                        return sub
                    }
                }

                if right_val.is_some() {
                    let sub = order(&lhs.source,  &Packet::from_digit(right_val.unwrap()),);
                    if sub.is_none() {
                        continue;
                    } else {
                        return sub
                    }
                }
            }
        }
    }
}


fn main() {
    let input_data = fs::read_to_string("src/day13/input.txt").unwrap();

    let result1 = input_data.split("\n\n")
        .map(|pair_str| pair_str.split_once("\n"))
        .map(|result| result.unwrap())
        .enumerate()
        .map(|(e, (left_str, right_str))| ( e+1, (left_str, right_str)))
        .map(|(idx, (left, right))| (idx, order(left,right).unwrap()))
        .filter(|(_, res)| *res == Order::Right)
        .map(|(id, _)| id)
        .sum::<usize>();

    println!("Part 1 - Result: {}", result1);

    let mut signals = input_data.lines()
        .filter(|line| !line.is_empty())
        .chain(vec!["[[2]]", "[[6]]"])
        .collect::<Vec<&str>>();

    signals.sort_by(|a, b| {
        let o = order(*a,*b).unwrap();
        match o {
            Order::Right => Ordering::Less,
            Order::Wrong => Ordering::Greater,
        }
    });

    let result2 = signals.iter()
        .enumerate()
        .map(|(i, line)| (i+1, line))
        .filter(|(_, line) | {
            *line == &"[[2]]" || *line == &"[[6]]"
        })
        .map(|(i, _)| i)
        .product::<usize>();

    println!("Part 2 - Result: {}", result2);

}