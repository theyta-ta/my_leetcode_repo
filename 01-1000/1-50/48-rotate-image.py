class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        l = 0
        r = len(matrix) - 1

        while l < r:
            for i in range(r - l):
                temp = matrix[l][l+i]
                matrix[l][l+i] = matrix[r-i][l] 
                matrix[r-i][l] = matrix[r][r-i]
                matrix[r][r-i] = matrix[l+i][r]
                matrix[l+i][r] = temp
            l += 1; r -= 1
