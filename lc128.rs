use std::cmp::max;
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut hash_set: HashSet<i32> = HashSet::new();
        for num in nums {
            let _ = hash_set.insert(num);
        }

        let mut longest_streak = 0;

        for num in &hash_set {
            if !hash_set.contains(&(num - 1)) {
                let mut current_num = num.to_owned();
                let mut current_streak = 1;

                while hash_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }
                longest_streak = max(longest_streak, current_streak);
            }
        }
        longest_streak
    }
}
