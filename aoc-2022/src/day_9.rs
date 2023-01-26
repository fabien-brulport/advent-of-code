use std::cmp;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Direction {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Action {
    direction: Direction,
    repetition: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Position {
    x: isize,
    y: isize,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|l| {
            let mut splitted_line = l.split_whitespace();
            let direction = splitted_line.next().unwrap();
            let repetition = splitted_line.next().unwrap();
            Action {
                direction: match direction {
                    "L" => Direction { x: -1, y: 0 },
                    "R" => Direction { x: 1, y: 0 },
                    "U" => Direction { x: 0, y: 1 },
                    "D" => Direction { x: 0, y: -1 },
                    _ => unreachable!(),
                },
                repetition: repetition.parse().unwrap(),
            }
        })
        .collect()
}

fn compute_new_position(head: &Position, tail: &Position) -> Position {
    let direction = Direction {
        x: (head.x - tail.x),
        y: (head.y - tail.y),
    };
    // If the head and tail are close enough, do nothing
    if cmp::max(direction.x.abs(), direction.y.abs()) <= 1 {
        return tail.clone();
    }
    // Move by 1 in the correct direction
    Position {
        x: tail.x + cmp::max(cmp::min(direction.x, 1), -1),
        y: tail.y + cmp::max(cmp::min(direction.y, 1), -1),
    }
}

fn update_head(head: &mut Position, direction: &Direction) {
    head.x += direction.x;
    head.y += direction.y;
}

#[aoc(day9, part1)]
pub fn solve_part1(actions: &Vec<Action>) -> usize {
    let mut tail = Position { x: 0, y: 0 };
    let mut head = Position { x: 0, y: 0 };
    let mut unique_position = HashSet::new();
    unique_position.insert(tail.clone());
    for action in actions {
        for _ in 0..action.repetition {
            update_head(&mut head, &action.direction);
            tail = compute_new_position(&head, &tail);
            unique_position.insert(tail.clone());
        }
    }
    unique_position.len()
}

#[aoc(day9, part2)]
pub fn solve_part2(actions: &Vec<Action>) -> usize {
    let n_elements = 10;
    let mut positions = vec![Position { x: 0, y: 0 }; n_elements];
    let mut unique_position = HashSet::new();
    unique_position.insert(positions[n_elements - 1].clone());
    for action in actions {
        for _ in 0..action.repetition {
            update_head(&mut positions[0], &action.direction);
            for i in 1..n_elements {
                positions[i] = compute_new_position(&positions[i - 1], &positions[i]);
            }
            unique_position.insert(positions[n_elements - 1].clone());
        }
    }
    unique_position.len()
}
