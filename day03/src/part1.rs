use crate::*;

pub fn solve(input: &str) -> u64 {
    multiply_some_numbers(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_example() {
        let result = solve(
            "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
                "
            .trim(),
        );
        assert_eq!(result, 161);
    }
}
