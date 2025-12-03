// Day 2: Gift Shop - Invalid Product IDs
//
// Problem: Find all product IDs that are a sequence of digits repeated exactly twice.
// Example: 11, 6464, 123123 are invalid (repeated patterns)

/// Check if a number is invalid (pattern repeated twice)
/// A number is invalid if it's even-length and first half == second half
fn is_invalid_id(num: u64) -> bool {
    let s = num.to_string(); // Convert number to string

    if !s.len().is_multiple_of(2) {
        return false; // Odd length cannot be repeated pattern
    }

    // where mid = length / 2
    let mid = s.len() / 2;
    let first_half = &s[0..mid]; // First half of the string
    let second_half = &s[mid..]; // Second half of the string

    first_half == second_half // Check if both halves are equal
}

/// Parse a range string like "11-22" into (start, end)
pub fn parse_range(range_str: &str) -> (u64, u64) {
    // Split the string by "-"
    let parts: Vec<&str> = range_str.split('-').collect(); // Split into two parts by '-' and saving in a vector
    if parts.len() != 2 {
        panic!("Invalid range format"); // Ensure we have exactly two parts
    }

    // Collect the parts
    let start_str = parts[0]; // First part before '-'
    let end_str = parts[1]; // Second part after '-'

    // Parse each part as u64
    let start = start_str.parse::<u64>().expect("Failed to parse start number"); // Convert first part to u64
    let end = end_str.parse::<u64>().expect("Failed to parse end number"); // Convert second part to u64

    (start, end)
}

/// Main solution function
pub fn solve(input: &str) -> u64 {
    let mut total = 0u64; // Total sum of invalid IDs

    for range_str in input.trim().split(',') {
        // Skip empty strings
        if range_str.trim().is_empty() {
            continue;
        } // Trim whitespace

        let (start, end) = parse_range(range_str); // Parse the range

        for num in start..=end {
            // Iterate through the range start to end
            if is_invalid_id(num) {
                // Check if the number is invalid
                total += num; // Add to total if invalid
            }
        }
    }

    total // Return the total sum of invalid IDs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        // Invalid IDs (repeated patterns)
        assert_eq!(is_invalid_id(11), true); // "11" = "1" + "1"
        assert_eq!(is_invalid_id(99), true); // "99" = "9" + "9"
        assert_eq!(is_invalid_id(6464), true); // "6464" = "64" + "64"
        assert_eq!(is_invalid_id(123123), true); // "123123" = "123" + "123"

        // Valid IDs (not repeated patterns)
        assert_eq!(is_invalid_id(101), false); // Not a simple repeat
        assert_eq!(is_invalid_id(123), false); // Odd length
        assert_eq!(is_invalid_id(1234), false); // "12" != "34"
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("11-22"), (11, 22));
        assert_eq!(parse_range("95-115"), (95, 115));
        assert_eq!(parse_range("998-1012"), (998, 1012));
    }

    #[test]
    fn test_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                     1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                     824824821-824824827,2121212118-2121212124";

        // The example should return 1227775554
        assert_eq!(solve(input), 1227775554);
    }
}
