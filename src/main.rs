// Function to find combinations of `n` numbers that sum up to `target_sum`
fn find_combinations(n: usize, target_sum: u32) -> Vec<Vec<u8>> {
    find_combinations_recursive(n, target_sum, 1)
}

// Recursive function to generate combinations with the current number to start from
fn find_combinations_recursive(n: usize, target_sum: u32, start: u8) -> Vec<Vec<u8>> {
    if n == 1 {
        // Base case: If there's only one number left, it must equal the target sum
        if start <= target_sum as u8 && target_sum <= 9 {
            return vec![vec![target_sum as u8]];
        } else {
            return vec![]; // No valid combinations
        }
    }

    let mut results = Vec::new();

    // Iterate through numbers from `start` to 9
    for num in start..=9 {
        if num >= target_sum as u8 {
            break; // No need to check further since numbers are too large
        }

        // Recursive case: Try fixing `num` and find combinations of (n-1) numbers for the remaining sum
        let remaining_sum = target_sum - num as u32;
        let sub_combinations = find_combinations_recursive(n - 1, remaining_sum, num + 1);

        // Append `num` to the front of each valid sub-combination
        for mut sub_combination in sub_combinations {
            sub_combination.insert(0, num);
            results.push(sub_combination);
        }
    }

    results
}

fn main() {
    // Example: Find combinations of 2 numbers that sum to 10
    let combinations = find_combinations(2, 10);
    println!("Combinations for sum 10 with 2 numbers:");
    for combo in combinations {
        println!("{:?}", combo);
    }

    // Example: Find combinations of 3 numbers that sum to 15
    let combinations_3 = find_combinations(3, 15);
    println!("\nCombinations for sum 15 with 3 numbers:");
    for combo in combinations_3 {
        println!("{:?}", combo);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    # Verifies that the function returns all valid pairs of numbers that sum to 10. 
    #[test]
    fn test_two_numbers_sum_to_10() {
        let expected = vec![
            vec![1, 9],
            vec![2, 8],
            vec![3, 7],
            vec![4, 6],
        ];
        let result = find_combinations(2, 10);
        assert_eq!(result, expected);
    }

    # Verifies that the function correctly handles 3 numbers summing to 15.
    #[test]
    fn test_three_numbers_sum_to_15() {
        let expected = vec![
            vec![1, 5, 9],
            vec![1, 6, 8],
            vec![2, 4, 9],
            vec![2, 5, 8],
            vec![2, 6, 7],
            vec![3, 4, 8],
            vec![3, 5, 7],
            vec![4, 5, 6],
        ];
        let result = find_combinations(3, 15);
        assert_eq!(result, expected);
    }

    # Tests the edge case where only 1 number is required.
    #[test]
    fn test_one_number_sum_to_5() {
        let expected = vec![vec![5]];
        let result = find_combinations(1, 5);
        assert_eq!(result, expected);
    }

    # Verifies that the function returns no combinations when the sum is too large for the possible numbers.
    #[test]
    fn test_no_valid_combinations_large_sum() {
        let result = find_combinations(2, 20);
        assert!(result.is_empty());
    }

    # Checks that the function correctly handles cases where the number of cells and the sum are incompatible.
    #[test]
    fn test_no_valid_combinations_too_many_numbers() {
        let result = find_combinations(5, 50); // 5 numbers summing to 50 is impossible
        assert!(result.is_empty());
    }

    # Tests the minimal valid combination (smallest sum of 2 distinct numbers).
    #[test]
    fn test_minimal_sum_with_two_numbers() {
        let expected = vec![vec![1, 2]]; // Smallest valid combination
        let result = find_combinations(2, 3);
        assert_eq!(result, expected);
    }
}