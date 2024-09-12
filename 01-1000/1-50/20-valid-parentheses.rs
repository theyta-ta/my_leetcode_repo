impl Solution {
    pub fn is_valid(s: String) -> bool {
        let (fst, lst) = ("([{)", ")]}");
        let mut count = Vec::new();
        for chara in s.chars() {
            match lst.find(chara) {
                Some(ind) => if count.pop() != fst.chars().nth(ind) {
                    return false;
                },
                None => count.push(chara),
            }
        }
        count.is_empty()
    }
}
