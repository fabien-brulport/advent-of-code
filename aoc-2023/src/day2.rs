use std::cmp;
use std::fs;

#[derive(PartialEq, Debug)]
struct Bag {
    r: u32,
    g: u32,
    b: u32,
}
// impl PartialOrd for Bag {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.r.cmp(&other.r) && self.g.cmp(&other.g) && self.b.cmp(&other.b))
//     }
// }
impl Bag {
    pub fn from_str(game: &str) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        game.split(", ").for_each(|x| {
            let parts = x.split(" ").collect::<Vec<&str>>();
            let number = parts[0].parse::<u32>().unwrap();
            match parts[1] {
                "red" => r = number,
                "green" => g = number,
                "blue" => b = number,
                _ => unreachable!(),
            }
        });
        Self { r, g, b }
    }
}

fn part1(input: &str) -> u32 {
    let bag = Bag {
        r: 12,
        g: 13,
        b: 14,
    };
    input
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            // game_number is also the line index + 1
            let game_number = parts[0].split(" ").last().unwrap().parse::<u32>().unwrap();
            let mut condition = true;
            parts[1].split("; ").for_each(|game| {
                let b = Bag::from_str(game);
                if b.r > bag.r || b.g > bag.g || b.b > bag.b {
                    condition = false;
                }
            });
            if condition {
                game_number
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let mut bag = Bag { r: 0, g: 0, b: 0 };
            parts[1].split("; ").for_each(|game| {
                let b = Bag::from_str(game);
                bag.r = cmp::max(bag.r, b.r);
                bag.g = cmp::max(bag.g, b.g);
                bag.b = cmp::max(bag.b, b.b);
            });
            bag.r * bag.g * bag.b
        })
        .sum()
}
fn main() {
    // part 1
    let contents = fs::read_to_string("input/day_2.txt").unwrap();
    dbg!(part1(&contents));
    // part 1
    let contents = fs::read_to_string("input/day_2.txt").unwrap();
    dbg!(part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let content = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(&content), 8);
    }

    #[test]
    fn test_part2() {
        let content = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(&content), 2286);
    }
}
