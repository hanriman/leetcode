fn main() {}

struct Solution {}

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut max_score: i32 = 0;
        let mut score: i32 = 0;
        let mut best_hour: i32 = -1;

        for (i, val) in customers.chars().enumerate() {
            if val == 'Y' {
                score += 1;
            } else if val == 'N' {
                score -= 1;
            }

            if score > max_score {
                max_score = score;
                best_hour = i as i32;
            }
        }

        return best_hour + 1;
    }

    pub fn best_closing_time_recursive(customers: String) -> i32 {
        fn helper(
            customers_chars: &[u8],
            idx: usize,
            max_score: i32,
            best_hour: i32,
            temp: i32,
        ) -> i32 {
            if idx == customers_chars.len() {
                return best_hour;
            }

            let temp: i32 = if customers_chars[idx] == b'Y' {
                temp + 1
            } else {
                temp - 1
            };
            let (max_score, best_hour) = if temp > max_score {
                (temp, idx as i32 + 1)
            } else {
                (max_score, best_hour)
            };

            helper(customers_chars, idx + 1, max_score, best_hour, temp)
        }

        let best_idx = helper(customers.as_bytes(), 0, 0, 0, 0);

        best_idx
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn best_closing_time_test() {
        let customers = String::from("YYNY");
        assert_eq!(Solution::best_closing_time(customers), 2);

        let customers2 = String::from("NNNNN");
        assert_eq!(Solution::best_closing_time(customers2), 0);

        let customers3 = String::from("YYYY");
        assert_eq!(Solution::best_closing_time(customers3), 4);
    }

    #[test]
    fn best_closing_time_test2() {
        let customers = String::from("YYNY");
        assert_eq!(Solution::best_closing_time_recursive(customers), 2);

        let customers2 = String::from("NNNNN");
        assert_eq!(Solution::best_closing_time_recursive(customers2), 0);

        let customers3 = String::from("YYYY");
        assert_eq!(Solution::best_closing_time_recursive(customers3), 4);
    }
}
