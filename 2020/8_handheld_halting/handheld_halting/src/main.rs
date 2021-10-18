/*use std::thread;
use std::time::Duration;

fn main() {
    let a: i8 = 3;

    // create a thread
    let thread_handle = thread::spawn(move || {
        for i in 0..15 {

            println!("Loop 1 iteration: {}", a+i);
            // wait a bit before next iteration
            // for demonstration purposes
            thread::sleep(Duration::from_millis(500));
        }
    });
    for i in 0..5 {

        println!("Loop 2 iteration: {}", i);
        thread::sleep(Duration::from_millis(500));
    }
    thread_handle.join().unwrap();
}*/
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
enum Operation {
    Jmp,
    Acc,
    Nop,
}

#[derive(Debug)]
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
                .map(|l| parse_instruction(l))
                .collect::<Vec<Instruction>>();
            println!("Parsed instructions {:?}", instructions);
            let loop_acc = detect_loop(instructions);
            println!("Accumulator was {:?} at Loop.", loop_acc);
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

fn parse_instruction<'a>(line: String) -> Instruction {
    let parts: Vec<&str> = line.split(" ").collect();
    let op: Operation = parse_operation(parts[0]);
    let arg: isize = parts[1].parse::<isize>().unwrap();
    return Instruction { op, arg };
}

fn parse_operation(op_str: &str) -> Operation {
    return match op_str {
        "jmp" => Operation::Jmp,
        "nop" => Operation::Nop,
        "acc" => Operation::Acc,
        _ => panic!()
    };
}

fn detect_loop(instructions: Vec<Instruction>) -> isize {
    let mut acc_value: isize = 0;
    let mut pc: usize = 0;
    let mut visited_vec: Vec<bool> = vec![false; instructions.len()];
    loop {
        if visited_vec[pc] {
            return acc_value;
        }

        visited_vec[pc] = true;

        match instructions[pc] {
            Instruction { op: Operation::Jmp, arg } => {
                let tmp = isize::try_from(pc).unwrap() + arg;
                if tmp < 0 { panic!(); } else {
                    pc = usize::try_from(tmp).unwrap();
                }
            }
            Instruction { op: Operation::Nop, arg } => { pc += 1; }
            Instruction { op: Operation::Acc, arg } => {
                acc_value += arg;
                pc += 1;
            }
        }
    }
}