mod lib;

use std::fs;

fn main() {
    println!("🎄 Advent of Code 2025 🎄\n");

    // Day 1
    println!("Day 1: Secret Entrance");
    let day01_input = fs::read_to_string("src/lib/day01/input.txt").expect("Failed to read day01 input file");

    let part1_answer = lib::day01::day01_part1::solve(&day01_input);
    println!("Part 1 Answer: {}", part1_answer);

    let part2_answer = lib::day01::day01_part2::solve(&day01_input);
    println!("Part 2 Answer: {}", part2_answer);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
