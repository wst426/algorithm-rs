struct Solution {}

use std::collections::HashMap;

impl Solution {
    // array
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     for i in 0..nums.len() {
    //         for j in i + 1..nums.len() {
    //             if nums[i] + nums[j] == target {
    //                 return vec![i as i32, j as i32];
    //             }
    //         }
    //     }
    //     unreachable!();
    // }

    // hash
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map = HashMap::new();
        for i in 0..nums.len() {
            if let Some(&v) = nums_map.get(&(target - nums[i])) {
                return vec![v, i as i32];
            } else {
                nums_map.insert(nums[i], i as i32);
            }
        }
        unreachable!();
    }    
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 5], 9));
}