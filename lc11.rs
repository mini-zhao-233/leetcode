use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        for i in 1..height.len() {
            for j in 0..i {
                let area = i32::try_from(i - j).unwrap() * min(height[i], height[j]);
                max_area = max(max_area, area);
            }
        }
        max_area
    }
}
