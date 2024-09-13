class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        # this ones piss easy lmaoooo
        return math.comb(m + n - 2, n - 1)
