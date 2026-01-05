fn main() {}

struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt_neg: i32 = 0;

        for row in grid {
            let mut right: usize = row.len();
            let mut left: usize = 0;

            while left < right {
                let mid: usize = left + (right - left) / 2;
                if row[mid] < 0 {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            cnt_neg += (row.len() - left) as i32;
        }

        cnt_neg
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn cont_negative_test() {
        let grid: Vec<Vec<i32>> = vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3],
        ];
        assert_eq!(Solution::count_negatives(grid), 8);
    }

    #[test]
    fn cont_negative_test2() {
        let grid: Vec<Vec<i32>> = vec![vec![3, 2], vec![1, 0]];
        assert_eq!(Solution::count_negatives(grid), 0);
    }
}
