use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    solve_pbm(input, 4)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    solve_pbm(input, 14)
}

fn solve_pbm(input: &str, buffer_size: usize) -> usize {
    let mut vec: VecDeque<char> = VecDeque::new();

    for (i, c) in input.char_indices() {
        vec.push_back(c);
        if vec.len() == buffer_size {
            if HashSet::<char>::from_iter(vec.clone()).len() == buffer_size {
                return i + 1;
            }
            vec.pop_front();
        }
    }
    unreachable!()
}
