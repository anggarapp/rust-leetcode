use crate::solver::ListNode;

pub struct P19;

impl P19 {
    pub fn solve_v1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow = &mut dummy;
        let mut fast = &slow.clone();
        for _ in 0..=n {
            fast = &fast.as_ref().unwrap().next;
        }

        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
        dummy.unwrap().next
        // Some(Box::new(ListNode { val: 0, next: head }))
    }
    pub fn solve_v2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode { val: 0, next: head };
        // maintain 2 pointers curr and lag, where lag is n nodes behind
        let mut curr = dummy.clone();
        let mut lag = &mut dummy;
        // advance curr by n+1 nodes (1 more because we need the node before the nth node)
        for _ in 0..n {
            curr = *(curr.next.unwrap());
        }
        // advance both pointers until curr is at the end
        while curr.next.is_some() {
            curr = *(curr.next.unwrap());
            lag = lag.next.as_mut().unwrap();
        }
        // remove the nth node
        lag.next = lag.next.as_mut().unwrap().next.clone();
        dummy.next
    }
}
