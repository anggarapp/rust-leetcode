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

#[test]
fn test_p23_merge_k_lists() {
    let node_one = Some(Box::new(ListNode { val: 5, next: None }));
    let node_one = Some(Box::new(ListNode {
        val: 4,
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

    let node_three = Some(Box::new(ListNode { val: 6, next: None }));
    let node_three = Some(Box::new(ListNode {
        val: 2,
        next: node_three,
    }));

    let node_result = Some(Box::new(ListNode { val: 6, next: None }));
    let node_result = Some(Box::new(ListNode {
        val: 5,
        next: node_result,
    }));
    let node_result = Some(Box::new(ListNode {
        val: 4,
        next: node_result,
    }));
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
    assert_eq!(
        P23::solve_v1(vec![node_one, node_two, node_three]),
        node_result
    );
}

#[test]
fn test_p24_swap_pairs() {
    let node_one = Some(Box::new(ListNode { val: 4, next: None }));
    let node_one = Some(Box::new(ListNode {
        val: 3,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 2,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 1,
        next: node_one,
    }));

    let node_res = Some(Box::new(ListNode { val: 3, next: None }));
    let node_res = Some(Box::new(ListNode {
        val: 4,
        next: node_res,
    }));
    let node_res = Some(Box::new(ListNode {
        val: 1,
        next: node_res,
    }));
    let node_res = Some(Box::new(ListNode {
        val: 2,
        next: node_res,
    }));
    assert_eq!(P24::solve_v1(node_one), node_res);
}

#[test]
fn test_p25_reverse_k_group() {
    let node_one = Some(Box::new(ListNode { val: 5, next: None }));
    let node_one = Some(Box::new(ListNode {
        val: 4,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 3,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 2,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 1,
        next: node_one,
    }));

    let node_res = Some(Box::new(ListNode { val: 5, next: None }));
    let node_res = Some(Box::new(ListNode {
        val: 4,
        next: node_res,
    }));
    let node_res = Some(Box::new(ListNode {
        val: 1,
        next: node_res,
    }));
    let node_res = Some(Box::new(ListNode {
        val: 2,
        next: node_res,
    }));
    let node_res = Some(Box::new(ListNode {
        val: 3,
        next: node_res,
    }));

    assert_eq!(P25::solve_v1(node_one, 3), node_res);
}

#[test]
fn test_p26_remove_duplicates() {
    fn test(
        input_array: &mut Vec<i32>,
        expected_array: Vec<i32>,
        fnc: fn(&mut Vec<i32>) -> i32,
    ) -> bool {
        let k = fnc(input_array);
        if k != expected_array.len() as i32 {
            return false;
        }
        for i in 0..k {
            if input_array[i as usize] != expected_array[i as usize] {
                return false;
            }
        }
        true
    }
    let mut vec_one = Vec::from([1, 1, 2]);
    assert!(test(&mut vec_one, vec![1, 2], P26::solve_v1));
    let mut vec_two = Vec::from([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);
    assert!(test(&mut vec_two, vec![0, 1, 2, 3, 4], P26::solve_v1));
}

#[test]
fn test_p27_remove_element() {
    fn test(
        input_array: &mut Vec<i32>,
        expected_array: Vec<i32>,
        val_ex: i32,
        fnc: fn(&mut Vec<i32>, i32) -> i32,
    ) -> bool {
        let k = fnc(input_array, val_ex);
        if k != expected_array.len() as i32 {
            return false;
        }
        let piece = input_array.clone();
        let mut piece = piece[0..k as usize].to_vec();
        piece.sort();

        for i in 0..k {
            if expected_array[i as usize] != piece[i as usize] {
                return false;
            }
        }
        true
    }

    let mut vec_one = Vec::from([3, 2, 2, 3]);
    assert!(test(&mut vec_one, vec![2, 2], 3, P27::solve_v1));

    let mut vec_two = Vec::from([0, 1, 2, 2, 3, 0, 4, 2]);
    assert_eq!(
        test(&mut vec_two, vec![0, 0, 1, 3, 4], 2, P27::solve_v1),
        true
    );
}

#[test]
fn test_p28_str_str() {
    assert_eq!(P28::solve_v2("sadbutsad".to_string(), "sad".to_string()), 0);
    assert_eq!(
        P28::solve_v2("leetcode".to_string(), "leeto".to_string()),
        -1
    );
    assert_eq!(
        P28::solve_v2("mississippi".to_string(), "issip".to_string()),
        4
    );
    assert_eq!(P28::solve_v2("abc".to_string(), "c".to_string()), 2);
    assert_eq!(P28::solve_v2("abc".to_string(), "c".to_string()), 2);
    assert_eq!(P28::solve_v2("abb".to_string(), "abaaa".to_string()), -1);
}

#[test]
fn test_p29_divide() {
    assert_eq!(P29::solve_v1(10, 3), 3);
    assert_eq!(P29::solve_v1(7, -3), -2);
}
#[test]
fn test_p30_find_substring() {
    assert_eq!(
        P30::solve_v1(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()]
        ),
        vec![0, 9]
    );
    assert_eq!(
        P30::solve_v1(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "word".to_string()
            ]
        ),
        vec![]
    );
    assert_eq!(
        P30::solve_v1(
            "barfoofoobarthefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string(), "the".to_string()]
        ),
        vec![6, 9, 12]
    );
}
