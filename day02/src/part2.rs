use crate::{Reactor, ReactorState};
use rayon::prelude::*;

pub fn solve(input: &str) -> u64 {
    let reactors: Vec<Reactor> = input.lines().map(|line| Reactor::from(line)).collect();
    reactors
        .par_iter()
        .filter(|&e| e.get_state_with_damper() == ReactorState::Safe)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_example() {
        let result = solve(
            "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
            "
            .trim(),
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn r0() {
        let result = Reactor::from(
            "
7 6 4 2 1
            "
            .trim(),
        )
        .get_state_with_damper();
        assert_eq!(result, ReactorState::Safe);
    }

    #[test]
    fn r1() {
        let result = Reactor::from(
            "
1 2 7 8 9
            "
            .trim(),
        )
        .get_state_with_damper();
        assert_eq!(result, ReactorState::Unsafe);
    }

    #[test]
    fn r2() {
        let result = Reactor::from(
            "
9 7 6 2 1
            "
            .trim(),
        )
        .get_state_with_damper();
        assert_eq!(result, ReactorState::Unsafe);
    }

    #[test]
    fn r3() {
        let result = Reactor::from(
            "
1 3 2 4 5
            "
            .trim(),
        )
        .get_state_with_damper();
        assert_eq!(result, ReactorState::Safe);
    }

    #[test]
    fn r4() {
        let result = Reactor::from(
            "
8 6 4 4 1
            "
            .trim(),
        )
        .get_state_with_damper();
        assert_eq!(result, ReactorState::Safe);
    }

    #[test]
    fn r5() {
        let result = Reactor::from(
            "
1 3 6 7 9
            "
            .trim(),
        )
        .get_state_with_damper();
        assert_eq!(result, ReactorState::Safe);
    }
}
