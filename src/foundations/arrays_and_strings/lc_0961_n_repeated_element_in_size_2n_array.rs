use std::collections::HashSet;

fn main() {}

struct Solution {}

impl Solution {
    // time complexity: O(n)
    // space complexity: O(n)
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set_nums: HashSet<i32> = HashSet::new();

        for n in nums {
            if !set_nums.insert(n) {
                return n;
            }
        }

        -1
    }

    // time complexity: O(n)
    // space complexity: O(n)
    pub fn repeated_n_times_recursive(nums: Vec<i32>) -> i32 {
        fn helper(nums: &[i32], idx: usize, seen: &mut HashSet<i32>) -> i32 {
            if idx >= nums.len() {
                return -1;
            }

            if !seen.insert(nums[idx]) {
                return nums[idx];
            }

            helper(&nums, idx + 1, seen)
        }

        let mut seen: HashSet<i32> = HashSet::new();
        helper(&nums, 0, &mut seen)
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums: Vec<i32> = vec![1, 2, 3, 3];
        assert_eq!(Solution::repeated_n_times(nums), 3);
    }

    #[test]
    fn test2() {
        let nums: Vec<i32> = vec![2, 1, 2, 5, 3, 2];
        assert_eq!(Solution::repeated_n_times(nums), 2);
    }

    #[test]
    fn test3() {
        let nums: Vec<i32> = vec![5, 1, 5, 2, 5, 3, 5, 4];
        assert_eq!(Solution::repeated_n_times(nums), 5);
    }

    #[test]
    fn test4() {
        let nums: Vec<i32> = vec![1, 2, 3, 3];
        assert_eq!(Solution::repeated_n_times_recursive(nums), 3);
    }

    #[test]
    fn test5() {
        let nums: Vec<i32> = vec![2, 1, 2, 5, 3, 2];
        assert_eq!(Solution::repeated_n_times_recursive(nums), 2);
    }

    #[test]
    fn test6() {
        let nums: Vec<i32> = vec![5, 1, 5, 2, 5, 3, 5, 4];
        assert_eq!(Solution::repeated_n_times_recursive(nums), 5);
    }
}
