use crate::solver::ListNode;

pub struct P25;

impl P25 {
    pub fn solve_v1(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut head = head;
            let mut pre = None;

            while let Some(mut node) = head {
                head = node.next.take();
                node.next = pre.take();
                pre = Some(node);
            }
            pre
        }

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut pre = &mut dummy;
        let mut cur = head;
        while cur.is_some() {
            let mut q = &mut cur;
            for _ in 0..k - 1 {
                if q.is_none() {
                    break;
                }
                q = &mut q.as_mut().unwrap().next;
            }
            if q.is_none() {
                pre.as_mut().unwrap().next = cur;
                return dummy.unwrap().next;
            }
            let b = q.as_mut().unwrap().next.take();
            pre.as_mut().unwrap().next = reverse(cur);
            while pre.is_some() && pre.as_mut().unwrap().next.is_some() {
                pre = &mut pre.as_mut().unwrap().next;
            }
            cur = b;
        }
        dummy.unwrap().next
    }
    pub fn solve_v2(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut pre = &mut dummy;

        let mut stack = vec![];

        let mut cnt = 0;
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next;
            node.next = None;
            stack.push(Some(node));
            cnt += 1;

            if cnt == k {
                while let Some(node) = stack.pop() {
                    pre.next = node;
                    pre = pre.next.as_mut().unwrap();
                }

                cnt = 0;
            }
        }

        let mut p = None;
        while let Some(Some(mut node)) = stack.pop() {
            node.next = p;
            p = Some(node);
        }

        pre.next = p;

        dummy.next
    }
}
