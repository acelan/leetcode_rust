pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums: Vec<i32> = Vec::new();
        let ret: f64;
        for i in nums1 {
            nums.push(i);
        }
        for i in nums2 {
            nums.push(i);
        }
        nums.sort();

        if nums.len() % 2 == 0 {
            ret = (f64::from(nums[nums.len()/2 - 1]) + f64::from(nums[nums.len()/2])) / 2.0;
        } else {
            ret = nums[nums.len()/2].into();
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let testcases: Vec<(Vec<i32>, Vec<i32>, f64)> = vec![
            (
                vec![1, 3],
                vec![2],
                2.0,
            ),
            (
                vec![1, 3],
                vec![2, 4],
                2.50000,
            ),
        ];
        for case in testcases {
            assert_eq!(Solution::find_median_sorted_arrays(case.0, case.1), case.2);
        }
    }
}
