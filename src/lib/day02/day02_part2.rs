// Day 2 Part 2: Gift Shop - Extended Invalid Product IDs
//
// Problem: Find all product IDs that are a sequence of digits repeated AT LEAST twice.
// Example: 123123 (123 × 2), 12341234 (1234 × 2), 111 (1 × 3) are all invalid

use super::day02_part1::parse_range;

/// Check if a number is invalid (pattern repeated at least twice)
/// A number is invalid if ANY pattern divides it evenly and repeats
fn is_invalid_id(num: u64) -> bool {
    let s = num.to_string(); // Convert number to string
    let len = s.len(); // Length of the string

    // Try all possible pattern lengths
    for pattern_len in 1..=(len / 2) {
        // Check if pattern length divides evenly
        if len.is_multiple_of(pattern_len) {
            let repetitions = len / pattern_len;
            if repetitions >= 2 {
                let pattern = &s[0..pattern_len];
                let constructed = pattern.repeat(repetitions);
                if constructed == s {
                    return true; // Found a valid repeating pattern
                }
            }
        }
    }

    false // No repeating pattern found
}

/// Main solution function for Part 2
pub fn solve(input: &str) -> u64 {
    let mut total = 0u64;

    for range_str in input.trim().split(',') {
        if range_str.trim().is_empty() {
            continue;
        }

        let (start, end) = parse_range(range_str.trim());

        for num in start..=end {
            if is_invalid_id(num) {
                total += num;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        // Invalid IDs (repeated patterns - at least twice)
        assert_eq!(is_invalid_id(11), true); // "1" × 2
        assert_eq!(is_invalid_id(99), true); // "9" × 2
        assert_eq!(is_invalid_id(111), true); // "1" × 3
        assert_eq!(is_invalid_id(1111111), true); // "1" × 7
        assert_eq!(is_invalid_id(123123), true); // "123" × 2
        assert_eq!(is_invalid_id(123123123), true); // "123" × 3
        assert_eq!(is_invalid_id(1212121212), true); // "12" × 5

        // Valid IDs (not repeated patterns)
        assert_eq!(is_invalid_id(101), false); // No repeating pattern
        assert_eq!(is_invalid_id(123), false); // Single occurrence
        assert_eq!(is_invalid_id(1234), false); // No repeating pattern
    }

    #[test]
    fn test_example_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                     1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                     824824821-824824827,2121212118-2121212124";

        // The example should return 4174379265
        assert_eq!(solve(input), 4174379265);
    }
}
