use std::env;
use std::error;
use std::ffi::OsString;

//mod forest_08;
//mod planks_09;
mod cathode_ray_tube_10;

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}


fn main/*_10_cathode_ray_tube*/() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: rust_solutions <input.txt>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_str_vec_from_file(filename) {
        Ok(instruction_strings) => {
            println!("Successful: {:?}", instruction_strings);
            let instructions = cathode_ray_tube_10::parse_commands(instruction_strings).unwrap();
            println!("Parsed instructions {:?}", instructions);
            let count: isize = cathode_ray_tube_10::get_signal_strengths_sum(instructions);
            println!("Signal strengths sum is {:?}.", count);
        }
        Err(text) => println!("Error occured: {}", text),
    }
}

/*fn main_09_bridge() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: rust_solutions <input.txt>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_str_vec_from_file(filename) {
        Ok(movement_vector) => {
            println!("Successful: {:?}", movement_vector);
            let movements = planks_09::parse_movements(movement_vector).unwrap();
            println!("Parsed movements {:?}", movements);
            let count: usize = planks_09::simulate_the_rope_return_tail_positions_part2(movements);
            println!("Tail moves over  {:?} fields.", count);
        }
        Err(text) => println!("Error occured: {}", text),
    }
}


fn main_08_forest() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: rust_solutions <input.txt>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_str_vec_from_file(filename) {
        Ok(terminal_output) => {
            println!("Successful: {:?}", terminal_output);
            let forest = forest_08::parse_forest(terminal_output).unwrap();
            println!("Parsed forest {:?}", forest);
            let count: usize = forest_08::calculate_scenic_view(forest);
            println!("Best Tree has {:?} scenic score.", count);
        }
        Err(text) => println!("Error occured: {}", text),
    }
}*/

fn get_args() -> Result<String, ArgsError> {
    // Prints each argument on a separate line

    match env::args_os().count() {
        n if n > 2 => return Err(ArgsError::TooManyArgs(n - 1)),
        n if n < 2 => return Err(ArgsError::NotEnoughArgs),
        _ => {}
    }

    env::args_os()
        //get me the first two
        .skip(1)
        .take(1)
        //map OsString into utf8
        .map(|oss| oss.into_string())
        // collect to get the Results on the outside
        .collect::<Result<Vec<_>, _>>()
        //convert vector into tuple of Strings
        .map(|mut v| (v.remove(0)))
        //wrap conversion error into our Error
        .map_err(|oss| ArgsError::NotUtf8(oss))
}

fn collect_str_vec_from_file(inputname: String) -> Result<Vec<String>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

    let buf = BufReader::new(fi);

    Ok(buf.lines()
        .map(|f| f.unwrap())
        .collect::<Vec<String>>())
}