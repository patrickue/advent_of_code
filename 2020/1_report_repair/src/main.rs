use std::env;
use std::error;
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
            println!("{:?} Usage: accounting <expense_report>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_usize_vec_from_file(filename) {
        Ok(expense_vec) => {
            let res = combine_all_for_2020(&expense_vec);
            println!("Successful: {:?}", res)
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

fn collect_usize_vec_from_file(inputname: String) -> Result<Vec<usize>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

    let buf = BufReader::new(fi);
//let lines: Result<i32, _> =

    //let lines_iter = buf.lines().map(|l| l.unwrap());
    // We only expect one line:
    //Ok(lines_iter.collect::<Vec<String>>())
    buf.lines()
        .map(|p| p.unwrap().parse::<usize>())
        .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
        //Map a possible ParseIntError onto Box Error
        .map_err(|e| e.into())
}

fn combine_all_for_2020(expenses: &Vec<usize>) -> Option<usize> {
    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            for k in j..expenses.len() {
                if expenses[i] + expenses[j] + expenses[k]== 2020
                {
                    return Some(expenses[i]*expenses[j]*expenses[k])
                }
            }
        }
    }
    None
}