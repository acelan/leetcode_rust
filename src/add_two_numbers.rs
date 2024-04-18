// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans_nums: Vec<i32> = Vec::new();
        let mut carry = 0;

        let p1 = Solution::convert_to_vec(&l1);
        let p2 = Solution::convert_to_vec(&l2);
        let mut idx = 0;

        while (p1.len() > 0 || p2.len() > 0) && std::cmp::max(p1.len(), p2.len()) > idx {
            let x = p1.get(idx).map_or(0, |&v| v);
            let y = p2.get(idx).map_or(0, |&v| v);
            //println!("idx = {}: x = {}, y = {}", idx, x,y );

            let sum = carry + x + y;
            carry = sum / 10;
            ans_nums.push(sum % 10);
            idx = idx + 1;
        }

        if carry != 0 {
            ans_nums.push(carry);
        }

        Solution::build_list_node(&ans_nums)
    }

    pub fn convert_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32>{
        let mut ret = Vec::new();
        let mut node = head;
        while let Some(ref v) = *node {
            ret.push(v.val);
            node = &v.next;
        }
        ret
    }

    pub fn build_list_node(nums: &Vec<i32>) -> Option<Box<ListNode>> {
        let len = nums.len();
        if len == 0 { return None}

        let mut res = Some(Box::new(ListNode::new(nums[0])));
        let mut cur: &mut ListNode = &mut *res.as_mut().unwrap();
        for i in 1..len {
            (*cur).next = Some(Box::new(ListNode::new(nums[i])));
            //cur = &mut <Option<Box<ListNode>> as Clone>::clone(&(*cur).next)? as *mut Box<ListNode>;
            //cur = &mut (*cur).next as *mut ListNode;
            cur = (*cur).next.as_mut().unwrap();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let cases = vec![
            (
                Solution::build_list_node(&vec![2,4,3]),
                Solution::build_list_node(&vec![7,8,9]),
                0,
                Solution::build_list_node(&vec![9,2,3,1])
            ),
            (
                Solution::build_list_node(&vec![9,9,9,9,9,9,9]),
                Solution::build_list_node(&vec![9,9,9,9]),
                0,
                Solution::build_list_node(&vec![8,9,9,9,0,0,0,1])
            ),
            (
                Solution::build_list_node(&vec!()),
                Solution::build_list_node(&vec![0]),
                0,
                Solution::build_list_node(&vec!(0))
            )
        ];
        for case in cases {
            assert_eq!(Solution::add_two_numbers(case.0, case.1), case.3);
        }
    }
}
