use std::collections::HashMap;

type Game = (i32, i32);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    // Match the letter to an integer
    let letter_mapping =
        HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    input
        .lines()
        .map(|l| {
            let mut game = l.split_whitespace();
            let other = game.next().unwrap();
            let me = game.next().unwrap();
            (
                *letter_mapping.get(other).unwrap(),
                *letter_mapping.get(me).unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> i32 {
    // This is the outcome score: 6 - 3 * (game.0 - game.1 - 2).rem_euclid(3))
    // This is the shape score: game.1
    input
        .iter()
        .map(|game| 6 - 3 * (game.0 - game.1 - 2).rem_euclid(3) + game.1)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> i32 {
    // This is the outcome score: (game.1 - 1) * 3
    // This is the shape score: (game.0 + game.1 - 3).rem_euclid(3) + 1
    input
        .iter()
        .map(|game| (game.0 + game.1 - 3).rem_euclid(3) + 1 + (game.1 - 1) * 3)
        .sum()
}
