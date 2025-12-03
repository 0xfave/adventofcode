// Day 1: Secret Entrance - Dial Safe Cracker
//
// Problem: Track a dial (0-99) starting at 50, follow rotation instructions,
// and count how many times it lands on 0.

/// Parse a single instruction line like "L50" or "R6"
/// Returns: (direction_char, distance_number)
pub fn parse_instruction(line: &str) -> (char, i32) {
    let direction = line.chars().next().expect("Empty instruction line");
    let distance = line[1..].parse::<i32>().expect("Failed to parse distance");

    (direction, distance)
}

/// Calculate the new position after a rotation
///
/// Parameters:
///   - current: current dial position (0-99)
///   - direction: 'L' or 'R'
///   - distance: how many clicks to rotate
///
/// Returns: new position (0-99)
pub fn rotate_dial(current: i32, direction: char, distance: i32) -> i32 {
    // L means subtract (move left/down), R means add (move right/up)
    let change = match direction {
        'L' => -distance,
        'R' => distance,
        _ => panic!("Invalid direction character"),
    };

    ((current + change) % 100 + 100) % 100
}

/// Main solution function
pub fn solve(input: &str) -> i32 {
    let mut position = 50; // Starting position
    let mut zero_count = 0; // Count how many times we land on 0

    for line in input.lines() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Parse the instruction
        let (direction, distance) = parse_instruction(line);
        // Calculate the new position
        position = rotate_dial(position, direction, distance);
        // Check if we landed on 0
        if position == 0 {
            zero_count += 1; // Increment counter
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
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

        // The example should return 3
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction("L50"), ('L', 50));
        assert_eq!(parse_instruction("R6"), ('R', 6));
    }

    #[test]
    fn test_rotate_dial() {
        // Starting at 11, R8 should give 19
        assert_eq!(rotate_dial(11, 'R', 8), 19);

        // Starting at 5, L10 should give 95
        assert_eq!(rotate_dial(5, 'L', 10), 95);
    }
}
