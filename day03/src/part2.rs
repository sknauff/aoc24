use crate::*;

pub fn solve(input: &str) -> u64 {
    let do_group: Vec<&str> = input
        .split("do()")
        .into_iter()
        .map(|sub_str| {
            let sub_sub: Vec<&str> = sub_str.split("don't()").collect();
            sub_sub.first().unwrap().to_owned()
        })
        .collect();
    multiply_some_numbers(&do_group.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_example() {
        let result = solve(
            "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
                "
            .trim(),
        );
        assert_eq!(result, 48);
    }
}
