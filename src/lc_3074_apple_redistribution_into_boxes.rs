fn main() {}

struct Solution {}

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let total_apples: i32 = apple.into_iter().sum();
        let mut sorted_capacity: Vec<i32> = capacity.clone();
        sorted_capacity.sort_unstable_by(|a, b| b.cmp(a));
        let mut curr_capacity: i32 = 0;
        let mut cnt_boxes: i32 = 0;

        for cap in sorted_capacity {
            curr_capacity += cap;
            cnt_boxes += 1;

            if curr_capacity >= total_apples {
                break;
            }
        }

        cnt_boxes
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn minimum_boxes_test() {
        let apple = vec![1, 3, 2];
        let capacity = vec![4, 3, 1, 5, 2];
        assert_eq!(Solution::minimum_boxes(apple, capacity), 2);
    }

    #[test]
    fn minimum_boxes_test2() {
        let apple = vec![5, 5, 5];
        let capacity = vec![2, 4, 2, 7];
        assert_eq!(Solution::minimum_boxes(apple, capacity), 4);
    }
}
