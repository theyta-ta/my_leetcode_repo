# TIME: O(len(nums))
# SPACE: O(1) - the algorithm can easily be extended to finding an
# increasing n-tuple. in which case it is O(n)
class Solution:
    def increasingTriplet(self, nums: List[int]) -> bool:
        # i find thinking about this one to be easier when thinking
        # about finding either the longest increasing subsequence or
        # an n-tuple. instead of specifically a triplet, so thats
        # how ill explain it.
        #
        # we can form an increasing n-tuple from an increasing (n-1)-tuple
        # and a number bigger than the last number in the (n-1)-tuple
        # so for the 3-tuple case, we need a 2-tuple and a 1-tuple
        # and then if the number is bigger than the 2-tuple
        # we have the 3-tuple. the triplet we need. 
        # if its not, but its bigger than the 1-tuple, then we can form
        # a smaller 2-tuple. and if its not bigger than the 1-tuple, we
        # can form a smaller 1-tuple.
        #
        # this works! however, since we dont care about what the triplet is
        # we dont need to store the whole triplet, do we. same with the 2-tuple
        # all we need to store is a number to compare to, to see if our new number
        # is better or worse.
        # we only need the largest/final element of each tuple

        MAX = 2 ** 31 # bigger than any value in `nums`
        finals = [MAX, MAX]

        for num in nums:
            # check <= as the question asks for strictly increasing
            if num <= finals[0]:
                finals[0] = num
            elif num <= finals[1]:
                finals[1] = num
            else:
                return True
        return False
