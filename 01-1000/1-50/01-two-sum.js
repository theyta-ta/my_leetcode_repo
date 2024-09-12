/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    const map = {};
    for (let i = 0; i < nums.length; i++) {
        const n = nums[i];

        if (map[n] !== undefined) {
            return [i, map[n]];
        } else {
            map[target - n] = i;
        }
    }
};
