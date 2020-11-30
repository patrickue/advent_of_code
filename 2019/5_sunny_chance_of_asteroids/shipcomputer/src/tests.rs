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

    /// Test for Opcode 5
    /// General description for Opcode 5 is jump-if-true:
    /// if the first parameter is non-zero, it sets the
    /// instruction pointer to the value from the second
    /// parameter. Otherwise, it does nothing.

    #[test]
    fn opcode5_jumptrue_immediate()
    {
        //if 42 is reached, program fails
        let intcode = "1105,1,4,42,99";
        let int_code_vec = collect_intcode_from_string(intcode.to_string());
        //This is assert does not really matter, successful execution is .
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }

    #[test]
    fn opcode5_jumpfalse_immediate()
    {
        //if 42 is reached, program fails
        let intcode = "1105,0,4,99,42";
        let int_code_vec = collect_intcode_from_string(intcode.to_string());
        //This is assert does not really matter, successful execution is .
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }

    #[test]
    fn opcode5_jumptrue_positional()
    {
        //if 42 is reached, program fails
        let intcode = "5,10,11,42,99,0,0,0,0,0,-1,4";
        let int_code_vec = collect_intcode_from_string(intcode.to_string());
        //This is assert does not really matter, successful execution is .
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }

    #[test]
    fn opcode5_jumpfalse_positional()
    {
        //if 42 is reached, program fails
        let intcode = "5,10,11,99,42,0,0,0,0,0,0,4";
        let int_code_vec = collect_intcode_from_string(intcode.to_string());
        //This is assert does not really matter, successful execution is .
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }

    /// Test for Opcode 6
    /// Opcode 6 is jump-if-false: if the first parameter is zero,
    /// it sets the instruction pointer to the value from the second
    /// parameter. Otherwise, it does nothing.

    #[test]
    fn opcode6_jumptrue_immediate()
    {
        //if 42 is reached, program fails
        let intcode = "1106,0,4,42,99";
        let int_code_vec = collect_intcode_from_string(intcode.to_string());
        //This is assert does not really matter, successful execution is .
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }

    #[test]
    fn opcode6_jumpfalse_immediate()
    {
        //if 42 is reached, program fails
        let intcode = "1106,1,4,99,42";
        let int_code_vec = collect_intcode_from_string(intcode.to_string());
        //This is assert does not really matter, successful execution is .
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }

    #[test]
    fn opcode6_jumptrue_positional()
    {
        //if 42 is reached, program fails
        let intcode = "6,10,11,42,99,0,0,0,0,0,0,4";
        let int_code_vec = collect_intcode_from_string(intcode.to_string());
        //This is assert does not really matter, successful execution is .
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec.unwrap(), None));
    }

    #[test]
    fn opcode6_jumpfalse_positional()
    {
        //if 42 is reached, program fails
        let intcode = "6,10,11,99,42,0,0,0,0,0,-173,4";
        let int_code_vec = collect_intcode_from_string(intcode.to_string());
        //This is assert does not really matter, successful execution is .
        assert_eq!(None, execute_intopcode_program(&mut int_code_vec.unwrap(), None));
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

    /// See previous description
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
    fn position_mode_notlessthan8_day5_p2() {
        let examplecode = "3,9,7,9,10,9,4,9,99,-1,8";
        let input = Some(9);
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(0), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }

    ///Using position mode, consider whether the input is less than 8;
    /// output 1 (if it is) or 0 (if it is not).
    #[test]
    fn position_mode_lessthan8_day5_p2() {
        let examplecode = "3,9,7,9,10,9,4,9,99,-1,8";
        let input = Some(7);
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(1), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }

    ///Using immediate mode, consider whether the input is equal to 8;
    /// output 1 (if it is) or 0 (if it is not).
    #[test]
    fn immediate_mode_notequal8_day5_p2() {
        let examplecode = "3,3,1108,-1,8,3,4,3,99";
        let input = Some(7);
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(0), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }

    #[test]
    fn immediate_mode_equal8_day5_p2() {
        let examplecode = "3,3,1108,-1,8,3,4,3,99";
        let input = Some(8);
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(1), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }

    ///Using immediate mode, consider whether the input is less than 8;
    /// output 1 (if it is) or 0 (if it is not).
    #[test]
    fn immediate_mode_lessthan8_day5_p2() {
        let examplecode = "3,3,1107,-1,8,3,4,3,99";
        let input = Some(-5);
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(1), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }

    #[test]
    fn immediate_mode_notlessthan8_day5_p2() {
        let examplecode = "3,3,1107,-1,8,3,4,3,99";
        let input = Some(10005);
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(0), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }


    const LONG_EXAMPLE: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    /// The above example program uses an input instruction to ask for a single number.
    /// The program will then output 999 if the input value is below 8, output 1000 if
    /// the input value is equal to 8, or output 1001 if the input value is greater than 8.
    #[test]
    fn long_example_case1_lowerthan8 () {
        let long_example = LONG_EXAMPLE.to_string();
        let input = Some(7);
        let int_code_vec = collect_intcode_from_string(long_example.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(999), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }

    #[test]
    fn long_example_case1_equalto8 () {
        let long_example = LONG_EXAMPLE.to_string();
        let input = Some(8);
        let int_code_vec = collect_intcode_from_string(long_example.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(1000), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }

    #[test]
    fn long_example_case1_largerthan8 () {
        let long_example = LONG_EXAMPLE.to_string();
        let input = Some(8);
        let int_code_vec = collect_intcode_from_string(long_example.to_string());
        //This is assert does not really matter, like previous test.
        assert_eq!(Some(1001), execute_intopcode_program(&mut int_code_vec.unwrap(), input));
    }
}
