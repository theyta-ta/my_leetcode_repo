impl Solution {
    // if you look, the first digit is separated in blocks of (n-1)
    // and after the first digit its just the same damn thing
    // on n-1 elements this time. so just rotate right by 1 to get
    // the correct element in front, then iterate this.
    // obviously the next k is just k / (n-1)!
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut v: Vec<_> = (1..=n).map(|x| b'0' + x as u8).collect();
        let mut f: i32 = (1..n).product(); // store (n-1)!. will use this to calc (n-2)! ...

        let mut k: i32 = k - 1; // urgh. just... please start counting at 0.

        for i in (1..n) {
            let pos = (k / f) as usize;
            let left = i as usize - 1;
            let right = left + pos;
            v[left..=right].rotate_right(1);

            k %= f;
            f /= n - i;
        }

        // collect into a string
        v.into_iter().map(|x| x as char).collect::<String>()
    }
}
