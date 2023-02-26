import pytest
from leetcode_py.two_sum import Solution


@pytest.mark.parametrize('nums, target, expected',
    [
        ([2, 7, 11, 15], 9, [0, 1]),
    ]
)
def test_two_sum(nums, target, expected):
    assert Solution().two_sum(nums, target) == expected
