use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut count = [0; 26];
            for b in s.bytes().map(|b| b - b'a') {
                count[b as usize] += 1;
            }
            map.entry(count).and_modify(|v| v.push(s.clone())).or_insert(vec![s]);
        }

        map.into_values().collect()
    }
}
