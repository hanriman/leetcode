use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let allowed_chars: HashSet<char> =
            "abcdefghijklmnopqrstuvwxyz0123456789_".chars().collect();
        let allowed_business_line: HashSet<&str> =
            ["electronics", "grocery", "pharmacy", "restaurant"]
                .iter()
                .cloned()
                .collect();
        let mut valid: Vec<(String, String)> = Vec::new();

        for i in 0..code.len() {
            if code[i] == "" {
                continue;
            }
            let mut is_coupon_valid = true;
            for ch in code[i].chars() {
                if !allowed_chars.contains(&ch.to_ascii_lowercase()) {
                    is_coupon_valid = false;
                    break;
                }
            }
            if !is_coupon_valid {
                continue;
            }
            if !allowed_business_line.contains(business_line[i].as_str()) {
                continue;
            }
            if !is_active[i] {
                continue;
            }

            valid.push((business_line[i].clone(), code[i].clone()));
        }

        valid.sort();

        valid.into_iter().map(|(_, coupon)| coupon).collect()
    }
}

fn main() {
    let code = vec!["SAVE20", "", "PHARMA5", "SAVE@20"];
    let business_line = vec!["restaurant", "grocery", "pharmacy", "restaurant"];
    let is_active = vec![true, true, true, true];

    assert_eq!(
        Solution::validate_coupons(code, business_line, is_active),
        vec!["PHARMA5", "SAVE20"]
    );
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn coupon_test() {
        let code = vec![
            "SAVE20".to_string(),
            "".to_string(),
            "PHARMA5".to_string(),
            "SAVE@20".to_string(),
        ];
        let business_line = vec![
            "restaurant".to_string(),
            "grocery".to_string(),
            "pharmacy".to_string(),
            "restaurant".to_string(),
        ];
        let is_active = vec![true, true, true, true];

        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            vec!["PHARMA5", "SAVE20"]
        );

        let code2 = vec!["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"];
        let business_line2 = vec!["grocery", "electronics", "invalid"];
        let is_active2 = vec![false, true, true];
        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            vec!["ELECTRONICS_50"]
        );
    }
}
