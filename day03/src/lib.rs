mod part1;
mod part2;

pub use part1::solve as solve_pt1;
pub use part2::solve as solve_pt2;

use regex::Regex;

fn multiply_some_numbers(input: &str) -> u64 {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|m| (m.name("a").unwrap().as_str(), m.name("b").unwrap().as_str()))
        .map(|(a, b)| {
            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();
            a * b
        })
        .sum()
}