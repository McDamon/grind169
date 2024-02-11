// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;

        let mut min_price = i32::MAX;

        prices.into_iter().for_each(|price| {
            let profit = price - min_price;
            if min_price > price {
                min_price = price;
            } else if profit > max_profit {
                max_profit = profit;
            }
        });

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::best_time_to_buy_and_sell_stock::Solution;

    #[test]
    fn test_max_profit_case1() {
        assert_eq!(Solution::max_profit([7, 1, 5, 3, 6, 4].to_vec()), 5);
    }

    #[test]
    fn test_max_profit_case2() {
        assert_eq!(Solution::max_profit([7, 6, 4, 3, 1].to_vec()), 0);
    }
}
