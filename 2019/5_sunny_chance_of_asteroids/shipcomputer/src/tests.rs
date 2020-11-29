#[cfg(test)]
mod tests {
    use crate::{collect_intcode_from_string, execute_intopcode_program};

    #[test]
    fn execute_example_program_day_1() {
        let examplecode = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut int_code_vec = collect_intcode_from_string(examplecode.to_string()).unwrap();
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec, None));
        assert_eq!(3500, int_code_vec[0]);
    }

    #[test]
    fn execute_example_program_day_5_p1() {
        let examplecode = "4,4,3,0,99";
        let mut int_code_vec: Vec<isize> = collect_intcode_from_string(examplecode.to_string()).unwrap();

        execute_intopcode_program(&mut int_code_vec, Some(1));
        assert_eq!(99, int_code_vec[0]);
    }

    #[test]
    fn self_ending_program_day5_p1() {
        let examplecode = "1002,4,3,4,33";
        let mut int_code_vec = collect_intcode_from_string(examplecode.to_string()).unwrap();
        //This is assert does not really matter.
        // If the program above does not work correctly, it might reach an
        // "unreachable" state. So it this test executes at all, it is OK.
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec, Some(1)));
        assert_eq!(1002, int_code_vec[0]);
    }

    #[test]
    fn negative_number_program_day5_p1() {
        let examplecode = "1101,100,-1,4,0";
        let mut int_code_vec = collect_intcode_from_string(examplecode.to_string()).unwrap();
        //This is assert does not really matter, like previous test.
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec, None));
        assert_eq!(1101, int_code_vec[0]);
    }

    /// Using position mode, consider whether the input is equal to 8;
    /// output 1 (if it is) or 0 (if it is not).
    #[test]
    fn position_mode_equal8_day5_p2_false() {
        let examplecode = "3,9,8,9,10,9,4,9,99,-1,8";
        let input = Some(7);
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(0), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }

    #[test]
    fn position_mode_equal8_day5_p2_true() {
        let examplecode = "3,9,8,9,10,9,4,9,99,-1,8";
        let input = Some(8);
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(1), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }

    ///Using position mode, consider whether the input is less than 8;
    /// output 1 (if it is) or 0 (if it is not).
    #[test]
    fn position_mode_lessthan8_day5_p2() {
        let examplecode = "3,9,7,9,10,9,4,9,99,-1,8";
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(1101), execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }

    ///Using immediate mode, consider whether the input is equal to 8;
    /// output 1 (if it is) or 0 (if it is not).
    #[test]
    fn immediate_mode_equal8_day5_p2() {
        let examplecode = "3,3,1108,-1,8,3,4,3,99";
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(1101), execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }

    ///Using immediate mode, consider whether the input is less than 8;
    /// output 1 (if it is) or 0 (if it is not).
    #[test]
    fn immediate_mode_lessthan8_day5_p2() {
        let examplecode = "3,3,1107,-1,8,3,4,3,99";
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(1101), execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }
}
