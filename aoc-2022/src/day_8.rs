use std::collections::HashMap;

type Map = HashMap<(usize, usize), usize>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    let mut map: Map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), c.to_digit(10).unwrap().try_into().unwrap());
        }
    }
    map
}

fn get_neighboors(
    index: (usize, usize),
    map: &Map,
    x_max: usize,
    y_max: usize,
) -> Vec<Vec<&usize>> {
    // Return the neighboors value in the 4 directions, each time in a separated Vec
    let mut neighboors: Vec<Vec<&usize>> = vec![Vec::<&usize>::new(); 4];
    // Reverse the iter because it is needed for part2:
    for x in (0..index.0).rev() {
        neighboors[0].push(map.get(&(x, index.1)).unwrap());
    }
    for x in index.0 + 1..x_max {
        neighboors[1].push(map.get(&(x, index.1)).unwrap());
    }
    for y in (0..index.1).rev() {
        neighboors[2].push(map.get(&(index.0, y)).unwrap());
    }
    for y in index.1 + 1..y_max {
        neighboors[3].push(map.get(&(index.0, y)).unwrap());
    }
    neighboors
}

fn get_dimensions(map: &Map) -> (usize, usize) {
    let x_max = map.keys().max_by_key(|coord| coord.0).unwrap().0;
    let y_max = map.keys().max_by_key(|coord| coord.1).unwrap().1;
    (x_max + 1, y_max + 1)
}

#[aoc(day8, part1)]
pub fn solve_part1(map: &Map) -> usize {
    let (x_max, y_max) = get_dimensions(&map);

    // Directly add the borders
    let mut count = 2 * (x_max + y_max - 2);
    for x in 1..x_max - 1 {
        for y in 1..y_max - 1 {
            let current_value = map.get(&(x, y)).unwrap();
            let neighboors = get_neighboors((x, y), &map, x_max, y_max);
            if neighboors
                .iter()
                .any(|nn| nn.iter().all(|&n| n < current_value))
            {
                count += 1
            }
        }
    }

    count
}

#[aoc(day8, part2)]
pub fn solve_part2(map: &Map) -> usize {
    let (x_max, y_max) = get_dimensions(&map);

    let mut scores: Vec<usize> = vec![];
    for x in 1..x_max - 1 {
        for y in 1..y_max - 1 {
            let current_value = map.get(&(x, y)).unwrap();
            let neighboors = get_neighboors((x, y), &map, x_max, y_max);
            let tree_score: usize = neighboors
                .iter()
                .map(|n| {
                    let mut x = 0;
                    for elem in n {
                        x += 1;
                        if *elem >= current_value {
                            break;
                        }
                    }
                    x
                })
                .product();
            scores.push(tree_score)
        }
    }

    *scores.iter().max().unwrap()
}
