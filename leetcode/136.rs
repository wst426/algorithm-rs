struct Solution {} 

use std::collections::HashMap;

impl Solution {
    // hash
    // pub fn single_number(nums: Vec<i32>) -> i32 {
    //     let mut nums_map = HashMap::new();
    //     for i in nums {
    //         if let Some(&v) = nums_map.get(&i) {
    //             nums_map.insert(i, v + 1);
    //         } else {
    //             nums_map.insert(i, 1);
    //         }
    //     }
    //     for (k, v) in nums_map {
    //         if v == 1 {
    //             return k;
    //         }
    //     }
    //     unreachable!();
    // }

    // bitwise operation
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |accum, item| accum ^ item)
    }
}

fn main() {
    println!("{}", Solution::single_number(vec![4, 1, 2, 1, 2]));
}