use std::collections::HashMap;

fn main() {}

struct Solution {}

impl Solution {
    // time complexity: O(n)
    // space complexity: O(1)
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }

        let mut prev: i32 = 0;
        let mut curr: i32 = 1;

        for _ in 0..n - 1 {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }

        curr
    }

    // time complexity: O(2^n)
    // space complexity: O(n)
    pub fn fib_recursive(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n < 3 {
            return 1;
        }

        Self::fib(n - 1) + Self::fib(n - 2)
    }

    // time complexity: O(n)
    // space complexity: O(n)
    pub fn fib_memoization(n: i32) -> i32 {
        fn helper(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if n <= 1 {
                return n;
            }

            if let Some(&val) = memo.get(&n) {
                return val;
            }

            let res = helper(n - 1, memo) + helper(n - 2, memo);
            memo.insert(n, res);
            res
        }

        let mut memo = HashMap::new();
        helper(n, &mut memo)
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let input1 = 2;
        let output = 1;
        assert_eq!(Solution::fib(input1), output);
    }

    #[test]
    fn test2() {
        let input1 = 3;
        let output = 2;
        assert_eq!(Solution::fib(input1), output);
    }

    #[test]
    fn test3() {
        let input1 = 4;
        let output = 3;
        assert_eq!(Solution::fib(input1), output);
    }

    #[test]
    fn test4() {
        let input1 = 2;
        let output = 1;
        assert_eq!(Solution::fib_recursive(input1), output);
    }

    #[test]
    fn test5() {
        let input1 = 3;
        let output = 2;
        assert_eq!(Solution::fib_recursive(input1), output);
    }

    #[test]
    fn test6() {
        let input1 = 4;
        let output = 3;
        assert_eq!(Solution::fib_recursive(input1), output);
    }

    #[test]
    fn test7() {
        let input1 = 2;
        let output = 1;
        assert_eq!(Solution::fib_memoization(input1), output);
    }

    #[test]
    fn test8() {
        let input1 = 3;
        let output = 2;
        assert_eq!(Solution::fib_memoization(input1), output);
    }

    #[test]
    fn test9() {
        let input1 = 4;
        let output = 3;
        assert_eq!(Solution::fib_memoization(input1), output);
    }
}
