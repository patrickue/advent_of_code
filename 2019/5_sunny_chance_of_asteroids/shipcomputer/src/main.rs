use std::env;
use std::error;
use std::ffi::OsString;

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
    IntermediateMode,
}

#[derive(Debug)]
struct ComputerState {
    //program counter
    pc: usize,
    //the one and only register
    reg: Option<isize>,
    end: bool,
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
                Ok(mut opcode_vec) => {
                    let res = execute_intopcode_program(&mut opcode_vec, Some(5));
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

fn execute_intopcode_program(mut opcode_vec: &mut Vec<isize>, start_val: Option<isize>) -> Option<isize> {
    let mut state = ComputerState {
        pc: 0,
        reg: start_val,
        end: false,
    };
    loop {
        match state.end {
            false => {
                state = execute_one_opcode(&mut opcode_vec, state);
            }
            true => {
                //we reached a 99
                return state.reg;
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

    let lines_iter = buf.lines().map(|l| l.unwrap());
    // We only expect one line:
    Ok(lines_iter.collect::<Vec<String>>())
    /*next().unwrap()
    .split(',')
    .map(|p| p.parse::<isize>())
    .collect::<Result<Vec<isize>, std::num::ParseIntError>>()
//Map a possible ParseIntError onto Box Error
    .map_err(|e| e.into())*/
}

fn collect_intcode_from_string(inputstring: String) -> Result<Vec<isize>, Box<dyn error::Error>> {
    inputstring
        .split(',')
        .map(|p| p.parse::<isize>())
        .collect::<Result<Vec<isize>, std::num::ParseIntError>>()
        //Map a possible ParseIntError onto Box Error
        .map_err(|e| e.into())
}

/// Return next state for computer
///
fn execute_one_opcode(opcode_vec: &mut Vec<isize>, state: ComputerState) -> ComputerState {
    //Decode command:
    println!("Before: {:?}", opcode_vec);
    println!("State: {:?}", state);
    let curr_pos: usize = state.pc;


    let mut command = opcode_vec[curr_pos];

    let opcode = command % 100;
    command = command / 100;

    let mut param_mode = [ParamMode::PositionMode; 3];

    let mut pm_idx = 0;
    while command > 0 {
        match command % 10 {
            1 => {
                param_mode[pm_idx] = ParamMode::IntermediateMode;
            }
            0 => { /*param mode is default position mode, so keep everything*/ }
            _ => {
                unreachable!();
            }
        };
        command = command / 10;
        pm_idx += 1;
    }

    match opcode {
        1 => {
            let operand1 = get_operand(opcode_vec, curr_pos + 1, param_mode[0]);
            let operand2 = get_operand(opcode_vec, curr_pos + 2, param_mode[1]);
            /*let add_pos1 = opcode_vec[curr_pos + 1];
            let add_pos2 = opcode_vec[curr_pos + 2];*/
            let res_pos = opcode_vec[curr_pos + 3];

            /*let add_1 = opcode_vec[add_pos1];
            let add_2 = opcode_vec[add_pos2];*/
            opcode_vec[res_pos as usize] = operand1 + operand2;
            //println!("After Add: {:?}", opcode_vec);
            ComputerState {
                pc: curr_pos + 4,
                reg: state.reg,
                end: false,
            }
        }
        2 => {
            let operand1 = get_operand(opcode_vec, curr_pos + 1, param_mode[0]);
            let operand2 = get_operand(opcode_vec, curr_pos + 2, param_mode[1]);
            /*let mul_pos1 = opcode_vec[curr_pos + 1];
            let mul_pos2 = opcode_vec[curr_pos + 2];*/
            let res_pos = opcode_vec[curr_pos + 3];

            /*let mul_1 = opcode_vec[mul_pos1];
            let mul_2 = opcode_vec[mul_pos2];*/
            opcode_vec[res_pos as usize] = operand1 * operand2;
            //println!("After Mul: {:?}", opcode_vec);
            ComputerState {
                pc: curr_pos + 4,
                reg: state.reg,
                end: false,
            }
        }
        3 => {
            //take intermediate register and save to position
            let res_pos = opcode_vec[curr_pos + 1];
            match state.reg {
                Some(buff_cont) => opcode_vec[res_pos as usize] = buff_cont,
                None => {
                    println!("{}", curr_pos);
                    unreachable!();
                }
            }
            ComputerState {
                pc: curr_pos + 2,
                reg: state.reg,
                end: false,
            }
        }
        4 => {
            //output signaled position to intermediate register
            let res_pos = opcode_vec[curr_pos + 1];
            let output = get_operand(opcode_vec, curr_pos +1 , param_mode[0]);
            ComputerState {
                pc: curr_pos + 2,
                reg: Some(output),
                end: false,
            }
            //println!("{:?}", next_state);
        }
        // Opcode 5 is jump-if-true: if the first parameter is non-zero,
        // it sets the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing.
        5 => {
            let operand1 = get_operand(opcode_vec, curr_pos + 1, param_mode[0]);
            let operand2 = get_operand(opcode_vec, curr_pos + 2, param_mode[1]);
            let new_pc: usize =
                match operand1 != 0 {
                    true => operand2 as usize,
                    false => curr_pos + 3
                };
            ComputerState {
                pc: new_pc,
                reg: state.reg,
                end: false,
            }
        }
        // Opcode 6 is jump-if-false: if the first parameter is zero,
        // it sets the instruction pointer to the value from the second
        // parameter. Otherwise, it does nothing.
        6 => {
            let operand1 = get_operand(opcode_vec, curr_pos + 1, param_mode[0]);
            let operand2 = get_operand(opcode_vec, curr_pos + 2, param_mode[1]);
            let new_pc: usize =
                match operand1 == 0 {
                    true => operand2 as usize,
                    false => curr_pos + 3
                };
            ComputerState {
                pc: new_pc,
                reg: state.reg,
                end: false,
            }
        }
        // Opcode 7 is less than: if the first parameter is less than the
        // second parameter, it stores 1 in the position given by the third
        // parameter. Otherwise, it stores 0.
        7 => {
            let operand1 = get_operand(opcode_vec, curr_pos + 1, param_mode[0]);
            let operand2 = get_operand(opcode_vec, curr_pos + 2, param_mode[1]);

            let res_pos = opcode_vec[curr_pos + 3];
            opcode_vec[res_pos as usize] =
                match operand1 < operand2 {
                    true =>  1,
                    false => 0,
                };
            ComputerState {
                pc: curr_pos + 4,
                reg: state.reg,
                end: false,
            }
        }
        // Opcode 8 is equals: if the first parameter is equal to the second
        // parameter, it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.
        8 => {
            let operand1 = get_operand(opcode_vec, curr_pos + 1, param_mode[0]);
            let operand2 = get_operand(opcode_vec, curr_pos + 2, param_mode[1]);

            let res_pos = opcode_vec[curr_pos + 3];
            opcode_vec[res_pos as usize] =
                match operand1 == operand2 {
                    true =>  1,
                    false => 0,
                };
            ComputerState {
                pc: curr_pos + 4,
                reg: state.reg,
                end: false,
            }
        }
        99 => {
            ComputerState {
                pc: curr_pos,
                reg: state.reg, //keep old register state to output it
                end: true,
            }
        }
        a => {
            println!("Unreachable! Opcode was: {}, pos: {}, opcodevec: {:?}",
                     a, curr_pos, opcode_vec);
            unreachable!();
        }
    }
}

fn get_operand(opcode_vec: &mut Vec<isize>, param_pos: usize, param_mode: ParamMode) -> isize
{
    match param_mode {
        ParamMode::PositionMode => {
            opcode_vec[opcode_vec[param_pos] as usize]
        }
        ParamMode::IntermediateMode => {
            opcode_vec[param_pos]
        }
    }
}