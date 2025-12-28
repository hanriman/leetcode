fn main() {}

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut cnt: usize = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[cnt] = nums[i];
                cnt += 1;
            }
        }

        cnt as i32
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn remove_element_test() {
        let mut nums: Vec<i32> = vec![3, 2, 2, 3];
        let val: i32 = 3;
        let actual_length = 2;
        assert_eq!(Solution::remove_element(&mut nums, val), actual_length);
        let actual_num: Vec<i32> = vec![2, 2, 3, 3];
        for i in 0..actual_length {
            assert_eq!(nums[i as usize], actual_num[i as usize]);
        }
    }

    #[test]
    fn remove_element_test2() {
        let mut nums: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val: i32 = 2;
        let actual_length = 5;
        assert_eq!(Solution::remove_element(&mut nums, val), actual_length);
        let actual_num: Vec<i32> = vec![0, 1, 3, 0, 4, 2, 2, 2];
        for i in 0..actual_length {
            assert_eq!(nums[i as usize], actual_num[i as usize]);
        }
    }
}
