fn main() {}

struct Solution {}

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut sorted_happiness: Vec<i32> = happiness.clone();
        sorted_happiness.sort_unstable_by(|a, b| b.cmp(a));
        let mut cnt_happiness: i64 = 0;

        for i in 0..k {
            sorted_happiness[i as usize] -= i;
            if sorted_happiness[i as usize] <= 0 {
                return cnt_happiness;
            }
            cnt_happiness += sorted_happiness[i as usize] as i64;
        }

        cnt_happiness
    }

    pub fn maximum_happiness_sum_recursive(happiness: Vec<i32>, k: i32) -> i64 {
        fn helper(i: usize, k: usize, sorted_happiness: &mut Vec<i32>, cnt_happiness: &mut i64) {
            if i == k || i >= sorted_happiness.len() {
                return;
            }

            let value: i32 = sorted_happiness[i] - i as i32;
            if value <= 0 {
                return;
            }

            *cnt_happiness += value as i64;

            helper(i + 1, k, sorted_happiness, cnt_happiness);
        }

        let mut sorted_happiness: Vec<i32> = happiness.clone();
        sorted_happiness.sort_unstable_by(|a, b| b.cmp(a));
        let mut cnt_happiness: i64 = 0;

        helper(0, k as usize, &mut sorted_happiness, &mut cnt_happiness);
        cnt_happiness
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn maximum_happiness_sum_test() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        assert_eq!(Solution::maximum_happiness_sum(happiness, k), 4);
    }

    #[test]
    fn maximum_happiness_sum_test2() {
        let happiness = vec![1, 1, 1];
        let k = 2;
        assert_eq!(Solution::maximum_happiness_sum(happiness, k), 1);
    }

    #[test]
    fn maximum_happiness_sum_test3() {
        let happiness = vec![2, 3, 4, 5];
        let k = 1;
        assert_eq!(Solution::maximum_happiness_sum(happiness, k), 5);
    }

    #[test]
    fn maximum_happiness_sum_recursive_test() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        assert_eq!(Solution::maximum_happiness_sum_recursive(happiness, k), 4);
    }

    #[test]
    fn maximum_happiness_sum_recursive_test2() {
        let happiness = vec![1, 1, 1];
        let k = 2;
        assert_eq!(Solution::maximum_happiness_sum_recursive(happiness, k), 1);
    }

    #[test]
    fn maximum_happiness_sum_recursive_test3() {
        let happiness = vec![2, 3, 4, 5];
        let k = 1;
        assert_eq!(Solution::maximum_happiness_sum_recursive(happiness, k), 5);
    }
}
