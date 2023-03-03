from typing import List


# Problem 1.
# https://leetcode.com/problems/two-sum/
class Solution:
    def two_sum(self, nums: List[int], target: int) -> List[int]:
        prev_vals = {}  # val: index

        for i, val in enumerate(nums):
            diff = target - val
            if diff in prev_vals:
                return [prev_vals[diff], i]

            prev_vals[val] = i

        return
