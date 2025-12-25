fn main() {}

struct Solution {}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max_num: i32 = i32::MIN;
        let mut max_num_idx: usize = 0;
        let mut sec_max_num: i32 = i32::MIN;

        for i in 0..nums.len() {
            if nums[i] > max_num {
                sec_max_num = max_num;
                max_num = nums[i];
                max_num_idx = i;
            }

            if nums[i] > sec_max_num && i != max_num_idx {
                sec_max_num = nums[i];
            }
        }

        if max_num >= sec_max_num * 2 {
            max_num_idx as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn dominant_index() {
        let nums = [3, 6, 1, 0].to_vec();
        assert_eq!(Solution::dominant_index(nums), 1);
    }

    #[test]
    fn dominant_index2() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(Solution::dominant_index(nums), -1);
    }
}
