# Problem 1.
# https://leetcode.com/problems/two-sum/
from typing import List


class Solution:
    def two_sum(self, nums: List[int], target: int) -> List[int]:
        control = [target - i for i in nums]

        result = []
        for i, val in enumerate(control):
            for j in nums:
                if val == j:
                    result.append(i)

        return result
