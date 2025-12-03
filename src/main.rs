use std::fs;

use adventofcode as lib;

fn main() {
    println!("🎄 Advent of Code 2025 🎄\n");

    // Day 1
    println!("Day 1: Secret Entrance");
    let day01_input = fs::read_to_string("src/lib/day01/input.txt").expect("Failed to read day01 input file");

    let part1_answer = lib::day01::day01_part1::solve(&day01_input);
    println!("Part 1 Answer: {}", part1_answer);

    let part2_answer = lib::day01::day01_part2::solve(&day01_input);
    println!("Part 2 Answer: {}", part2_answer);

    println!("\n---\n");

    // Day 2
    println!("Day 2: Gift Shop");
    let day02_input = fs::read_to_string("src/lib/day02/input.txt").expect("Failed to read day02 input file");

    let day02_part1 = lib::day02::day02_part1::solve(&day02_input);
    println!("Part 1 Answer: {}", day02_part1);

    let day02_part2 = lib::day02::day02_part2::solve(&day02_input);
    println!("Part 2 Answer: {}", day02_part2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
