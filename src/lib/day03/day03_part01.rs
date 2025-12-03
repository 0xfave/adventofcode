// Day 3: Lobby - Battery Joltage
//
// Problem: For each bank of batteries, pick exactly two batteries to maximize joltage.
// The joltage is the two-digit number formed by the selected batteries (maintaining order).

/// Find the maximum joltage possible from a single bank
/// A bank is a string of digits, and we need to pick two positions (i, j) where i < j
/// to form the largest two-digit number
fn max_joltage_for_bank(bank: &str) -> u32 {
    let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect(); // Convert chars to digits

    let mut max_joltage = 0; // Track the maximum joltage found
    for i in 0..digits.len() {
        // First digit position
        for j in (i + 1)..digits.len() {
            // Second digit position
            let joltage = digits[i] * 10 + digits[j]; // Form the two-digit number
            if joltage > max_joltage {
                // Update max if larger
                max_joltage = joltage; // New maximum found
            }
        }
    }

    max_joltage // Return the maximum joltage for this bank
}

/// Main solution function
pub fn solve(input: &str) -> u32 {
    let mut total = 0; // Total joltage across all banks

    for line in input.lines() {
        // Process each bank line
        if line.trim().is_empty() {
            // Skip empty lines
            continue;
        }

        let bank_joltage = max_joltage_for_bank(line.trim()); // Get max joltage for this bank
        total += bank_joltage; // Add to total
    }

    total // Return the total joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage_for_bank() {
        // Test individual banks
        assert_eq!(max_joltage_for_bank("987654321111111"), 98);
        assert_eq!(max_joltage_for_bank("811111111111119"), 89);
        assert_eq!(max_joltage_for_bank("234234234234278"), 78);
        assert_eq!(max_joltage_for_bank("818181911112111"), 92);
    }

    #[test]
    fn test_example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        // The example should return 357 (98 + 89 + 78 + 92)
        assert_eq!(solve(input), 357);
    }
}
