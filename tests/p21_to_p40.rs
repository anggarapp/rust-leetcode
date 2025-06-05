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

#[test]
fn test_p31_next_permutation() {
    let mut vec_one = vec![1, 2, 3];
    P31::solve_v1(&mut vec_one);
    assert_eq!(vec_one, vec![1, 3, 2]);

    let mut vec_two = vec![3, 2, 1];
    P31::solve_v1(&mut vec_two);
    assert_eq!(vec_two, vec![1, 2, 3]);

    let mut vec_three = vec![1, 1, 5];
    P31::solve_v1(&mut vec_three);
    assert_eq!(vec_three, vec![1, 5, 1]);
}

#[test]
fn test_p32_longest_valid_parentheses() {
    assert_eq!(P32::solve_v3("(()".to_string()), 2);
    assert_eq!(P32::solve_v3(")()())".to_string()), 4);
    assert_eq!(P32::solve_v3("".to_string()), 0);
}

#[test]
fn test_p33_search() {
    assert_eq!(P33::solve_v2(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(P33::solve_v2(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(P33::solve_v2(vec![1], 0), -1);
}

#[test]
fn test_p34_search_range() {
    assert_eq!(P34::solve_v2(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(P34::solve_v2(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    assert_eq!(P34::solve_v2(vec![], 0), [-1, -1]);
}
#[test]
fn test_p35_search_insert() {
    assert_eq!(P35::solve_v1(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(P35::solve_v1(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(P35::solve_v1(vec![1, 3, 5, 6], 7), 4);
}

#[test]
fn test_p36_is_valid_sudoku() {
    assert_eq!(
        P36::solve_v1(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]),
        true
    );
    assert_eq!(
        P36::solve_v1(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]),
        false
    );
    assert_eq!(
        P36::solve_v1(vec![
            vec!['.', '8', '7', '6', '5', '4', '3', '2', '1'],
            vec!['2', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['3', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['4', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['6', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['9', '.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        true
    );
}

#[test]

fn test_p37_solve_sudoku() {
    let mut board_one_start = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let board_one_end = vec![
        vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
        vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
        vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
        vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
        vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
        vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
        vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
        vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
        vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
    ];
    P37::solve_v3(&mut board_one_start);
    assert_eq!(board_one_start, board_one_end);
}

#[test]
fn test_p38_count_and_say() {
    assert_eq!(P38::solve_v1(1), "1".to_string());
    assert_eq!(P38::solve_v1(4), "1211".to_string());
}

#[test]
fn test_p39_combination_sum() {
    assert_eq!(
        P39::solve_v1(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
    assert_eq!(
        P39::solve_v1(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );

    assert_eq!(P39::solve_v1(vec![2], 1), vec![] as Vec<Vec<i32>>);

    assert_eq!(
        P39::solve_v1(vec![8, 7, 4, 3], 11),
        vec![vec![3, 4, 4], vec![3, 8], vec![4, 7]]
    );
}

#[test]
fn test_p40_combination_sum_distinct() {
    assert_eq!(
        P40::solve_v1(vec![10, 1, 2, 7, 6, 1, 5], 8),
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
    );
}

#[test]
fn test_p41_first_missing_positive() {
    assert_eq!(P41::solve_v2(vec![1,2,0]), 3);
    assert_eq!(P41::solve_v2(vec![3,4,-1,1]), 2);
    assert_eq!(P41::solve_v2(vec![7,8,9,11,12]), 1);
}

#[test]
fn test_p42_trap() {
    assert_eq!(P42::solve_v1(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    assert_eq!(P42::solve_v1(vec![4,2,0,3,2,5]), 9);
    
}

#[test]
fn test_p43_multiply_string() {
    assert_eq!(P43::solve_v1("2".to_string(), "3".to_string()), "6".to_string());
    assert_eq!(P43::solve_v1("123".to_string(), "456".to_string()), "56088".to_string());
}