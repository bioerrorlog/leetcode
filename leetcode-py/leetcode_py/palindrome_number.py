

# Problem 9.
# https://leetcode.com/problems/palindrome-number/
class Solution:
    def is_palindrome(self, x: int) -> bool:
        x = str(x)

        while len(x) >= 2:
            x, first = x[1:], x[0]
            x, last = x[:-1], x[-1]

            if first != last:
                return False

        return True
