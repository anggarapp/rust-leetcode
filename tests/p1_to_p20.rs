use rust_solve_leetcode::solver::*;

#[test]
fn test_p1_two_sum_v1() {
    assert_eq!(P1::naive_solve(vec![2, 7, 9, 1], 8), vec![1, 3]);
}

#[test]
fn test_p1_two_sum_vx() {
    assert_eq!(P1::v1_solve(vec![2, 7, 9, 1], 8), vec![1, 3]);
}

#[test]
fn test_p2_add_two_number() {
    let node_one = Some(Box::new(ListNode { val: 3, next: None }));
    let node_one = Some(Box::new(ListNode {
        val: 4,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 2,
        next: node_one,
    }));
    let node_two = Some(Box::new(ListNode { val: 4, next: None }));
    let node_two = Some(Box::new(ListNode {
        val: 6,
        next: node_two,
    }));
    let node_two = Some(Box::new(ListNode {
        val: 5,
        next: node_two,
    }));

    let res_node = Some(Box::new(ListNode { val: 8, next: None }));
    let res_node = Some(Box::new(ListNode {
        val: 0,
        next: res_node,
    }));
    let res_node = Some(Box::new(ListNode {
        val: 7,
        next: res_node,
    }));
    assert_eq!(P2::solve(node_one, node_two), res_node);

    let node_one = Some(Box::new(ListNode { val: 0, next: None }));
    let node_two = Some(Box::new(ListNode { val: 0, next: None }));
    let res_node = Some(Box::new(ListNode { val: 0, next: None }));

    assert_eq!(P2::solve(node_one, node_two), res_node);

    let node_one = Some(Box::new(ListNode { val: 9, next: None }));
    let node_one = Some(Box::new(ListNode {
        val: 9,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 9,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 9,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 9,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 9,
        next: node_one,
    }));
    let node_one = Some(Box::new(ListNode {
        val: 9,
        next: node_one,
    }));

    let node_two = Some(Box::new(ListNode { val: 9, next: None }));
    let node_two = Some(Box::new(ListNode {
        val: 9,
        next: node_two,
    }));
    let node_two = Some(Box::new(ListNode {
        val: 9,
        next: node_two,
    }));
    let node_two = Some(Box::new(ListNode {
        val: 9,
        next: node_two,
    }));

    let res_node = Some(Box::new(ListNode { val: 1, next: None }));
    let res_node = Some(Box::new(ListNode {
        val: 0,
        next: res_node,
    }));
    let res_node = Some(Box::new(ListNode {
        val: 0,
        next: res_node,
    }));
    let res_node = Some(Box::new(ListNode {
        val: 0,
        next: res_node,
    }));
    let res_node = Some(Box::new(ListNode {
        val: 9,
        next: res_node,
    }));
    let res_node = Some(Box::new(ListNode {
        val: 9,
        next: res_node,
    }));
    let res_node = Some(Box::new(ListNode {
        val: 9,
        next: res_node,
    }));
    let res_node = Some(Box::new(ListNode {
        val: 8,
        next: res_node,
    }));

    assert_eq!(P2::solve(node_one, node_two), res_node);
}

#[test]
fn test_p3_longest_substring_without_repeating_char_v1() {
    assert_eq!(P3::naive_solve(String::from("abcaabbca")), 3);
}
#[test]
fn test_p3_longest_substring_without_repeating_char_vx() {
    assert_eq!(P3::better_solve(String::from("abcaabbca")), 3);
}
#[test]
fn test_p4_median_of_two_sorted_array_v1() {
    assert_eq!(P4::solve(vec![1, 2], vec![3, 4]), 2.5);
    assert_eq!(P4::solve(vec![1, 3], vec![2]), 2.0);
    assert_eq!(P4::solve(vec![2, 4], vec![2, 4]), 3.0);
    assert_eq!(P4::solve(vec![-2, 4], vec![2, 4]), 3.0);
}

#[test]
fn test_p5_longest_palindromic_substring() {
    assert_eq!(P5::solve(String::from("babdabb")), String::from("bab"));
    assert_eq!(
        P5::solve(String::from("sssaassdbccd")),
        String::from("ssaass")
    );
    assert_eq!(
        P5::solve(String::from("tattarrattat")),
        String::from("tattarrattat")
    );
}
#[test]
fn test_p6_zigzag_convertion() {
    assert_eq!(
        P6::solve(String::from("PAYPALISHIRING"), 3),
        String::from("PAHNAPLSIIGYIR")
    );
    assert_eq!(
        P6::solve(String::from("PAYPALISHIRING"), 4),
        String::from("PINALSIGYAHRPI")
    );
}

