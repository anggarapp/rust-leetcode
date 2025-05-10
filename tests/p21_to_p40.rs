use rust_solve_leetcode::solver::*;

#[test]
fn test_p21_merge_two_lists() {
    let node_one = Some(Box::new(ListNode { val: 4, next: None }));
    let node_one = Some(Box::new(ListNode {
        val: 2,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 1,
        next: node_one,
    }));

    let node_two = Some(Box::new(ListNode { val: 4, next: None }));
    let node_two = Some(Box::new(ListNode {
        val: 3,
        next: node_two,
    }));
    let node_two = Some(Box::new(ListNode {
        val: 1,
        next: node_two,
    }));

    let node_result = Some(Box::new(ListNode { val: 4, next: None }));
    let node_result = Some(Box::new(ListNode {
        val: 4,
        next: node_result,
    }));
    let node_result = Some(Box::new(ListNode {
        val: 3,
        next: node_result,
    }));
    let node_result = Some(Box::new(ListNode {
        val: 2,
        next: node_result,
    }));
    let node_result = Some(Box::new(ListNode {
        val: 1,
        next: node_result,
    }));
    let node_result = Some(Box::new(ListNode {
        val: 1,
        next: node_result,
    }));
    assert_eq!(P21::solve_v1(node_one, node_two), node_result);

    let node_one = Some(Box::new(ListNode { val: 4, next: None }));
    let node_one = Some(Box::new(ListNode {
        val: 2,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 1,
        next: node_one,
    }));

    let node_two = Some(Box::new(ListNode { val: 4, next: None }));
    let node_two = Some(Box::new(ListNode {
        val: 3,
        next: node_two,
    }));
    let node_two = Some(Box::new(ListNode {
        val: 1,
        next: node_two,
    }));

    let node_result = Some(Box::new(ListNode { val: 4, next: None }));
    let node_result = Some(Box::new(ListNode {
        val: 4,
        next: node_result,
    }));
    let node_result = Some(Box::new(ListNode {
        val: 3,
        next: node_result,
    }));
    let node_result = Some(Box::new(ListNode {
        val: 2,
        next: node_result,
    }));
    let node_result = Some(Box::new(ListNode {
        val: 1,
        next: node_result,
    }));
    let node_result = Some(Box::new(ListNode {
        val: 1,
        next: node_result,
    }));
    assert_eq!(P21::solve_v2(node_one, node_two), node_result);
}

#[test]
fn test_p22_generate_parenthesis() {
    assert_eq!(
        P22::solve_v1(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
    assert_eq!(P22::solve_v1(1), vec!["()"]);
}
