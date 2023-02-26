from leetcode_py.two_sum import Solution


def test_two_sum():
    sol = Solution()

    # Input
    nums = [2, 7, 11, 15]
    target = 9

    expected = [0, 1]

    assert sol.two_sum(nums, target) == expected
