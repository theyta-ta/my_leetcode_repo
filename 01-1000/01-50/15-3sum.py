#lmaoooo this code is shit! this was made to explain 3sum to a friend tho...
# so i wasnt caring too much about efficiency.
# also why so many comments??? huh???


# this code is **not** efficient. but i hope its helpful
class Solution(object):
    # we will twosum in a loop.
    # for each `i` in `nums` we will twosum for `-i`

    # one thing to note. [a, b] + [c] == [a, b, c]
    def threeSum(self, nums):
        # we dont want duplicates so lets use a dict
        # a set would make more sense but idk if yall know about them
        s = {}
        # lets sort the list. 
        # twosum is possible w/out dicts in O(n) time if input is soeted
        nums = sorted(nums)

        # we want to reduce the number of checks done
        # so for each element only twosum over smaller elements
        # because the list is sorted this is just all previous elements
        # this also means that `x` is the max of the three elems that sum to 0
        # we will want to store the elements sorted in the dict 
        # else [1, 2, -3] and [-3, 1, 2] would be treated as different
        for (i, x) in enumerate(nums):
            for y in self.twosum_sorted(nums[:i], -x):
                # lists are *mutable*, and so they cannot be used as keys for dicts
                # we will need to convert it to a tuple first
                key = tuple(y + [x])
                # we dont care about the value. set it to garbage.
                s[key] = None

        ret = []
        for i in s:
            # because we set the keys to be tuples we have to convert them back to lists first
            ret.append(list(i))

        return ret
    
    def twosum_sorted(self, nums, target):

        i = 0
        j = len(nums) - 1
        ret = []
        while i < j:
            _sum = nums[i] + nums[j]
            if _sum < target:
                i += 1
            elif _sum > target:
                j -= 1
            else: 
                ret.append([nums[i], nums[j]])
                i += 1
                j -= 1

        return ret
