fn main() {}

struct Solution {}

impl Solution {
    // time complexity:
    // space complexity:
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut zero_idx: usize = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(zero_idx, i);
                zero_idx += 1;
            }
        }
    }

    // time complexity:
    // space complexity:
    pub fn move_zeroes_recursive(nums: &mut Vec<i32>) {
        fn helper(nums: &mut Vec<i32>, idx: usize, zero_idx: usize) {
            if idx == nums.len() {
                return;
            }

            if nums[idx] != 0 {
                nums.swap(zero_idx, idx);
                helper(nums, idx + 1, zero_idx + 1);
            } else {
                helper(nums, idx + 1, zero_idx);
            }
        }

        helper(nums, 0, 0);
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let mut input = vec![0, 1, 0, 3, 12];
        let output = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn test2() {
        let mut input = vec![0];
        let output = vec![0];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn test3() {
        let mut input = vec![0, 1, 0, 3, 12];
        let output = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes_recursive(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn test4() {
        let mut input = vec![0];
        let output = vec![0];
        Solution::move_zeroes_recursive(&mut input);
        assert_eq!(input, output);
    }
}
