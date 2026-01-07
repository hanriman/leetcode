fn main() {}

struct Solution {}

impl Solution {
    // time complexity: O(n)
    // space complexity: O(1)
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate: i32 = 0;
        let mut cnt: i32 = 0;

        for n in nums {
            if cnt == 0 {
                candidate = n;
            }

            if n == candidate {
                cnt += 1;
            } else {
                cnt -= 1
            }
        }

        candidate
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = vec![3, 2, 3];
        let output = 3;
        assert_eq!(Solution::majority_element(input), output);
    }

    #[test]
    fn test2() {
        let input = vec![2, 2, 1, 1, 1, 2, 2];
        let output = 2;
        assert_eq!(Solution::majority_element(input), output);
    }
}
