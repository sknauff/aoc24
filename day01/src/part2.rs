use crate::seperate_lists_left_right;
use rayon::prelude::*;

pub fn solve(input: &str) -> u64 {
    let (list_left, list_right) = seperate_lists_left_right(input);
    let sim: Vec<u64> = list_left
        .par_iter()
        .map(|value| {
            value
                * list_right
                    .par_iter()
                    .filter(|&x| x == value)
                    .collect::<Vec<&u64>>()
                    .len() as u64
        })
        .collect();
    sim.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_example() {
        let result = solve(
            "
                3   4
                4   3
                2   5
                1   3
                3   9
                3   3
                "
            .trim(),
        );
        assert_eq!(result, 31);
    }
}
