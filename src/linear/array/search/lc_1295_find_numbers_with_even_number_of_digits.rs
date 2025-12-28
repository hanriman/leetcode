struct Solution {}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count_even: i32 = 0;

        for n in nums {
            if n.to_string().len() % 2 == 0 {
                count_even += 1;
            }
        }

        count_even
    }

    pub fn find_numbers2(nums: Vec<i32>) -> i32 {
        let mut cnt_even: i32 = 0;

        for mut num in nums {
            let mut cnt_digit: i32 = 0;

            while num > 0 {
                cnt_digit += 1;
                num /= 10;
            }

            if cnt_digit % 2 == 0 {
                cnt_even += 1;
            }
        }

        cnt_even
    }

    pub fn find_numbers_recursive(nums: Vec<i32>) -> i32 {
        fn count_digit(n: i32) -> i32 {
            if n == 0 {
                return 0;
            }

            1 + count_digit(n / 10)
        }

        fn helper(idx: usize, nums: &Vec<i32>) -> i32 {
            if idx == nums.len() {
                return 0;
            }

            let cnt_digit: i32 = count_digit(nums[idx]);
            let cnt_even: i32 = if cnt_digit % 2 == 0 { 1 } else { 0 };

            cnt_even + helper(idx + 1, nums)
        }

        helper(0, &nums)
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn find_numbers_test1() {
        let nums = vec![12, 345, 2, 6, 7896];
        assert_eq!(Solution::find_numbers(nums), 2);
        let nums = vec![555, 901, 482, 1771];
        assert_eq!(Solution::find_numbers(nums), 1);
        let nums = vec![];
        assert_eq!(Solution::find_numbers(nums), 0);
    }

    #[test]
    fn find_numbers2_test() {
        let nums = vec![12, 345, 2, 6, 7896];
        assert_eq!(Solution::find_numbers2(nums), 2);
        let nums = vec![555, 901, 482, 1771];
        assert_eq!(Solution::find_numbers2(nums), 1);
        let nums = vec![];
        assert_eq!(Solution::find_numbers2(nums), 0);
    }

    #[test]
    fn find_numbers_recursive_test() {
        let nums = vec![12, 345, 2, 6, 7896];
        assert_eq!(Solution::find_numbers_recursive(nums), 2);
        let nums = vec![555, 901, 482, 1771];
        assert_eq!(Solution::find_numbers_recursive(nums), 1);
        let nums = vec![];
        assert_eq!(Solution::find_numbers_recursive(nums), 0);
    }
}
