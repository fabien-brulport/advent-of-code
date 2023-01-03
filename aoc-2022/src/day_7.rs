use std::collections::HashMap;

struct File {
    size: usize,
}

pub struct Directory {
    dirs: Vec<String>,
    files: Vec<File>,
}

impl Directory {
    fn new() -> Self {
        Self {
            dirs: Vec::<String>::new(),
            files: Vec::<File>::new(),
        }
    }
}

fn compute_size(
    name: String,
    tree: &HashMap<String, Directory>,
    size: &mut HashMap<String, usize>,
) -> usize {
    // Recursive function to compute the size of each directory
    // and store them in a HashMap
    let elem = tree.get(&name).unwrap();
    let file_size = elem.files.iter().map(|f| f.size).sum::<usize>();
    let dir_size = elem
        .dirs
        .iter()
        .map(|d| compute_size(d.clone(), tree, size))
        .sum::<usize>();
    let total_size = file_size + dir_size;
    size.insert(name, total_size);
    total_size
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, usize> {
    let mut tree: HashMap<String, Directory> = HashMap::new();
    let mut current_dir = vec![];
    for line in input.lines() {
        let vec: Vec<&str> = line.split_whitespace().collect();
        if vec.len() == 3 {
            let directory = vec[2];
            if directory == ".." {
                current_dir.pop();
            } else {
                current_dir.push(directory);
            }
            let current_dir_str = current_dir.join("-").to_string();
            if !tree.contains_key(&current_dir_str) {
                tree.insert(current_dir_str.clone(), Directory::new());
            }
        } else {
            if vec[0] == "$" {
                continue;
            }
            let current_dir_str = current_dir.join("-").to_string();
            let current_node = tree.get_mut(&current_dir_str).unwrap();
            if vec[0] == "dir" {
                current_node
                    .dirs
                    .push(current_dir_str + "-" + &vec[1].to_string());
            } else {
                current_node.files.push(File {
                    size: vec[0].parse().unwrap(),
                });
            }
        }
    }
    let mut size: HashMap<String, usize> = HashMap::new();
    compute_size("/".to_string(), &tree, &mut size);
    size
}

#[aoc(day7, part1)]
pub fn solve_part1(size: &HashMap<String, usize>) -> usize {
    size.iter()
        .filter(|(_, &v)| v < 100000_usize)
        .map(|(_, &v)| v)
        .sum::<usize>()
}

#[aoc(day7, part2)]
pub fn solve_part2(size: &HashMap<String, usize>) -> usize {
    let total_size_occupied = size.get(&"/".to_string()).unwrap();
    let limit = 30000000 + total_size_occupied - 70000000;

    size.iter()
        .filter(|(_, &v)| v > limit)
        .map(|(_, &v)| v)
        .min()
        .unwrap()
}
