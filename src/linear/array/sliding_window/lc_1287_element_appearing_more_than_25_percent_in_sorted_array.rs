struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let cnt_percent: usize = (arr.len() as f32 * 0.25) as usize;

        for i in 0..(arr.len() - cnt_percent) {
            if arr[i] == arr[i + cnt_percent] {
                res = arr[i];
            }
        }

        res
    }
}

fn main() {
    let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];

    println!("{}", Solution::find_special_integer(arr));
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn find_special_integer_test() {
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
        assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
    }
}
