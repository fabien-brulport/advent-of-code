#[derive(Debug)]
pub struct Point {
    elevation: u8,
    coordinate: (usize, usize),
    start: bool,
    end: bool,
}

fn compute_elevation(character: char) -> u8 {
    (character as u8).saturating_sub(97)
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Point> {
    let output = input
        .split("\n")
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, character)| {
                Point {
                    elevation: match character {
                        'S' => compute_elevation('a'),
                        'E' => compute_elevation('z'),
                        c => compute_elevation(c),
                    },
                    coordinate: (x, y),
                    start: character == 'S',
                    end: character == 'E',
                }
            })
        })
        .collect();

    output
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Point]) -> u32 {
    // dbg!(input);
    0
}
