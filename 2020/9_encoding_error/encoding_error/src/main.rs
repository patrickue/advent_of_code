use std::env;
use std::error;
use std::ffi::OsString;
use std::convert::TryFrom;

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

#[derive(Debug)]
#[derive(Copy, Clone)]
enum Operation {
    Jmp,
    Acc,
    Nop,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Instruction {
    op: Operation,
    arg: isize,
}

fn main() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: handheld_halting <handheld_code.txt>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_str_vec_from_file(filename) {
        Ok(answer_vec) => {
            println!("Successful: {:?}", answer_vec);
            let instructions = answer_vec.into_iter()
                .map(|l| parse_value(l))
                .collect::<Vec<usize>>();
            println!("Parsed instructions {:?}", instructions);
            let loop_acc = check_if_recombination(instructions[0..4].to_vec(), instructions[5]);
            println!("Trying to recombined resulted in: {:?}.", loop_acc);
        }
        Err(text) => println!("Error occured: {}", text),
    }
}

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
    //Map a possible ParseIntError onto Box Error
}

fn parse_value<'a>(line: String) -> usize {
    line.parse::<usize>().unwrap()
}

fn check_if_recombination(mut preamble: Vec<usize>, expected_sum: usize) -> bool {
    for i in 0..preamble.len() {
        for j in i..preamble.len() {
            if i != j {
                if preamble[i]+preamble[j] == expected_sum {
                    return true;
                }
            }
        }
    }
    return false;
}