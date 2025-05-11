use crate::solver::ListNode;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub struct P23;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val).reverse()
    }
}

impl P23 {
    pub fn solve_v1(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut pq = lists
            .into_iter()
            .filter_map(|head| head)
            .collect::<BinaryHeap<_>>();
        let mut head = None;
        let mut cur = &mut head;
        while let Some(node) = pq.pop() {
            cur = &mut cur.insert(Box::new(ListNode::new(node.val))).next;
            if let Some(next) = node.next {
                pq.push(next);
            }
        }
        head
    }
    pub fn solve_v2(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        let mut _head = Some(Box::new(ListNode::new(0)));
        for mut list in lists {
            _head = list.take();

            while let Some(node) = _head {
                heap.push(Reverse(node.val));
                _head = node.next;
            }
        }

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut cur = dummy.as_mut();

        while let Some(Reverse(val)) = heap.pop() {
            if let Some(node) = cur {
                node.next = Some(Box::new(ListNode::new(val)));
                cur = node.next.as_mut();
            }
        }

        dummy.unwrap().next
    }

    pub fn solve_v3(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut lists = lists;
        let mut dummmy = ListNode::new(0);
        let mut curr = &mut dummmy;
        loop {
            let mut min_val = i32::MAX;
            let mut min_index = None;

            for (i, list) in lists.iter_mut().enumerate() {
                if let Some(node) = list {
                    if node.val < min_val {
                        min_val = node.val;
                        min_index = Some(i);
                    }
                }
            }

            if min_index == None {
                break;
            }

            let i = min_index.unwrap();
            let mut node = lists[i].take().unwrap();
            let next = node.next.take();
            lists[i] = next;

            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
        }

        dummmy.next
    }
}
