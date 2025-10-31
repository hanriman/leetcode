struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m_idx = m as usize - 1;
        let mut n_idx = n as usize - 1;
        let mut right_idx = (m + n) as usize - 1;

        while n_idx >= 0 {
            if m_idx >= 0 && nums1[m_idx] > nums2[n_idx] {
                nums1[right_idx] = nums1[m_idx];
                if m_idx == 0 {
                    break;
                }
                m_idx -= 1;
            } else {
                nums1[right_idx] = nums2[n_idx];
                if n_idx == 0 {
                    break;
                }
                n_idx -= 1;
            }

            right_idx -= 1;
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;

    Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn basic_test() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn emtpy_nums1() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn emtpy_nums2() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }
}
