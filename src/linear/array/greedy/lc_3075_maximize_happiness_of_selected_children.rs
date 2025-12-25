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
}
