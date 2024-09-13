#this is slow this is sloooowwwwww
import numpy as np

class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        if num1 == "0" or num2 == "0": return "0"
        # convert strs to list of ints
        # fft. then mult coefficients pairwise
        # then inverse fft. the convert from list of int to str
        l = len(num1) + len(num2)
        #rfft was bugging for me. if l==3 would only make len 2 output
        num1 = np.fft.fft(np.fromiter(map(int, num1), np.int64), l)
        num2 = np.fft.fft(np.fromiter(map(int, num2), np.int64), l)
        coeffs = np.fft.ifft(num1 * num2)
        ret = ["0"] * len(coeffs)
        c = 0
        for idx, coeff in enumerate(reversed(coeffs)):
            c += int(round(coeff))
            ret[-idx] = c % 10
            c = c // 10
        ret[0] = c

        return "".join(map(str, ret)).lstrip("0")
