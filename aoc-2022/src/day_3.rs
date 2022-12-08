use std::collections::HashSet;

type Rucksack = (HashSet<char>, HashSet<char>);

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|l| {
            let length = l.len();
            let elem = l.split_at(length / 2);
            (
                HashSet::from_iter(elem.0.chars()),
                HashSet::from_iter(elem.1.chars()),
            )
        })
        .collect()
}

fn char_to_value(c: &char) -> i32 {
    // Use a conversion to ascii to easily assign the correct value
    // https://www.rapidtables.com/code/text/ascii-table.html
    let mut value = *c as i32;
    if value > 90 {
        // lower case
        value -= 96;
    } else {
        // upper case
        value -= 38;
    }
    value
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Rucksack]) -> i32 {
    input
        .iter()
        .map(|r| r.0.intersection(&r.1).next().unwrap())
        .map(char_to_value)
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Rucksack]) -> i32 {
    let n_groups = input.len() / 3;
    (0..n_groups)
        .map(|i| {
            // Create 3 sets from the 6 total sets (union)
            let mut sets: Vec<HashSet<&char>> = vec![];
            for j in 0..3 {
                let elf_idx = i * 3 + j;
                sets.push(HashSet::from_iter(
                    input[elf_idx].0.union(&input[elf_idx].1),
                ));
            }

            // Intersection over multiple sets
            let mut intersection = sets.pop().unwrap();
            intersection.retain(|item| sets.iter().all(|set| set.contains(item)));

            // Get the character
            let result = intersection.drain().next().unwrap();
            result
        })
        .map(char_to_value)
        .sum::<i32>()
}
