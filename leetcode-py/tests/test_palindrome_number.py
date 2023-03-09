import pytest
from leetcode_py.palindrome_number import Solution


@pytest.mark.parametrize('input, expected',
                         [
                             (121, True),
                             (-121, False),
                             (10, False),
                         ]
                         )
def test_two_sum(input: int, expected: bool):
    assert Solution().is_palindrome(input) == expected
