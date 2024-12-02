mod part1;
mod part2;

pub use part1::solve as solve_pt1;
pub use part2::solve as solve_pt2;

use rayon::prelude::*;

#[derive(PartialEq, Debug)]
enum ReactorState {
    Safe,
    Unsafe,
}

#[derive(Debug)]
struct Reactor {
    levels: Vec<i64>,
}

#[derive(Debug)]
enum Direction {
    ASC,
    DESC,
}

impl Reactor {
    fn from(input: &str) -> Self {
        Reactor {
            levels: input
                .split_whitespace()
                .map(|element| element.parse().unwrap())
                .collect(),
        }
    }

    fn test_vec_reactor_state(&self, vec_to_test: Vec<i64>) -> ReactorState {
        let expected_safe_intersections = vec_to_test.len() - 1;

        let dir = match vec_to_test.first() < vec_to_test.last() {
            true => Direction::ASC,
            false => Direction::DESC,
        };

        let found_safe_intersections = (0..vec_to_test.len() - 1)
            .into_par_iter()
            .map(|n| vec_to_test[n] - vec_to_test[n + 1])
            .filter(|&n| {
                let abs_val = n.abs();
                abs_val >= 1 && abs_val <= 3
            })
            .filter(|&elem| match dir {
                Direction::ASC => elem < 0,
                Direction::DESC => elem > 0,
            })
            .count();

        match expected_safe_intersections == found_safe_intersections {
            true => ReactorState::Safe,
            false => ReactorState::Unsafe,
        }
    }

    fn is_safe(&self, with_damper: bool) -> ReactorState {
        if self.test_vec_reactor_state(self.levels.clone()) == ReactorState::Safe {
            return ReactorState::Safe;
        } else if with_damper == false {
            return ReactorState::Unsafe;
        }

        let works_with_one_damper = (0..self.levels.len())
            .into_par_iter()
            .map(|n| {
                let mut damper_test = self.levels.clone();
                damper_test.remove(n);
                self.test_vec_reactor_state(damper_test)
            })
            .any(|x| x == ReactorState::Safe);

        if works_with_one_damper {
            return ReactorState::Safe;
        }
        ReactorState::Unsafe
    }

    fn get_state_no_damper(&self) -> ReactorState {
        self.is_safe(false)
    }

    fn get_state_with_damper(&self) -> ReactorState {
        self.is_safe(true)
    }
}
