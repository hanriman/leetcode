fn main() {
    println!("todo")
}

struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn find_pivot_test() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6], 3));
        assert_eq!(Solution::pivot_index(vec![1, 2, 3], -1));
        assert_eq!(Solution::pivot_index(vec![2, 1, -1], 0));
    }
}
