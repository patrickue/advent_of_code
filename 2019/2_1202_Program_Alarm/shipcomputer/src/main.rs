use std::env;
//use std::fs::File as File;
use std::error;
//use std::io::ErrorKind as ErrorKind;
use std::ffi::OsString;

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

fn main() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: shipcomputer <IntCodeFile>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_intcode_vec_from_file(filename) {
        Ok(opcode_vec) => {
            let res = execute_modified_program(opcode_vec);
            println!("Successfully executed the program. Result is: {:?}", res);
        }
        Err(text) => println!("Error occured: {}", text),
    }
}

fn get_args() -> Result<String, ArgsError>{
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
        .map( |oss| oss.into_string())
        // collect to get the Results on the outside
        .collect::<Result<Vec<_>, _>>()
        //convert vector into tuple of Strings
        .map(|mut v| (v.remove(0)))
        //wrap conversion error into our Error
        .map_err(|oss| ArgsError::NotUtf8(oss))

}

fn execute_modified_program(opcode_vec: Vec<usize>) -> Option<usize> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut tmp_vec = opcode_vec.clone();
            tmp_vec[1] = noun;
            tmp_vec[2] = verb;
            let res = execute_intopcode_program(tmp_vec);
            if res == Some(19690720) {
                println!("Noun: {}, verb: {}, Res: {:?}", noun, verb, res);
            }
        }
    }
    None
}

fn execute_intopcode_program(mut opcode_vec: Vec<usize>) -> Option<usize> {
    let mut pos = Some(0);
    loop {
        match pos {
            Some(old_pos) => {
                pos = execute_one_opcode(&mut opcode_vec, old_pos);
            }
            None => {
                //we reached a 99
                return Some(opcode_vec[0])
            }
        }
    }
}

fn collect_intcode_vec_from_file(inputname: String) -> Result<Vec<usize>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

// === New Part, also works ===
    let buf = BufReader::new(fi);
//let lines: Result<i32, _> =

    let mut lines_iter = buf.lines().map(|l| l.unwrap());
    // We only expect one line:
    lines_iter.next().unwrap()
        .split(',')
        .map(|p| p.parse::<usize>())
        .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
//Map a possible ParseIntError onto Box Error
        .map_err(|e| e.into())
}

fn execute_one_opcode(opcode_vec: &mut Vec<usize>, curr_pos: usize) -> Option<usize> {
    //Decode command:
    //println!("Before:    {:?}", opcode_vec);
    match opcode_vec[curr_pos] {
        1 => {
            let add_pos1 = opcode_vec[curr_pos+1];
            let add_pos2 = opcode_vec[curr_pos+2];
            let res_pos = opcode_vec[curr_pos+3];

            let add_1 = opcode_vec[add_pos1];
            let add_2 = opcode_vec[add_pos2];
            opcode_vec[res_pos] = add_1 + add_2;
            //println!("After Add: {:?}", opcode_vec);
            Some(curr_pos+4)
        }
        2 => {
            let mul_pos1 = opcode_vec[curr_pos+1];
            let mul_pos2 = opcode_vec[curr_pos+2];
            let res_pos = opcode_vec[curr_pos+3];

            let mul_1 = opcode_vec[mul_pos1];
            let mul_2 = opcode_vec[mul_pos2];
            opcode_vec[res_pos] = mul_1 * mul_2;
            //println!("After Mul: {:?}", opcode_vec);
            Some(curr_pos+4)
        }
        99 => {None}
        _ => unreachable!()
    }
}

