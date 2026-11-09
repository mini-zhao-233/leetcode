use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

        for str in strs {
            let mut key = str.as_bytes().to_vec();
            key.sort_unstable();

            map.entry(key).or_default().push(str)
        }
        map.into_values().collect()
    }
}
