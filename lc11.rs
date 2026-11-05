use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn max_area_bf(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        for i in 1..height.len() {
            for j in 0..i {
                let area = i32::try_from(i - j).unwrap() * min(height[i], height[j]);
                max_area = max(max_area, area);
            }
        }
        max_area
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut cl = 0usize;
        let mut cr = height.len() - 1;
        let mut max_area = 0;

        while cl != cr {
            let area = i32::try_from(cr - cl).unwrap() * min(height[cl], height[cr]);
            max_area = max(max_area, area);
            if height[cl] < height[cr] {
                cl += 1;
            } else {
                cr -= 1;
            }
        }
        max_area
    }
}
