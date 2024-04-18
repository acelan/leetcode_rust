// for solution2
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans_nums = Vec::new();

        // solution 1 - brute force
        /*
        // iterate nums from the first element
        for i in 0..nums.len() {
            // iterate nums from the next element of `i`
            for  j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    ans_nums.push(i as i32);
                    ans_nums.push(j as i32);
                    return ans_nums;
                }
            }
        }
        */

        // solution 2
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            num_map.insert(nums[i], i as i32);
        }

        for i in 0..nums.len() {
            let diff = target - nums[i];
            match num_map.get(&diff) {
                Some(&index) => {
                    if index != i as i32 {
                        ans_nums.push((index as i32).clone());
                        ans_nums.push(i as i32);
                        return ans_nums;
                    }
                }
                None => {}
            }
        }

        return ans_nums;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let testcases = vec![
            (
                vec![2, 7, 11, 15],
                9,
                vec![0, 1]
            ),
            (
                vec![3, 2, 4],
                6,
                vec![1, 2]
            ),
            (
                vec![3, 3],
                6,
                vec![0, 1]
            )
        ];
        for mut case in testcases {
            assert_eq!(Solution::two_sum(case.0, case.1).sort(), case.2.sort());
        }
    }
}

