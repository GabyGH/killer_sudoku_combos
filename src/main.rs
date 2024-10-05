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