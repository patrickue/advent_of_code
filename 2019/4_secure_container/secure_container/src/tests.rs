
#[cfg(test)]
mod tests {
    use crate::{two_digits_adjacent, never_decreases};

    #[test]
    fn two_adjacent_four_digits() {
        assert_eq!(true, two_digits_adjacent(1223));
    }

    #[test]
    fn two_adjacent_end() {
        assert_eq!(true, two_digits_adjacent(101245622));
    }

    #[test]
    fn two_adjacent_six_begin() {
        assert_eq!(true, two_digits_adjacent(661245627894567));
    }

    #[test]
    fn two_adjacent_zero_middle() {
        assert_eq!(true, two_digits_adjacent(124562007894567));
    }

    #[test]
    fn two_adjacent_zero_begin() {
        assert_eq!(true, two_digits_adjacent(1245627456700));
    }

    #[test]
    fn no_adjacent_four_digits() {
        assert_eq!(false, two_digits_adjacent(123563765));
    }

    #[test]
    fn accept_two_adjacent_when_also_three() {
        assert_eq!(true, two_digits_adjacent(11122700));
    }

    #[test]
    fn accept_double_two_subsequent_digits() {
        assert_eq!(true, two_digits_adjacent(23556637));
    }

    #[test]
    fn dont_accept_three_subsequent_digits() {
        assert_eq!(false, two_digits_adjacent(23566637));
    }

    #[test]
    fn dont_accept_three_subsequent_digits_at_end() {
        assert_eq!(false, two_digits_adjacent(235666));
    }

    #[test]
    fn dont_accept_three_subsequent_digits_at_begin() {
        assert_eq!(false, two_digits_adjacent(222356));
    }

    #[test]
    fn never_decreases_four_digits_yes() {
        assert_eq!(true, never_decreases(1235));
    }

    #[test]
    fn never_decreases_nine_digits_no() {
        assert_eq!(false, never_decreases(212356789));
    }

    #[test]
    fn never_decreases_seven_digits_yes() {
        assert_eq!(true, never_decreases(23567899));
    }

    #[test]
    fn never_decreases_seven_digits_no() {
        assert_eq!(false, never_decreases(11111110));
    }

    #[test]
    fn never_decreases_six_digits_yes() {
        assert_eq!(true, never_decreases(111112));
    }
}
