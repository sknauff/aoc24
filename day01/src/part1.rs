use crate::seperate_lists_left_right;

pub fn solve(input: &str) -> u64 {
    let (mut list_left, mut list_right) = seperate_lists_left_right(input);
    list_left.sort();
    list_right.sort();
    let mut diff: Vec<u64> = Vec::new();
    for i in 0..list_left.len() {
        diff.push((list_left[i] as i64 - list_right[i] as i64).abs() as u64)
    }
    diff.iter().sum()
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
        assert_eq!(result, 11);
    }
}
