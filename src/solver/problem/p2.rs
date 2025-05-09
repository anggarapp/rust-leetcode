use crate::solver::ListNode;
pub struct P2;

type List = Option<Box<ListNode>>;

pub fn to_list(vec: Vec<i32>) -> List {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

impl P2 {
    pub fn solve(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut carry = 0;
        let mut result = None;
        let mut cur = &mut result;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = &node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = &node.next;
            }

            carry = sum / 10;
            *cur = Some(Box::new(ListNode::new(sum % 10)));
            cur = &mut cur.as_mut().unwrap().next;
        }
        // println!("{:?}", result);
        result
    }
}
