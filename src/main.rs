mod bleeding;
mod heartbreak;
mod read_input;

fn main() {
    println!("advent of code\n");

    let solutions = [
        bleeding::day1_part1,
        bleeding::day1_part2,
        bleeding::day2_part1,
        bleeding::day2_part2,
        bleeding::day3_part1,
        bleeding::day3_part2,
        bleeding::day4_part1,
        bleeding::day4_part2,
        bleeding::day5_part1,
        bleeding::day5_part2,
        bleeding::day6_part1,
        bleeding::day6_part2,
        bleeding::day7_part1,
        bleeding::day7_part2,
        bleeding::day8_part1,
        bleeding::day8_part2,
        bleeding::day9_part1,
        bleeding::day9_part2,
    ];

    for (day, sol) in solutions.iter().enumerate() {
        println!(
            "running solution for day {} part {}",
            day / 2 + 1,
            day % 2 + 1
        );
        let _ = sol();
        println!();
    }
}
