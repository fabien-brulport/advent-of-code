#[derive(Debug)]
pub struct Action {
    n_cycle: usize,
    value: isize,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|l| {
            let mut splitted_line = l.split_whitespace();
            let action = splitted_line.next().unwrap();
            let value = splitted_line.next().unwrap_or("0");
            Action {
                n_cycle: match action {
                    "noop" => 1,
                    "addx" => 2,
                    _ => unreachable!(),
                },
                value: value.parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(actions: &Vec<Action>) -> isize {
    let mut register = 1;
    let mut timer = 0;
    let mut result: Vec<isize> = vec![];
    for action in actions {
        for _ in 0..action.n_cycle {
            timer += 1;
            if (timer + 20) % 40 == 0 {
                result.push((register * timer).clone())
            }
        }
        register += action.value;
    }
    result.iter().sum()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}

#[aoc(day10, part2)]
pub fn solve_part2(actions: &Vec<Action>) -> String {
    let n_rows = 6;
    let n_cols = 40;
    let mut register = 1;
    let mut timer = 0;
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; n_cols]; n_rows];
    let mut row = 0;
    for action in actions {
        for _ in 0..action.n_cycle {
            if (register - ((timer as isize) % n_cols as isize)).abs() < 2 {
                grid[row][timer % n_cols] = '#'
            }
            timer += 1;
            if timer % n_cols == 0 {
                row += 1;
            }
        }
        register += action.value;
    }
    print_grid(&grid);
    "Result printed above".to_string()
}
