fn main() {
    let n = 30;

    println!("{:?}", Solution::fizz_buzz(n));
}

struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        for i in 1..n + 1 {
            if i % 3 == 0 && i % 5 == 0 {
                result.push("FizzBuzz".to_string());
            } else if i % 5 == 0 {
                result.push("Buzz".to_string());
            } else if i % 3 == 0 {
                result.push("Fizz".to_string());
            } else {
                result.push(i.to_string());
            }
        }

        return result;
    }

    pub fn fizz_buzz_recursive(n: i32) -> Vec<String> {
        fn helper(i: i32, n: i32, acc: &mut Vec<String>) {
            if i > n {
                return;
            }

            match i {
                i if i % 15 == 0 => {
                    acc.push("FizzBuzz".to_string());
                }
                i if i % 5 == 0 => {
                    acc.push("Buzz".to_string());
                }
                i if i % 3 == 0 => {
                    acc.push("Fizz".to_string());
                }
                _ => {
                    acc.push(i.to_string());
                }
            }

            helper(i + 1, n, acc);
        }

        let mut res: Vec<String> = Vec::with_capacity(n as usize);
        helper(1, n, &mut res);
        res
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn fizz_buzz_test() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }

    #[test]
    fn fizz_buzz_recursive_test() {
        assert_eq!(Solution::fizz_buzz_recursive(3), vec!["1", "2", "Fizz"]);
        assert_eq!(
            Solution::fizz_buzz_recursive(5),
            vec!["1", "2", "Fizz", "4", "Buzz"]
        );
        assert_eq!(
            Solution::fizz_buzz_recursive(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
