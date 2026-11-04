pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut cursor = 0usize;

        for i in 0..nums.len() {
            let mut temp = 0;
            if nums[i] != 0 {
                nums.swap(i,cursor);
                cursor += 1;
            }
        }
        
        println!("{:?}", nums);
    }
}
