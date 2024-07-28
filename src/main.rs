fn main() {
    println!("Hello Kiddos");
}

#[cfg(test)]
mod tests {
    use rust_solve_leetcode::solver::*;

    #[test]
    fn test_two_sum_v1() {
        assert_eq!(Problem::solve_two_sum_v1(vec![2, 7, 9, 1], 8), vec![1, 3]);
    }

    #[test]
    fn test_two_sum_vx() {
        assert_eq!(Problem::solve_two_sum_vx(vec![2, 7, 9, 1], 8), vec![1, 3]);
    }

    #[test]
    fn test_longest_substring_without_repeating_char_v1() {
        assert_eq!(
            Problem::solve_longest_substring_without_repeating_char_v1(String::from("abcaabbca")),
            3
        );
    }
    #[test]
    fn test_longest_substring_without_repeating_char_vx() {
        assert_eq!(
            Problem::solve_longest_substring_without_repeating_char_vx(String::from("abcaabbca")),
            3
        );
    }
    #[test]
    fn test_median_of_two_sorted_array_v1() {
        assert_eq!(
            Problem::solve_median_of_two_sorted_array_v1(vec![1, 2], vec![3, 4]),
            2.5
        );
        assert_eq!(
            Problem::solve_median_of_two_sorted_array_v1(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Problem::solve_median_of_two_sorted_array_v1(vec![2, 4], vec![2, 4]),
            3.0
        );
        assert_eq!(
            Problem::solve_median_of_two_sorted_array_v1(vec![-2, 4], vec![2, 4]),
            3.0
        );
    }

    #[test]
    fn test_longest_palindromic_substring() {
        assert_eq!(
            Problem::solve_longest_palindromic_substring_v1(String::from("babdabb")),
            String::from("bab")
        );
        assert_eq!(
            Problem::solve_longest_palindromic_substring_v1(String::from("sssaassdbccd")),
            String::from("ssaass")
        );
        assert_eq!(
            Problem::solve_longest_palindromic_substring_v1(String::from("tattarrattat")),
            String::from("tattarrattat")
        );
    }
    #[test]
    fn test_zigzag_convertion() {
        assert_eq!(
            Problem::solve_zigzag_convertion_v1(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
        assert_eq!(
            Problem::solve_zigzag_convertion_v1(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
    }

    #[test]
    fn test_reverse_integer() {
        assert_eq!(Problem::solve_reverse_integer_v1(123), 321);
        assert_eq!(Problem::solve_reverse_integer_v1(-123), -321);
        assert_eq!(Problem::solve_reverse_integer_v1(120), 21);
        assert_eq!(Problem::solve_reverse_integer_v1(1534236469), 0);
    }
}
