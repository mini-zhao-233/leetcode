use std::cmp::{max, min};
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max_height = 0;
        let n = height.len();
        let mut cl = 0;
        let mut cr = n;
        let mut total_volume = 0;

        while cl != cr {
            //获取当前左右指针的大小，按照小的一端设置当前高度
            let hl = height[cl];
            let hr = height[cr - 1];

            //如果当前指针所围成的高度不大于之前已经处理的最大高度，则跳过
            let current_height = min(hl, hr);
            if current_height <= max_height {
                if hl < hr {
                    cl += 1;
                } else {
                    cr -= 1;
                }
                continue;
            }

            //计算当前高度的水体积
            let mut current_layer_volume = 0;
            for i in cl + 1..cr {
                let column_volume = max(current_height - max(height[i], max_height), 0);
                current_layer_volume += column_volume;
            }
            total_volume += current_layer_volume;

            if hl < hr {
                cl += 1;
            } else {
                cr -= 1;
            }
            max_height = current_height;
        }
        total_volume
    }
}
