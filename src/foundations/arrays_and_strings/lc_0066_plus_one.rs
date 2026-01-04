fn main() {}

struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits_plus_one: Vec<i32> = digits.clone();
        let last_idx: usize = digits_plus_one.len();

        for idx in (0..last_idx).rev() {
            if digits_plus_one[idx] != 9 {
                digits_plus_one[idx] += 1;
                return digits_plus_one;
            }

            digits_plus_one[idx] = 0;
        }

        digits_plus_one.insert(0, 1);
        digits_plus_one
    }

    pub fn plus_one_recursive(digits: Vec<i32>) -> Vec<i32> {
        fn helper(digits: &mut Vec<i32>, idx: isize) {
            if idx < 0 {
                digits.insert(0, 1);
                return;
            }

            if digits[idx as usize] < 9 {
                digits[idx as usize] += 1;
                return;
            }

            digits[idx as usize] = 0;
            helper(digits, idx - 1)
        }

        let mut digits_plus_one: Vec<i32> = digits.clone();
        let last_idx: isize = digits_plus_one.len() as isize - 1;

        helper(&mut digits_plus_one, last_idx);
        digits_plus_one
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn plus_one_test() {
        let digits: Vec<i32> = vec![1, 2, 3];
        let res: Vec<i32> = vec![1, 2, 4];
        assert_eq!(Solution::plus_one(digits), res);

        let digits: Vec<i32> = vec![4, 3, 2, 1];
        let res: Vec<i32> = vec![4, 3, 2, 2];
        assert_eq!(Solution::plus_one(digits), res);

        let digits: Vec<i32> = vec![9];
        let res: Vec<i32> = vec![1, 0];
        assert_eq!(Solution::plus_one(digits), res);
    }

    #[test]
    fn plus_one_recursive_test() {
        let digits: Vec<i32> = vec![1, 2, 3];
        let res: Vec<i32> = vec![1, 2, 4];
        assert_eq!(Solution::plus_one_recursive(digits), res);

        let digits: Vec<i32> = vec![4, 3, 2, 1];
        let res: Vec<i32> = vec![4, 3, 2, 2];
        assert_eq!(Solution::plus_one_recursive(digits), res);

        let digits: Vec<i32> = vec![9];
        let res: Vec<i32> = vec![1, 0];
        assert_eq!(Solution::plus_one_recursive(digits), res);
    }
}
