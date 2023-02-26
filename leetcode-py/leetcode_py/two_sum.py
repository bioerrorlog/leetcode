# Problem 1.
# https://leetcode.com/problems/two-sum/
from typing import List


class Solution:
    def two_sum(self, nums: List[int], target: int) -> List[int]:
        control = [target - i for i in nums]

        result = []
        for i, ctrl_val in enumerate(control):
            for j, val in enumerate(nums):
                if i != j and ctrl_val == val:
                    result.append(i)

        return result
