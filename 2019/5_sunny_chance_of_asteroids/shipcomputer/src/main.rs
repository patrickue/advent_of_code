use std::env;
//use std::fs::File as File;
use std::error;
//use std::io::ErrorKind as ErrorKind;
use std::ffi::OsString;
use std::hint::unreachable_unchecked;

mod tests;

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

#[derive(Copy, Clone)]
enum ParamMode {
    PositionMode,
    IntermediateMode
}

struct ComputerState {
    //program counter
    pc: usize,
    //the one and only register
    reg: Option<usize>
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
    match collect_str_vec_from_file(filename) {
        Ok(intcode_str_vec) => {
            match collect_intcode_from_string(intcode_str_vec[0].to_string()) {
                Ok(opcode_vec) => {
                    let res = execute_modified_program(opcode_vec);
                    println!("Successfully executed the program. Result is: {:?}", res);
                }
                Err(text) => println!("Error occured: {}", text),
            }
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
    let mut state_opt = Some(ComputerState {
        pc: 0,
        reg: None
    });
    loop {
        match state_opt {
            Some(state) => {
                state_opt = execute_one_opcode(&mut opcode_vec, state);
            }
            None => {
                //we reached a 99
                return Some(opcode_vec[0]);
            }
        }
    }
}

fn collect_str_vec_from_file(inputname: String) -> Result<Vec<String>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

// === New Part, also works ===
    let buf = BufReader::new(fi);
//let lines: Result<i32, _> =

    let mut lines_iter = buf.lines().map(|l| l.unwrap());
    // We only expect one line:
    Ok(lines_iter.collect::<Vec<String>>())
    /*next().unwrap()
    .split(',')
    .map(|p| p.parse::<usize>())
    .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
//Map a possible ParseIntError onto Box Error
    .map_err(|e| e.into())*/
}

fn collect_intcode_from_string(inputstring: String) -> Result<Vec<usize>, Box<dyn error::Error>> {
    inputstring
        .split(',')
        .map(|p| p.parse::<usize>())
        .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
        //Map a possible ParseIntError onto Box Error
        .map_err(|e| e.into())
}

fn execute_one_opcode(opcode_vec: &mut Vec<usize>, state: ComputerState) -> Option<ComputerState> {
    //Decode command:
    //println!("Before:    {:?}", opcode_vec);
    let mut next_state = None;
    let curr_pos: usize = state.pc;


    let mut command = opcode_vec[curr_pos];

    let opcode = command % 100;
    command = command/100;

    let mut param_mode = [ParamMode::PositionMode; 3];

    let mut pm_idx = 0;
    while command > 0 {
        match command % 10 {
            1 => {
                param_mode[pm_idx] = ParamMode::IntermediateMode;
            },
            0 => { /*param mode is default position mode, so keep everything*/ },
            _ => {
                unreachable!();
            }
        };
        command = command / 10;
        pm_idx += 1;
    }

    match opcode {
        1 => {
            let operand1 = get_operand(opcode_vec, curr_pos+1, param_mode[0]);
            let operand2 = get_operand(opcode_vec, curr_pos+2, param_mode[1]);
            /*let add_pos1 = opcode_vec[curr_pos + 1];
            let add_pos2 = opcode_vec[curr_pos + 2];*/
            let res_pos = opcode_vec[curr_pos + 3];

            /*let add_1 = opcode_vec[add_pos1];
            let add_2 = opcode_vec[add_pos2];*/
            opcode_vec[res_pos] = operand1 + operand2;
            //println!("After Add: {:?}", opcode_vec);
            next_state = Some(ComputerState{
                pc: curr_pos + 4,
                reg: None
            });
        }
        2 => {
            let operand1 = get_operand(opcode_vec, curr_pos+1, param_mode[0]);
            let operand2 = get_operand(opcode_vec, curr_pos+2, param_mode[1]);
            /*let mul_pos1 = opcode_vec[curr_pos + 1];
            let mul_pos2 = opcode_vec[curr_pos + 2];*/
            let res_pos = opcode_vec[curr_pos + 3];

            /*let mul_1 = opcode_vec[mul_pos1];
            let mul_2 = opcode_vec[mul_pos2];*/
            opcode_vec[res_pos] = operand1 * operand2;
            //println!("After Mul: {:?}", opcode_vec);
            next_state = Some(ComputerState{
                pc: curr_pos + 4,
                reg: None
            });
        }
        3 => {
            //take intermediate register and save to position
            let res_pos = opcode_vec[curr_pos + 1];
            match state.reg {
                Some(buff_cont) => opcode_vec[res_pos] = buff_cont,
                None => unreachable!()
            }
            next_state = Some(ComputerState{
                pc: curr_pos + 2,
                reg: None
            });
        }
        4 => {
            //output signaled position to intermediate register
            let res_pos = opcode_vec[curr_pos + 1];
            next_state = Some(ComputerState{
                pc: curr_pos + 2,
                reg: Some(opcode_vec[res_pos])
            });
        }
        99 => { }
        a => {
            println!("Unreachable! Opcode was: {}", a);
            unreachable!();
        }
    }
    return next_state;
}

fn get_operand(opcode_vec: &mut Vec<usize>, param_pos: usize, param_mode: ParamMode) -> usize
{
    match param_mode {
        ParamMode::PositionMode => {
            opcode_vec[opcode_vec[param_pos]]
        },
        ParamMode::IntermediateMode => {
            opcode_vec[param_pos]
        }
    }
}