type Pair = ((i32, i32), (i32, i32));

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|l| {
            // Parse everything into a Vec of Vec, then get the correct indexes
            let vec: Vec<Vec<i32>> = l
                .split(",")
                .map(|el| el.split("-").map(|c| c.parse::<i32>().unwrap()).collect())
                .collect();
            ((vec[0][0], vec[0][1]), (vec[1][0], vec[1][1]))
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Pair]) -> i32 {
    input
        .iter()
        .filter(|p| {
            ((p.0 .0 <= p.1 .0) && (p.0 .1 >= p.1 .1)) || ((p.0 .0 >= p.1 .0) && (p.0 .1 <= p.1 .1))
        })
        .count() as i32
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Pair]) -> i32 {
    input
        .iter()
        .filter(|p| (p.0 .1 >= p.1 .0) && (p.0 .0 <= p.1 .1))
        .count() as i32
}
