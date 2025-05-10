use crate::solver::ListNode;
pub struct P21;
impl P21 {
    pub fn solve_v1(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(list), None) => Some(list),
            (None, Some(list)) => Some(list),
            (Some(mut list1), Some(mut list2)) => {
                if list1.val < list2.val {
                    list1.next = Self::solve_v1(list1.next, Some(list2));
                    Some(list1)
                } else {
                    list2.next = Self::solve_v1(Some(list1), list2.next);
                    Some(list2)
                }
            }
        }
    }
    pub fn solve_v2(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(i32::MIN));
        let mut curr = &mut dummy;
        let mut list1 = list1;
        let mut list2 = list2;
        while list1.is_some() && list2.is_some() {
            let node1 = list1.as_ref().unwrap();
            let node2 = list2.as_ref().unwrap();
            if node1.val < node2.val {
                curr.next = list1;
                curr = curr.next.as_mut().unwrap();
                list1 = curr.next.take();
            } else {
                curr.next = list2;
                curr = curr.next.as_mut().unwrap();
                list2 = curr.next.take();
            }
        }

        curr.next = list1.or(list2);
        dummy.next
    }
}
