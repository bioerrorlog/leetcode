use std::collections::HashMap;

// Problem 1.
// https://leetcode.com/problems/two-sum/
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
