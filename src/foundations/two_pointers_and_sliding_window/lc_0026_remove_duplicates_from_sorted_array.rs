fn main() {}

struct Solution {}

impl Solution {
    pub fn remove_duplicate(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut prev: usize = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[prev] {
                prev += 1;
                nums[prev] = nums[i];
            }
        }

        prev as i32 + 1
    }

    pub fn remove_duplicate_recursive(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        fn helper(nums: &mut Vec<i32>, idx: usize, mut prev: usize) -> i32 {
            if idx == nums.len() {
                return prev as i32 + 1;
            }

            if nums[idx] != nums[prev] {
                prev += 1;
                nums[prev] = nums[idx];
            }

            return helper(nums, idx + 1, prev);
        }

        helper(nums, 1, 0)
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn remove_duplicate_test() {
        let mut nums: Vec<i32> = vec![1, 1, 2];
        let actual_length = 2;
        assert_eq!(Solution::remove_duplicate(&mut nums), actual_length);
        let actual_num: Vec<i32> = vec![1, 2, 1];
        for i in 0..actual_length {
            assert_eq!(nums[i as usize], actual_num[i as usize]);
        }
    }

    #[test]
    fn remove_duplicate_test2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let actual_length = 5;
        assert_eq!(Solution::remove_duplicate(&mut nums), actual_length);
        let actual_num: Vec<i32> = vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4];
        for i in 0..actual_length {
            assert_eq!(nums[i as usize], actual_num[i as usize]);
        }
    }

    #[test]
    fn remove_duplicate_recursive_test() {
        let mut nums: Vec<i32> = vec![1, 1, 2];
        let actual_length = 2;
        assert_eq!(
            Solution::remove_duplicate_recursive(&mut nums),
            actual_length
        );
        let actual_num: Vec<i32> = vec![1, 2, 1];
        for i in 0..actual_length {
            assert_eq!(nums[i as usize], actual_num[i as usize]);
        }
    }

    #[test]
    fn remove_duplicate_recursive_test2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let actual_length = 5;
        assert_eq!(
            Solution::remove_duplicate_recursive(&mut nums),
            actual_length
        );
        let actual_num: Vec<i32> = vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4];
        for i in 0..actual_length {
            assert_eq!(nums[i as usize], actual_num[i as usize]);
        }
    }
}
