use std::collections::HashMap;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (HashMap<usize, Vec<char>>, Vec<Vec<usize>>) {
    let paragraph: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = HashMap::new();

    // Reverse the lines and skip the number, so that the
    // stacks are in the correct order
    for line in paragraph[0].lines().rev().skip(1) {
        for i in 0..9 {
            let char_idx = i * 4 + 1;
            // Detect if there is a char
            let c = line.chars().nth(char_idx).unwrap();
            if c != ' ' {
                let vec = stacks.entry(i + 1).or_insert(vec![]);
                vec.push(c);
            }
        }
    }
    // Parse instructions
    let instructions: Vec<Vec<usize>> = paragraph[1]
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect();
    (stacks, instructions)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(HashMap<usize, Vec<char>>, Vec<Vec<usize>>)) -> String {
    let (stack, instructions) = input;
    // Need to clone the HashMap to modify it
    let mut stack_cloned = stack.clone();
    for instruction in instructions {
        let number = instruction[0];
        let from = instruction[1];
        let to = instruction[2];

        // Transfer the elements n times, by always taking tha last element
        for _ in 0..number {
            let from_vec = stack_cloned.get_mut(&from).unwrap();
            let elem = from_vec.pop().unwrap();
            let to_vec = stack_cloned.get_mut(&to).unwrap();
            to_vec.push(elem);
        }
    }
    (1..10)
        .filter_map(|i| stack_cloned.get_mut(&i).unwrap().pop())
        .collect::<String>()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &(HashMap<usize, Vec<char>>, Vec<Vec<usize>>)) -> String {
    let (stack, instructions) = input;
    // Need to clone the HashMap to modify it
    let mut stack_cloned = stack.clone();
    for instruction in instructions {
        let number = instruction[0];
        let from = instruction[1];
        let to = instruction[2];
        let idx = stack_cloned.get(&from).unwrap().len() - number;

        // Transfer the elements n times, by always takng the same index
        for _ in 0..number {
            let from_vec = stack_cloned.get_mut(&from).unwrap();
            let elem = from_vec.remove(idx);
            let to_vec = stack_cloned.get_mut(&to).unwrap();
            to_vec.push(elem);
        }
    }
    (1..10)
        .filter_map(|i| stack_cloned.get_mut(&i).unwrap().pop())
        .collect::<String>()
}
