use std::collections::HashMap;

// Problem 1.
// https://leetcode.com/problems/two-sum/
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // val: index
    let mut prev_vals: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    for (idx, val) in nums.into_iter().enumerate() {
        let diff = target - val;
        match prev_vals.get(&diff) {
            Some(&prev_idx) => {
                return vec![prev_idx as i32, idx as i32];
            }
            None => {
                prev_vals.insert(val, idx);
            }
        }
    }

    // Because each input would have exactly one solution.
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2, 7, 11, 15], 9, vec![0, 1])]
    #[case(vec![3, 2, 4], 6, vec![1, 2])]
    #[case(vec![3, 3], 6, vec![0, 1])]
    fn two_sum_test(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        let result = two_sum(nums, target);
        assert_eq!(result, expected);
    }
}
