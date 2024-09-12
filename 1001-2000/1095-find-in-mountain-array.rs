/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */

use std::cmp::Ordering;

impl Solution {
    pub fn find_in_mountain_array(target: i32, m_arr: &MountainArray) -> i32 {
        let len = m_arr.length();

        // find max
        let mut l_ptr = 0;
        let mut r_ptr = len - 1;
        let midpoint = |a, b| (a & b) + ((a ^ b) >> 1);
        while l_ptr < r_ptr {
            let mid = midpoint(l_ptr, r_ptr);
            let m_mid = m_arr.get(mid);
            //if m_mid == target { return mid; }

            let m_next = m_arr.get(mid + 1);
            //if m_next == target { return mid + 1; }

            match m_mid.cmp(&m_next) {
                Ordering::Less => l_ptr = mid + 1,
                Ordering::Greater => r_ptr = mid - 1,
                Ordering::Equal => unreachable!(),
            }
        };
        let peak = l_ptr;
        //println!("{peak}");

        // binary search for target in each half
        l_ptr = 0; r_ptr = peak;
        while l_ptr <= r_ptr {
            let mid = midpoint(l_ptr, r_ptr);
            match m_arr.get(mid).cmp(&target) {
                Ordering::Less => l_ptr = mid + 1,
                Ordering::Greater => r_ptr = mid - 1,
                Ordering::Equal => return mid,
            }
        }

        l_ptr = peak + 1; r_ptr = len - 1;
        while l_ptr <= r_ptr {
            let mid = midpoint(l_ptr, r_ptr);
            match m_arr.get(mid).cmp(&target) {
                Ordering::Less => r_ptr = mid - 1,
                Ordering::Greater => l_ptr = mid + 1,
                Ordering::Equal => return mid,
            }
        }

        -1
    }
}
