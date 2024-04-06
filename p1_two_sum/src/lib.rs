use std::cmp::Ordering;

/// Start at either end
/// If we're lower than target, increase lower end
/// If we're higher than target, decrease upper end
fn sorted_end_walk(nums: &[i32], target: i32) -> Vec<usize> {

    // Sort & add enumeration to keep track of true indices
    let mut nums = nums.iter()
        .enumerate()
        .collect::<Vec<_>>();

    nums.sort_unstable_by(|(_, &x), (_, &y)| x.cmp(&y));

    // End to end walk
    let mut i = 0;
    let mut j = nums.len() - 1;

    loop {
        match (nums[i].1 + nums[j].1).cmp(&target) {
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
            Ordering::Equal => return vec![nums[i].0, nums[j].0]
        }
    }
}

/// Brute force, but we stop early on inner loop when we get too high
fn sorted_early_stopping(nums: &[i32], target: i32) -> Vec<usize> {
    let mut nums = nums.iter()
        .enumerate()
        .collect::<Vec<_>>();

    nums.sort_unstable_by(|(_, &x), (_, &y)| x.cmp(&y));

    for (k, (i, &x)) in nums.iter().take(nums.len() - 1).enumerate() {
        let pair_target = target - x;
        for (j, &y) in nums[k + 1..].iter() {
            match y.cmp(&pair_target) {
                Ordering::Equal => return vec![*i, *j],
                Ordering::Greater => break,
                _ => {}
            }
        }
    }

    panic!("Failed to find valid solution")
}

/// Brute force
fn dumb_solution(nums: &[i32], target: i32) -> Vec<usize> {
    for i in 0..nums.len() - 1 {
        let pair_target = target - nums[i];

        for j in i + 1..nums.len() {
            if nums[j] == pair_target {
                return vec![i, j];
            }
        }
    }

    panic!("Failed to find valid solution")
}

pub fn find_indices(nums: &[i32], target: i32) -> Vec<usize> {
    // dumb_solution(nums, target)
    // sorted_early_stopping(nums, target)
    sorted_end_walk(nums, target)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let solution = HashSet::from([0, 1]);

        let guess = HashSet::from_iter(find_indices(&nums, target));
        assert_eq!(guess, solution);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let solution = HashSet::from([1, 2]);

        let guess = HashSet::from_iter(find_indices(&nums, target));
        assert_eq!(guess, solution);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 3];
        let target = 6;
        let solution = HashSet::from([0, 1]);

        let guess = HashSet::from_iter(find_indices(&nums, target));
        assert_eq!(guess, solution);
    }

    #[test]
    fn worst_case() {
        let nums = (0..10_i32.pow(4) + 1).collect::<Vec<_>>();
        let target = 10_i32.pow(4) + 10_i32.pow(4) - 1;
        let solution = HashSet::from([10_usize.pow(4) - 1, 10_usize.pow(4)]);

        let guess = HashSet::from_iter(find_indices(&nums, target));
        assert_eq!(guess, solution);
    }

    #[test]
    fn worst_case_reverse() {
        let nums = (0..10_i32.pow(4) + 1).rev().collect::<Vec<_>>();
        let target = 10_i32.pow(4) + 10_i32.pow(4) - 1;
        let solution = HashSet::from([0, 1]);

        let guess = HashSet::from_iter(find_indices(&nums, target));
        assert_eq!(guess, solution);
    }
}
