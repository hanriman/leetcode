fn main() {}

struct Solution {}

impl Solution {
    // time complexity: O(n)
    // space complexity: O(1)
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() <= 1 {
            return;
        }

        let mut i: usize = 0;
        let mut j: usize = s.len() - 1;

        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    // time complexity: O(n)
    // space complexity: O(1)
    pub fn reverse_string_recursive(s: &mut Vec<char>) {
        if s.len() <= 1 {
            return;
        }

        fn helper(s: &mut Vec<char>, i: usize, j: usize) {
            if i < j {
                s.swap(i, j);

                helper(s, i + 1, j - 1)
            }
        }

        let i: usize = 0;
        let j: usize = s.len() - 1;
        helper(s, i, j);
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        let output = vec!['o', 'l', 'l', 'e', 'h'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn test2() {
        let mut input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let output = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn test3() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        let output = vec!['o', 'l', 'l', 'e', 'h'];
        Solution::reverse_string_recursive(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn test4() {
        let mut input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let output = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        Solution::reverse_string_recursive(&mut input);
        assert_eq!(input, output);
    }
}
