class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        # enums the old... annoying way
        TOP = 0
        RIGHT = 1
        BOTTOM = 2
        LEFT = 3
    
        current_side = TOP
        ret = []
    
        bounds = [0] * 4
        bounds[RIGHT] = len(matrix[0])
        bounds[BOTTOM] = len(matrix)
    
        while bounds[LEFT] < bounds[RIGHT] and bounds[TOP] < bounds[BOTTOM]:
            # this if elif mess should be a match case block
            # but python makes this do annyoing to do
            if current_side == TOP:
                ret += matrix[bounds[TOP]][bounds[LEFT] : bounds[RIGHT]]
                bounds[TOP] += 1
                current_side = RIGHT
    
            elif current_side == RIGHT:
                right_side = (matrix[i][bounds[RIGHT] - 1] for i in range(bounds[TOP], bounds[BOTTOM]))
                ret.extend(right_side)
                bounds[RIGHT] -= 1
                current_side = BOTTOM
    
            elif current_side == BOTTOM:
                ret += reversed(matrix[bounds[BOTTOM] - 1][bounds[LEFT] : bounds[RIGHT]])
                bounds[BOTTOM] -= 1
                current_side = LEFT
    
            elif current_side == LEFT:
                left_side = (
                    matrix[i][bounds[LEFT]]
                    for i in reversed(range(bounds[TOP], bounds[BOTTOM]))
                )
                ret.extend(left_side)
                bounds[LEFT] += 1
                current_side = TOP
        
        return ret
    