#[test]
fn test_p7_reverse_integer() {
    assert_eq!(P7::solve_v1(123), 321);
    assert_eq!(P7::solve_v1(-123), -321);
    assert_eq!(P7::solve_v1(120), 21);
    assert_eq!(P7::solve_v1(1534236469), 0);
}

#[test]
fn test_p8_string_to_integer_atoi_v1() {
    assert_eq!(P8::naive_solve(String::from("   -042")), -42);
    assert_eq!(P8::naive_solve(String::from("42")), 42);
    assert_eq!(P8::naive_solve(String::from("1337c0d3")), 1337);
    assert_eq!(P8::naive_solve(String::from("words and 987")), 0);
    assert_eq!(P8::naive_solve(String::from("-91283472332")), -2147483648);
}

#[test]
fn test_p8_string_to_integer_atoi_vx() {
    assert_eq!(P8::better_solve(String::from("   -042")), -42);
    assert_eq!(P8::better_solve(String::from("42")), 42);
    assert_eq!(P8::better_solve(String::from("1337c0d3")), 1337);
    assert_eq!(P8::better_solve(String::from("words and 987")), 0);
    assert_eq!(P8::better_solve(String::from("-91283472332")), -2147483648);
}
#[test]
fn test_p9_palindrom_number_v1() {
    assert_eq!(P9::solve(122), false);
    assert_eq!(P9::solve(12121), true);
    assert_eq!(P9::solve(-121), false);
}

#[test]
fn test_p10_regular_expression_matching_v1() {
    assert_eq!(P10::solve("aa".to_string(), "a".to_string()), false);
    assert_eq!(P10::solve("aa".to_string(), "a*".to_string()), true);
    assert_eq!(P10::solve("ab".to_string(), ".*".to_string()), true);
    assert_eq!(P10::solve("aab".to_string(), "c*a*b".to_string()), true);
    assert_eq!(
        P10::solve("mississippi".to_string(), "mis*is*p*.".to_string()),
        false
    );
    assert_eq!(P10::solve("ab".to_string(), ".*".to_string()), true);
}

