use std::cmp::{min, max};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut profit = 0;
        for i in 0..prices.len(){
            // buy = min(buy, prices[i]);
            // profit = max(profit, prices[i]-buy);

            if prices[i] < buy{
                buy = prices[i];
            }else if prices[i] - buy > profit{
                profit = prices[i] - buy;
            }
        }
        profit        
    }
}
