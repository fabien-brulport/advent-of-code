use std::collections::HashSet;
use std::fs;

fn get_match(line: &str) -> u32 {
    let parts = line
        .split(": ")
        .last()
        .unwrap()
        .split(" | ")
        .collect::<Vec<&str>>();
    let winning: HashSet<&str> = HashSet::from_iter(parts[0].split_whitespace());
    let mine: HashSet<&str> = HashSet::from_iter(parts[1].split_whitespace());
    winning.intersection(&mine).count() as u32
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match get_match(line) {
            0 => 0,
            x => 2u32.pow(x as u32 - 1),
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    // Initialize with one card everywhere
    let mut cards = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let n_current = cards[i];
        let m = get_match(line);
        for j in 0..m {
            cards[i + j as usize + 1] += n_current
        }
    }
    cards.iter().sum()
}
fn main() {
    // part 1
    let contents = fs::read_to_string("input/day_4.txt").unwrap();
    dbg!(part1(&contents));
    // part 1
    let contents = fs::read_to_string("input/day_4.txt").unwrap();
    dbg!(part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(&content), 13);
    }
    #[test]
    fn test_part2() {
        let content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(&content), 30);
    }
}
