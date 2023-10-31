/**
 *  Problem 121. Best Time to Buy/Sell Stock (Easy)
 *  See: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
 * 
 *  You are given an array prices where prices[i] is the price of a given
 *  stock on the ith day.
 * 
 *  You want to maximize your profit by choosing a single day to buy one
 *  stock and choosing a different day in the future to sell that stock.
 *  
 *  Return the maximum profit you can achieve from this transaction.
 * 
 *  If you cannot achieve any profit, return 0.
 */
pub fn run(prices: Vec<i32>) -> i32 {
    let mut cur_min_index = 0;
    let (mut cur_min, mut cur_max) = (prices[0], prices[0]);
    let mut max = 0;

    for i in 1..prices.len() {
        let cur_price = prices[i];

        if cur_price < cur_min {
            let profit = cur_max - prices[cur_min_index];
            if profit > max {
                max = profit;
            }
            cur_min = cur_price;
            cur_max = cur_price;
            cur_min_index = i;
        } else if cur_price > cur_max {
            cur_max = cur_price;
        }
    }

    let end_profit = cur_max - prices[cur_min_index];
    if end_profit > max { end_profit } else { max }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_calculates_the_max_profit_of_buying_and_selling_stocks() {
        // Buy on day 2, sell on day 5 => 6 - 1 = 5
        assert_eq!(
            run(vec![7, 1, 5, 3, 6, 4]),
            5
        );

        // Constant decreasing, no profit
        assert_eq!(
            run(vec![7, 6, 4, 3, 1]),
            0
        );
    }
}