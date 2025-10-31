mod heartbreak;
mod read_input;
use heartbreak::*;

fn main() {
    println!("advent of code\n");

    let solutions = [
        day1_part1, day1_part2, day2_part1, day2_part2, day3_part1, day3_part2, day4_part1,
        day4_part2, day5_part1,
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
