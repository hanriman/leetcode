fn main() {}

struct Solution {}

impl Solution {
    // time complexity:
    // space complexity:
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut idx: usize = 0;

        for c in t.chars() {
            if idx < s_chars.len() && s_chars[idx] == c {
                idx += 1;
            }
        }

        idx == s_chars.len()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = "abc".to_string();
        let input2 = "ahbgdc".to_string();
        let output = true;
        assert_eq!(Solution::is_subsequence(input, input2), output);
    }

    #[test]
    fn test2() {
        let input = "axc".to_string();
        let input2 = "ahbgdc".to_string();
        let output = false;
        assert_eq!(Solution::is_subsequence(input, input2), output);
    }
}
