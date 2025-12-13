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
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn find_numbers_test1() {
        let nums = vec![12, 345, 2, 6, 7896];
        assert_eq!(Solution::find_numbers(nums), 2);
    }

    #[test]
    fn find_numbers_test2() {
        let nums = vec![555, 901, 482, 1771];
        assert_eq!(Solution::find_numbers(nums), 1);
    }

    #[test]
    fn find_numbers_test3() {
        let nums = vec![];
        assert_eq!(Solution::find_numbers(nums), 0);
    }
}
