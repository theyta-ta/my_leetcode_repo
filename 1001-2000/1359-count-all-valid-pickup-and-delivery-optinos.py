class Solution:
    def countOrders(self, n: int) -> int:
        ret = 1
        for i in range(1, n+1): 
            ret = ret*i*(2*i-1) % 1_000_000_007
        return ret
