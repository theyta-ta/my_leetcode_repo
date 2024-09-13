class Solution:
    def mySqrt(self, n: int) -> int:
        if n <= 1:
            return n
        # 1 << k is the same as 2^k where k is an integer
        # we want to only either ascend or descend to make the conditional easier. its easier to overestimate than underestimate
        # add 1 to guarantee an overestimate 
        xn = 1 << (n.bit_length() - 1)//2 + 1

        # you can newton rhapson to get this update function
        update = lambda xn: (xn + n//xn) // 2
        # x_n+1
        xn1 = update(xn)

        # descend on the answer
        # conditional only fails when xn == xn1 == floor(sqrt(n))
        while xn1 < xn:
            xn = xn1
            xn1 = update(xn)

        return xn 
