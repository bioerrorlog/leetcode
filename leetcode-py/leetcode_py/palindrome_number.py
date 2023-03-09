

# Problem 9.
# https://leetcode.com/problems/palindrome-number/
class Solution:
    def is_palindrome(self, x: int) -> bool:
        if x < 0 or (x > 0 and x % 10 == 0):
            return False

        return str(x) == str(x)[::-1]
