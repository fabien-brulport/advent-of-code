use std::collections::BinaryHeap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    // Create a BinaryHeap to esily get the top 3 values.
    let mut heap = BinaryHeap::new();
    for element in input {
        heap.push(*element);
    }

    // Get top 3 and sum the values.
    let mut top_3 = Vec::new();
    for _ in 0..3 {
        top_3.push(heap.pop().unwrap())
    }
    top_3.iter().sum()
}
