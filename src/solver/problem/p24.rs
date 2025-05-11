use crate::solver::ListNode;

pub struct P24;

impl P24 {
    pub fn solve_v1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut cur = dummy.as_mut().unwrap();
        while cur.next.is_some() && cur.next.as_ref().unwrap().next.is_some() {
            cur.next = {
                let mut b = cur.next.as_mut().unwrap().next.take();
                cur.next.as_mut().unwrap().next = b.as_mut().unwrap().next.take();
                let a = cur.next.take();
                b.as_mut().unwrap().next = a;
                b
            };
            cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        dummy.unwrap().next
    }

    pub fn solve_v2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        let mut current = &mut dummy;

        loop {
            let mut first = match current.next.take() {
                Some(node) => node,
                None => break,
            };

            let mut second = match first.next.take() {
                Some(node) => node,
                None => {
                    current.next = Some(first);
                    break;
                }
            };

            let rest = second.next.take();

            first.next = rest;
            second.next = Some(first);

            current.next = Some(second);

            if let Some(node) = &mut current.next {
                if let Some(next_node) = &mut node.next {
                    current = next_node;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        dummy.next
    }
}