#[test]
fn test_p11_container_with_most_water() {
    assert_eq!(P11::naive_solve(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(P11::naive_solve(vec![1, 1]), 1);
}

#[test]
fn test_p12_integer_to_roman() {
    assert_eq!(P12::other_v1_solve(3749), "MMMDCCXLIX".to_string());
    assert_eq!(P12::other_v1_solve(58), "LVIII".to_string());
    assert_eq!(P12::other_v1_solve(1994), "MCMXCIV".to_string());
}

#[test]
fn test_p13_roman_to_integer() {
    assert_eq!(P13::v1_solve("III".to_string()), 3);
    assert_eq!(P13::v1_solve("LVIII".to_string()), 58);
    assert_eq!(P13::v1_solve("MCMXCIV".to_string()), 1994);
}

#[test]
fn test_p14_longest_common_prefix() {
    assert_eq!(
        P14::solve_v1(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl".to_string()
    );
    assert_eq!(
        P14::solve_v1(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        "".to_string()
    );
}

#[test]
fn test_p15_three_sum() {
    assert_eq!(
        P15::solve_v1(vec![-1, 0, 1, 2, -1, -4]),
        vec![[-1, -1, 2], [-1, 0, 1]]
    );
    assert_eq!(
        P15::solve_v2(vec![-1, 0, 1, 2, -1, -4]),
        vec![[-1, -1, 2], [-1, 0, 1]]
    );
    assert_eq!(P15::solve_v1(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);
    assert_eq!(P15::solve_v2(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);

    assert_eq!(P15::solve_v1(vec![0, 0, 0]), vec![[0, 0, 0]]);
    assert_eq!(P15::solve_v2(vec![0, 0, 0]), vec![[0, 0, 0]]);
}

#[test]
fn test_p16_three_sum_closest() {
    assert_eq!(P16::solve_v1(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(P16::solve_v1(vec![0, 0, 0], 1), 0);
    assert_eq!(P16::solve_v2(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(P16::solve_v2(vec![0, 0, 0], 1), 0);
}

#[test]
fn test_p17_letter_combinations() {
    assert_eq!(
        P17::solve_v1("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    assert_eq!(P17::solve_v1("2".to_string()), vec!["a", "b", "c"]);
    assert_eq!(P17::solve_v1("".to_string()), vec![] as Vec<String>);

    assert_eq!(
        P17::solve_v2("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    assert_eq!(P17::solve_v2("2".to_string()), vec!["a", "b", "c"]);
    assert_eq!(P17::solve_v2("".to_string()), vec![] as Vec<String>);

    assert_eq!(
        P17::solve_v3("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    assert_eq!(P17::solve_v3("2".to_string()), vec!["a", "b", "c"]);
    assert_eq!(P17::solve_v3("".to_string()), vec![] as Vec<String>);
}

#[test]
fn test_p18_four_sum() {
    assert_eq!(
        P18::solve_v1(vec![1, 0, -1, 0, -2, 2], 0),
        vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
    );
    assert_eq!(P18::solve_v1(vec![2, 2, 2, 2, 2], 8), vec![[2, 2, 2, 2]]);
    assert_eq!(
        P18::solve_v1(
            vec![1000000000, 1000000000, 1000000000, 1000000000],
            -294967296
        ),
        vec![] as Vec<Vec<i32>>
    );

    assert_eq!(
        P18::solve_v2(vec![1, 0, -1, 0, -2, 2], 0),
        vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
    );
    assert_eq!(P18::solve_v2(vec![2, 2, 2, 2, 2], 8), vec![[2, 2, 2, 2]]);
    assert_eq!(
        P18::solve_v2(
            vec![1000000000, 1000000000, 1000000000, 1000000000],
            -294967296
        ),
        vec![] as Vec<Vec<i32>>
    );

    assert_eq!(
        P18::solve_v3(vec![1, 0, -1, 0, -2, 2], 0),
        vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
    );
    assert_eq!(P18::solve_v3(vec![2, 2, 2, 2, 2], 8), vec![[2, 2, 2, 2]]);
    assert_eq!(
        P18::solve_v3(
            vec![1000000000, 1000000000, 1000000000, 1000000000],
            -294967296
        ),
        vec![] as Vec<Vec<i32>>
    );
}

#[test]
fn test_p19_remove_nth_from_end() {
    let node_input = Some(Box::new(ListNode { val: 5, next: None }));
    let node_input = Some(Box::new(ListNode {
        val: 4,
        next: node_input,
    }));
    let node_input = Some(Box::new(ListNode {
        val: 3,
        next: node_input,
    }));
    let node_input = Some(Box::new(ListNode {
        val: 2,
        next: node_input,
    }));
    let node_input = Some(Box::new(ListNode {
        val: 1,
        next: node_input,
    }));

    //
    let node_output = Some(Box::new(ListNode { val: 5, next: None }));
    let node_output = Some(Box::new(ListNode {
        val: 3,
        next: node_output,
    }));
    let node_output = Some(Box::new(ListNode {
        val: 2,
        next: node_output,
    }));
    let node_output = Some(Box::new(ListNode {
        val: 1,
        next: node_output,
    }));

    assert_eq!(P19::solve_v1(node_input, 2), node_output);

    let node_input = Some(Box::new(ListNode { val: 2, next: None }));
    let node_input = Some(Box::new(ListNode {
        val: 1,
        next: node_input,
    }));
    let node_output = Some(Box::new(ListNode { val: 1, next: None }));

    assert_eq!(P19::solve_v1(node_input, 1), node_output);

    let node_input = Some(Box::new(ListNode { val: 1, next: None }));

    let node_output = None;

    assert_eq!(P19::solve_v1(node_input, 1), node_output);
}

#[test]
fn test_p20_valid_parentheses() {
    assert_eq!(P20::solve_v1("()".to_string()), true);
    assert_eq!(P20::solve_v1("()[]{}".to_string()), true);
    assert_eq!(P20::solve_v1("(]".to_string()), false);
    assert_eq!(P20::solve_v1("([])".to_string()), true);

    assert_eq!(P20::solve_v2("()".to_string()), true);
    assert_eq!(P20::solve_v2("()[]{}".to_string()), true);
    assert_eq!(P20::solve_v2("(]".to_string()), false);
    assert_eq!(P20::solve_v2("([])".to_string()), true);

    assert_eq!(P20::solve_v3("()".to_string()), true);
    assert_eq!(P20::solve_v3("()[]{}".to_string()), true);
    assert_eq!(P20::solve_v3("(]".to_string()), false);
    assert_eq!(P20::solve_v3("([])".to_string()), true);
}
