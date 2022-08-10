struct Solution {} 

impl Solution {
    // Time Limit Exceed
    // array
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     let mut ans = 0;
    //     let length = prices.len();
    //     for i in 0..length - 1 {
    //         for j in i + 1..length {
    //             if prices[j] - prices[i] > ans {
    //                 ans = prices[j] - prices[i];
    //             }
    //         }
    //     }
    //     ans
    // }

    // dynamic programming
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut min_price = 10000;
        for i in 0..prices.len() {
            if min_price > prices[i] {
                min_price = prices[i];
            } else if ans < prices[i] - min_price {
                ans = prices[i] - min_price;
            }
        }
        ans
    }

    // monotonic stack
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     let mut ans = 0;
    //     let mut stack = Vec::new();
    //     for i in 0..prices.len() {
    //         if stack.len() == 0 || prices[i] > stack[stack.len() - 1] {
    //             stack.push(prices[i]);
    //         } else {
    //             while stack.len() > 0 && prices[i] < stack[stack.len() - 1] {
    //                 stack.pop();
    //             }
    //             stack.push(prices[i]);
    //         }
    //         if stack.len() >= 2 && stack[stack.len() - 1] - stack[0] > ans {
    //             ans = stack[stack.len() - 1] - stack[0];
    //         }
    //     }
    //     ans
    // }
}

fn main() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
}