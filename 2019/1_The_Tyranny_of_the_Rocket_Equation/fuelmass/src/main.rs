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
            println!("{:?} Usage: fuelmass <source>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_mass_vec_from_file(filename) {
        Ok(mass_vec) => {
            let res = calculate_total_fuel(mass_vec);
            println!("Successfully found first repeating freq. Result is: {}", res);
        }
        Err(text) => println!("Error occured: {}", text),
    }
}

fn get_args() -> Result<(String), ArgsError>{
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

fn collect_mass_vec_from_file(inputname: String) -> Result<Vec<i32>, Box<error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

// === New Part, also works ===
    let buf = BufReader::new(fi);
//let lines: Result<i32, _> =
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
//Map a possible ParseIntError onto Box Error
        .map_err(|e| e.into())
}

fn calculate_total_fuel(mass_vec: Vec<i32>) -> i32 {
    mass_vec.into_iter().map(|mass| calculate_module_fuel(mass)).sum()
}

fn calculate_module_fuel(mass: i32) -> i32 {
    let tmp: i32 = mass / 3 - 2;
    //let tmp2 = tmp1.round() as i32;
    if (tmp) > 0
    {
        tmp + calculate_module_fuel(tmp)
    }
    else {
        0
    }
}