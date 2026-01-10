fn main() {}

struct Solution {}

impl Solution {
    // time complexity: O(n)
    // space complexity: O(n)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }

        let mut temp_min: i32 = prices[0];
        let mut max_diff: i32 = 0;

        for n in prices {
            if n - temp_min > max_diff {
                max_diff = n - temp_min;
            }
            if n < temp_min {
                temp_min = n;
            }
        }

        max_diff
    }

    pub fn max_profit_recursive(prices: Vec<i32>) -> i32 {
        fn helper(prices: &[i32], idx: usize, max_diff: i32, temp_min: i32) -> i32 {
            if idx == prices.len() {
                return max_diff;
            }

            let price = prices[idx];
            let new_max = max_diff.max(price - temp_min);
            let new_min = temp_min.min(price);

            helper(prices, idx + 1, new_max, new_min)
        }
        if prices.is_empty() {
            return 0;
        }

        helper(&prices, 0, 0, prices[0])
    }

    pub fn max_profit_recursive2(prices: Vec<i32>) -> i32 {
        fn helper(prices: &[i32], min_price: i32, max_diff: i32) -> i32 {
            if prices.is_empty() {
                return max_diff;
            }

            let new_max = if prices[0] - min_price > max_diff {
                prices[0] - min_price
            } else {
                max_diff
            };
            let new_min = if prices[0] < min_price {
                prices[0]
            } else {
                min_price
            };

            helper(&prices[1..], new_min, new_max)
        }

        helper(&prices, prices[0], 0)
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let output = 5;
        assert_eq!(Solution::max_profit(input), output);
    }

    #[test]
    fn test2() {
        let input = vec![7, 6, 4, 3, 1];
        let output = 0;
        assert_eq!(Solution::max_profit(input), output);
    }

    #[test]
    fn test3() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let output = 5;
        assert_eq!(Solution::max_profit_recursive(input), output);
    }

    #[test]
    fn test4() {
        let input = vec![7, 6, 4, 3, 1];
        let output = 0;
        assert_eq!(Solution::max_profit_recursive(input), output);
    }

    #[test]
    fn test5() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let output = 5;
        assert_eq!(Solution::max_profit_recursive2(input), output);
    }

    #[test]
    fn test6() {
        let input = vec![7, 6, 4, 3, 1];
        let output = 0;
        assert_eq!(Solution::max_profit_recursive2(input), output);
    }
}
