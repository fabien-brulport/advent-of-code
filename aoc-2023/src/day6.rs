use std::fs;

fn distance_from_time(time: usize, duration: usize) -> usize {
    time * duration.saturating_sub(time)
}

fn part1(input: &str) -> usize {
    let times = input
        .lines()
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim_start()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let distances = input
        .lines()
        .last()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim_start()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    times
        .zip(distances)
        .map(|(duration, dist)| {
            (0..=duration)
                .map(|x| (distance_from_time(x, duration) > dist) as usize)
                .sum::<usize>()
        })
        .product::<usize>()
}

fn part2(input: &str) -> usize {
    let time = input
        .lines()
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim_start()
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();
    let distances = input
        .lines()
        .last()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim_start()
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    (0..=time)
        .map(|x| (distance_from_time(x, time) > distances) as usize)
        .sum::<usize>()
}
fn main() {
    // part 1
    let contents = fs::read_to_string("input/day_6.txt").unwrap();
    dbg!(part1(&contents));
    // part 1
    let contents = fs::read_to_string("input/day_6.txt").unwrap();
    dbg!(part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let content = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(&content), 288);
    }
    #[test]
    fn test_part2() {
        let content = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part2(&content), 71503);
    }
}
