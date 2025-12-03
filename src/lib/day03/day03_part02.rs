// Day 3 Part 2: Lobby - Battery Joltage (12 batteries)
//
// Problem: For each bank, pick exactly 12 batteries to form the largest 12-digit number.
// Strategy: Greedily select the largest digit at each position while ensuring
// enough digits remain for the remaining positions.

/// Find the maximum joltage by selecting exactly `count` batteries from a bank
fn max_joltage_for_bank(bank: &str, count: usize) -> u64 {
    let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect(); // Convert chars to digits

    let mut result = Vec::new(); // Store selected digits
    let mut start_index = 0; // Start searching from this index

    for position in 0..count {
        // For each position to fill
        let remaining = count - position - 1; // Remaining positions to fill
        let max_search_index = digits.len() - remaining; // Limit search to ensure enough digits remain

        let mut max_digit = 0; // Track the maximum digit found
        let mut max_digit_index = start_index; // Track index of the maximum digit

        for (i, &digit) in digits.iter().enumerate().skip(start_index).take(max_search_index - start_index) {
            // Search for the max digit
            if digit > max_digit {
                // Found a new maximum
                max_digit = digit; // Update max digit
                max_digit_index = i; // Update index of max digit
            }
        }

        result.push(max_digit); // Add the found max digit to the result
        start_index = max_digit_index + 1; // Move start index forward
    }

    result.iter().fold(0u64, |acc, &d| acc * 10 + d as u64) // Convert selected digits to a single number
}

/// Main solution function
pub fn solve(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let bank_joltage = max_joltage_for_bank(line.trim(), 12);
        total += bank_joltage;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage_for_bank() {
        // Test individual banks with 12-battery selection
        assert_eq!(max_joltage_for_bank("987654321111111", 12), 987654321111);
        assert_eq!(max_joltage_for_bank("811111111111119", 12), 811111111119);
        assert_eq!(max_joltage_for_bank("234234234234278", 12), 434234234278);
        assert_eq!(max_joltage_for_bank("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        // Total: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619
        assert_eq!(solve(input), 3121910778619);
    }
}
