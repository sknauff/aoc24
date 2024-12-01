mod part1;
mod part2;

pub use part1::solve as solve_pt1;
pub use part2::solve as solve_pt2;

fn seperate_lists_left_right(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut list_left: Vec<u64> = Vec::new();
    let mut list_right: Vec<u64> = Vec::new();
    input.lines().into_iter().for_each(|line| {
        let line: Vec<u64> = line
            .split_whitespace()
            .into_iter()
            .map(|num| num.parse().unwrap())
            .collect();
        list_left.push(line[0]);
        list_right.push(line[1]);
    });
    (list_left, list_right)
}