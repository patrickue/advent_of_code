
#[cfg(test)]
mod tests {
    use crate::{collect_intcode_from_string, execute_intopcode_program};

    #[test]
    fn execute_example_program_day_1() {

        let examplecode = "1,9,10,3,2,3,11,0,99,30,40,50";
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        assert_eq!(Some(3500), execute_intopcode_program(int_code_vec.unwrap()));
    }

    #[test]
    fn execute_example_program_day_5_p1() {
        let examplecode = "4,4,3,0,99";
        let int_code_vec = collect_intcode_from_string(examplecode.to_string());
        assert_eq!(Some(99), execute_intopcode_program(int_code_vec.unwrap()));
    }
}
