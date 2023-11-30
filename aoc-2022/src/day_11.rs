use std::{collections::VecDeque, str::Lines};

#[derive(Debug, Clone)]
pub struct Monkey {
    id: u32,
    operation: Operation,
    items: VecDeque<u32>,
    divisor: u32,
    true_index: u32,
    false_index: u32,
}

fn parse_item(line: &str) -> VecDeque<u32> {
    line.split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
enum Operation {
    ADD(Option<u32>),
    SUB(Option<u32>),
    MUL(Option<u32>),
    DIV(Option<u32>),
}

impl Operation {
    fn from_line(line: &str) -> Result<Self, &str> {
        let vec: Vec<&str> = line
            .split(" = ")
            .skip(1)
            .next()
            .ok_or("Unable to parse line")?
            .split_whitespace()
            .collect();
        let value = vec.get(2).ok_or("Not enougth value.")?.parse().ok();
        match *vec.get(1).ok_or("Not enougth values.")? {
            "+" => Ok(Self::ADD(value)),
            "-" => Ok(Self::SUB(value)),
            "*" => Ok(Self::MUL(value)),
            "/" => Ok(Self::DIV(value)),
            _ => unreachable!(),
        }
    }

    fn exec(&self, old: u32) -> u32 {
        match self {
            Operation::ADD(num) => {
                old + match num {
                    Some(num) => num,
                    None => &old,
                }
            }
            Operation::SUB(num) => {
                old - match num {
                    Some(num) => num,
                    None => &old,
                }
            }
            Operation::MUL(num) => {
                old * match num {
                    Some(num) => num,
                    None => &old,
                }
            }
            Operation::DIV(num) => {
                old / match num {
                    Some(num) => num,
                    None => &old,
                }
            }
        }
    }
}

fn parse_last_word_to_int(lines: &mut Lines) -> u32 {
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> VecDeque<Monkey> {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    let paragraph: Vec<&str> = input.split("\n\n").collect();
    let mut res = VecDeque::new();
    for (monkey_id, p) in paragraph.into_iter().enumerate() {
        let mut lines = p.lines();
        lines.next();
        let items: VecDeque<u32> = parse_item(lines.next().unwrap());
        let operation = Operation::from_line(lines.next().unwrap()).unwrap();
        let divisor = parse_last_word_to_int(&mut lines);
        let true_index = parse_last_word_to_int(&mut lines);
        let false_index = parse_last_word_to_int(&mut lines);
        res.push_back(Monkey {
            id: monkey_id as u32,
            operation,
            items,
            divisor,
            true_index,
            false_index,
        });
    }
    res
}

fn do_one_round(monkeys: &mut VecDeque<Monkey>, count: &mut Vec<u32>) {
    let n_monkeys = monkeys.len() as u32;
    for i in 0..n_monkeys {
        let mut monkey = monkeys.pop_front().unwrap();
        for _ in 0..monkey.items.len() {
            let c = count.get_mut(i as usize).unwrap();
            *c = *c + 1;
            let item = monkey.items.pop_front().unwrap();
            let mut new = monkey.operation.exec(item);
            // new = new.div_euclid(3);
            let new_monkey: u32 = match new.checked_rem(monkey.divisor) == Some(0) {
                true => monkey.true_index,
                false => monkey.false_index,
            };
            let new_monkey = match new_monkey.checked_sub(i + 1) {
                Some(x) => x,
                None => n_monkeys - ((i + 1).checked_sub(new_monkey).unwrap()),
            };
            monkeys
                .get_mut(new_monkey as usize)
                .unwrap()
                .items
                .push_back(new);
        }
        monkeys.push_back(monkey);
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(monkeys: &VecDeque<Monkey>) -> u32 {
    let mut c = monkeys.clone();
    let mut count = vec![0; c.len()];
    for _ in 0..20 {
        do_one_round(&mut c, &mut count);
    }
    dbg!(&count);
    count.sort();
    count.get(count.len() - 1).unwrap() * count.get(count.len() - 2).unwrap()
}
