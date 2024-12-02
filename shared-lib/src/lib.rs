fn print_solution<T: std::fmt::Display>(idx: u8, solution: T) {
    println!(
        "{}",
        format!(
            "
    Solution Part {idx}:
        {solution}
    "
        )
    );
}

pub fn print_solutions<T: std::fmt::Display, U: std::fmt::Display>(part1: T, part2: U) {
    print_solution(1, part1);
    println!("\n------\n");
    print_solution(2, part2);
}
