pub struct Solution {}

impl Solution {
    pub fn three_sum_bf(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut nums_cloned = nums.clone();
        nums_cloned.sort();

        for first in 0..nums_cloned.len() {
            if first == 0 || nums_cloned[first] != nums_cloned[first - 1] {
                for second in first + 1..nums_cloned.len() {
                    if second == first + 1 || nums_cloned[second] != nums_cloned[second - 1] {
                        for third in second + 1..nums_cloned.len() {
                            if third == second + 1 || nums_cloned[third] != nums_cloned[third - 1] {
                                if Self::check(
                                    nums_cloned[first],
                                    nums_cloned[second],
                                    nums_cloned[third],
                                ) {
                                    let tuple = vec![
                                        nums_cloned[first],
                                        nums_cloned[second],
                                        nums_cloned[third],
                                    ];
                                    ret.push(tuple);
                                }
                            }
                        }
                    }
                }
            }
        }
        ret
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut nums_cloned = nums.clone();
        nums_cloned.sort();

        for first in 0..nums_cloned.len() {
            if first > 0 && nums_cloned[first] == nums_cloned[first - 1] {
                continue;
            }

            let mut third = nums_cloned.len() - 1;

            for second in first + 1..nums_cloned.len() {
                if second > 0 && nums_cloned[second] == nums_cloned[second - 1] {
                    continue;
                }

                while (second< third) && (nums_cloned[first] + nums_cloned[second] + nums_cloned[third]) > 0 {
                    third -= 1;
                }
                
                if Self::check(nums_cloned[first], nums_cloned[second], nums_cloned[third]) {
                    let tuple = vec![nums_cloned[first], nums_cloned[second], nums_cloned[third]];
                    ret.push(tuple);
                }
            }
        }
        ret
    }
    fn check(first: i32, second: i32, third: i32) -> bool {
        first + second + third == 0
    }
}
