use day03::*;
use shared_lib::print_solutions;

fn main() {
    let input = include_str!("./input.txt");
    print_solutions(solve_pt1(input), solve_pt2(input));
}
