struct Solution {} 

use std::collections::HashMap;

impl Solution {
    // hash
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut nums_map: HashMap<i32, i32> = HashMap::new();
        for i in nums.iter() {
            if let Some(&v) = nums_map.get(&i) {
                nums_map.insert(*i, v + 1);
            } else {
                nums_map.insert(*i, 1);
            }
        }
        for (k, v) in nums_map {
            if v > nums.len() as i32 / 2 {
                return k;
            }
        }
        unreachable!();
    }

    // sort
    // pub fn majority_element(mut nums: Vec<i32>) -> i32 {
    //     nums.sort();
    //     nums[nums.len() / 2]
    // }
}

fn main() {
    println!("{}", Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
}