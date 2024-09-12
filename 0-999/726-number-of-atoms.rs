use std::collections::{BTreeMap, HashMap};
use regex::Regex;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let re = Regex::new(r"([A-Z][a-z]*)(\d*)|(\()()|(\))(\d*)").unwrap();
        let mut caps: Vec<_> = re.captures_iter(&formula).map(|c| c.extract().1 ).collect();
        caps.reverse();

        let mut elems = HashMap::new();
        let mut mult_stack = Vec::new();
        let mut cur_mult = 1;

        for [s, d] in caps {
            match s {
                "(" => { cur_mult /= mult_stack.pop().unwrap(); },
                ")" => {
                    let factor = d.parse().unwrap_or(1);
                    cur_mult *= factor;
                    mult_stack.push(factor);
                },
                name => {
                    //println!("{}-{} mutl {cur_mult}", s, d);
                    let num = cur_mult * d.parse().unwrap_or(1);
                    elems.entry(name).and_modify(|e| *e += num).or_insert(num);
                },
            }
        }

        let elems: BTreeMap<_, _> = elems.into_iter().collect();
        elems.iter().map(|(&name, &count)| {
            let count = if count == 1 { "" } else { &count.to_string() };
            let mut name = name.to_string();
            name.push_str(count);
            name
        }).collect()
    }
}
