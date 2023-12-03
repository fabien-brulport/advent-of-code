use std::fs;

fn compute_line(line: &Vec<char>) -> u32 {
    format!("{}{}", line[0], line.last().unwrap())
        .parse::<u32>()
        .unwrap()
}

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut s = 0;
    for line in lines {
        let numbers = line
            .chars()
            .filter(|x| x.is_numeric())
            .collect::<Vec<char>>();
        s = s + compute_line(&numbers);
    }
    s
}
fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut s = 0;
    for line in lines {
        let numbers = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "th3ee")
            .replace("four", "f4ur")
            .replace("five", "fi5e")
            .replace("six", "s6x")
            .replace("seven", "s7ven")
            .replace("eight", "e8ght")
            .replace("nine", "n9ne")
            .chars()
            .filter(|x| x.is_numeric())
            .collect::<Vec<char>>();

        s = s + compute_line(&numbers);
    }
    s
}
fn main() {
    // part 1
    let contents = fs::read_to_string("input/day_1.txt").unwrap();
    dbg!(part1(&contents));
    // part 2
    let contents = fs::read_to_string("input/day_1.txt").unwrap();
    dbg!(part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let content = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(&content), 142);
    }

    #[test]
    fn test_part2() {
        let content = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(&content), 281);
    }
}
