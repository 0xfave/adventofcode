// Day 1 Part 2: Secret Entrance - Method 0x434C49434B
//
// Problem: Count EVERY time the dial passes through 0 during rotations,
// not just when it ends at 0.

use super::day01_part1::{parse_instruction, rotate_dial};

/// Count how many times the dial passes through 0 during a rotation
/// Parameters:
///   - start: starting position (0-99)
///   - direction: 'L' or 'R'
///   - distance: how many clicks to rotate
/// Returns: number of times 0 is crossed/passed
fn count_zero_crossings(start: i32, direction: char, distance: i32) -> i32 {
    let mut count = 0;

    // Count how many times we land on 0 during the rotation
    // We check each position from 1 to distance (not including starting position)

    match direction {
        'R' => {
            // For right rotation, count how many times we hit 0
            // We hit 0 when (start + steps) % 100 == 0
            for step in 1..=distance {
                if (start + step) % 100 == 0 {
                    count += 1;
                }
            }
        }
        'L' => {
            // For left rotation, count how many times we hit 0
            // We hit 0 when (start - steps) wraps to 0
            for step in 1..=distance {
                let pos = ((start - step) % 100 + 100) % 100;
                if pos == 0 {
                    count += 1;
                }
            }
        }
        _ => panic!("Invalid direction character"),
    }

    count
}

/// Main solution function for Part 2
pub fn solve(input: &str) -> i32 {
    let mut position = 50; // Starting position
    let mut zero_count = 0; // Count how many times we pass through 0

    for line in input.lines() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        let (direction, distance) = parse_instruction(line);
        zero_count += count_zero_crossings(position, direction, distance);
        position = rotate_dial(position, direction, distance);
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        // The example should return 6 for part 2
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn test_count_zero_crossings() {
        // TODO: Test cases for counting crossings

        // From 95, rotating R10 should cross 0 once
        // assert_eq!(count_zero_crossings(95, 'R', 10), 1);

        // From 5, rotating L10 should cross 0 once
        // assert_eq!(count_zero_crossings(5, 'L', 10), 1);

        // From 50, rotating R1000 should cross 0 ten times
        // assert_eq!(count_zero_crossings(50, 'R', 1000), 10);

        // From 10, rotating R20 should NOT cross 0
        // assert_eq!(count_zero_crossings(10, 'R', 20), 0);
    }
}
