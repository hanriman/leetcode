fn main() {}

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // time complexity: O(n.m)
        // space complexity: O(n)
        if strs.len() == 0 {
            return "".to_string();
        }

        let first: &str = &strs[0];

        for (i, c) in first.chars().enumerate() {
            for s in &strs {
                if Some(c) != s.chars().nth(i) {
                    return first[..i].to_string();
                }
            }
        }

        first.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let strs: Vec<String> = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let res: String = "fl".to_string();
        assert_eq!(Solution::longest_common_prefix(strs), res);
    }

    #[test]
    fn test2() {
        let strs: Vec<String> = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let res: String = "".to_string();
        assert_eq!(Solution::longest_common_prefix(strs), res);
    }

    #[test]
    fn test3() {
        let strs: Vec<String> = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let res: String = "fl".to_string();
        assert_eq!(Solution::longest_common_prefix(strs), res);
    }

    #[test]
    fn test4() {
        let strs: Vec<String> = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let res: String = "".to_string();
        assert_eq!(Solution::longest_common_prefix(strs), res);
    }
}
