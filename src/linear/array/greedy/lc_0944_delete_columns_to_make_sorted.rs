fn main() {
    let strs: Vec<String> = vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()];
    let res = assert_eq!(Solution::min_deletion_size(strs), 1);
    println!("{:?}", res);
}

struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut delete_count: i32 = 0;
        let n_col: usize = strs[0].len();
        let n_row: usize = strs.len();

        for c in 0..n_col {
            for s in 0..n_row - 1 {
                if strs[s].as_bytes()[c] > strs[s + 1].as_bytes()[c] {
                    delete_count += 1;
                    break;
                }
            }
        }

        delete_count
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn min_deletion_size_test1() {
        let strs: Vec<String> = vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()];
        assert_eq!(Solution::min_deletion_size(strs), 1);
    }

    #[test]
    fn min_deletion_size_test2() {
        let strs: Vec<String> = vec!["a".to_string(), "b".to_string()];
        assert_eq!(Solution::min_deletion_size(strs), 0);
    }

    #[test]
    fn min_deletion_size_test3() {
        let strs: Vec<String> = vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()];
        assert_eq!(Solution::min_deletion_size(strs), 3);
    }
}
