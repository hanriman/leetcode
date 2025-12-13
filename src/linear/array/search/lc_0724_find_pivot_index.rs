struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut right: i32 = nums.iter().sum();
        let mut left: i32 = 0;

        for i in 0..nums.len() {
            right -= nums[i];

            if right == left {
                return i as i32;
            }

            left += nums[i];
        }

        -1
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn pivot_index_test() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(Solution::pivot_index(nums), 3)
    }

    #[test]
    fn pivot_index_test2() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::pivot_index(nums), -1)
    }

    #[test]
    fn pivot_index_test3() {
        let nums = vec![2, 1, -1];
        assert_eq!(Solution::pivot_index(nums), 0)
    }
}
